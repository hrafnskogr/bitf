#![doc=include_str!("../README.md")]

/*
 * Main source file for the 'bitf' procedural macro definition
 * usage: #[bitf(size_of_bitfield, ordering_of_field)]
 *        size can be: u8 / u16 / u32 / u64 / u128
 *        ordering can be: Lsb or Msb
 *        when setting the attribute to msb, the first declared field
 *        will be set on the most significant bit, and the other way around
 *        when using the lsb mode
 */


extern crate proc_macro;

mod bitfield;
mod macroparams;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::Type;

use bitfield::Strukt;
use macroparams::{MacroParams, Endianness};


#[proc_macro_attribute]
pub fn bitf(_meta: TokenStream, _input: TokenStream) -> TokenStream
{
    // Get the parameters passed in the attribute
    let params = syn::parse_macro_input!(_meta as MacroParams);
    // Extract type to be returned by the redefined structure, for use in quote! code generation
    let ret_type = &params.ty;
    // Extract the size of the bitfield, for use in quote! code generation
    let bfield_size = params.bitfield_size;

    // Parse the structure attached to the attribute
    let strukt = syn::parse_macro_input!(_input as Strukt);
    // Extract name for quote! code generation
    let name = strukt.name.clone();
    // Extract fields for quote! code generation
    let mut bfields = strukt.bfields.clone();

    if !strukt.is_large_enough(bfield_size)
    {
        panic!("Selected size for bitfield is not large enough to hold every field");
    }

    // if Endianness enum is set on Big Endien (BE)
    // reverse position of the fields
    if params.endianness == Endianness::Msb
    {
        let mut new_pos = bfield_size;
        let mut i = 0;

        for f in strukt.bfields.iter()
        {
            new_pos -= f.bsize;
            bfields[i].pos = new_pos;
            i += 1;
        }
    }

    // Generate "impl" code for each bitfield
    let fields: Vec<_> = bfields.iter()
                          .map(|field| {
                                // Quote! variables formating for correct interpolation
                                let fname = format_ident!("{}", field.name);
                                let set_n = format_ident!("set_{}", field.name);
                                let fsize = field.bsize;
                                let fpos = field.pos;
                                /*
                                let ty = match &field.ty
                                {
                                    Type::Path(x) => &field.ty,
                                    Type::Tuple(x) => 
                                    {
                                        &field.ty/*
                                        if x.elems.len() == 0
                                        {
                                            &params.ty
                                        }
                                        else
                                        {
                                            &field.ty
                                        }*/
                                            println!("{:?}",  
                                    },
                                    _ => panic!("Unrecognized return type"),
                                }; */

                                let ty = &field.ty;

                                // Quote! code generation
                                quote!
                                {
                                    #[inline]
                                    #[allow(non_snake_case)]
                                    fn #fname(self: &Self) -> #ty
                                    {
                                        let mask = #ret_type::MAX >> (#bfield_size - #fsize) << #fpos;
                                        ((self.raw & mask) >> #fpos) as #ty
                                    }

                                    #[inline]
                                    #[allow(non_snake_case)]
                                    pub fn #set_n(self: &mut Self, val: #ty)
                                    {
                                        let mask = 0xff >> (#bfield_size - #fsize) << #fpos;
                                        let tmp = mask ^ self.raw;
                                        self.raw = tmp | ((val as #ret_type) << #fpos);
                                    }
                                }
                            })
                          .collect();

    // Generate full code
    // Struct redefinition
    // Implementation of Default
    // Implementation of each bitfield method
    TokenStream::from(
        quote! {
                #[derive(Debug)]
                struct #name
                {
                    pub raw: #ret_type,
                }

                impl Default for #name
                {
                    fn default() -> Self
                    {
                        #name
                        {
                            raw: 0x0
                        }
                    }

                }

                impl #name
                {
                    #(#fields)*
                }
            }
        )
}

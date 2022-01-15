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
    let raw_type = &params.ty;
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
                                
                                // Hell Match
                                // This match computes which return line should be added
                                //      either primitive type coercion with the "as" keyword
                                //      or based on the From trait
                                //  The latter should be implemented by the user, note that
                                //  the macro uses the .into() variation. So it is up to the
                                //  user to either implement the From trait, from which the Into
                                //  trait will be deducted, or directly the Into trait
                                let mut ty = raw_type.clone();
                                let return_line = match &field.ty
                                {
                                    // If we have something that ressembles a Type
                                    Type::Path(x) =>
                                    {
                                        match x.path.segments[0].ident.to_string().as_ref()
                                        {
                                            // Primitive type coercion
                                            "u8" | "u16" | "u32" | "u64" | "u128" | 
                                            "i8" | "i16" | "i32" | "i64" | "i128" => 
                                            {
                                                //let t = &field.ty;
                                                //quote!{#t}
                                                ty = format_ident!("{}", x.path.segments[0].ident);
                                                quote!{((self.raw & mask) >> #fpos) as #ty}
                                            },
                                            // Anything else will need to implement the From trait
                                            _ => 
                                            {
                                                ty = format_ident!("{}", x.path.segments[0].ident);
                                                quote!{
                                                    let res = ((self.raw & mask) >> #fpos);
                                                    res.into()
                                                    }
                                            }
                                        }
                                    },
                                    // If we have a Tuple, we consider only the empty one ()
                                    Type::Tuple(x) => 
                                    {
                                        if x.elems.len() == 0
                                        {
                                            quote!{ ((self.raw & mask) >> #fpos) as #ty }
                                        }
                                        else
                                        {
                                            panic!("Return type not supported (tuple of multiple elements)");
                                        }
                                    },
                                    // Could not recognize what has been supplied
                                    _ => panic!("Unrecognized return type."),
                                };

                                // Quote! code generation
                                // This section generates the impl code for each field on the
                                // struct (get / set)
                                quote!
                                {
                                    #[inline]
                                    #[allow(non_snake_case)]
                                    fn #fname(self: &Self) -> #ty
                                    {
                                        let mask = #raw_type::MAX >> (#bfield_size - #fsize) << #fpos;
                                        //((self.raw & mask) >> #fpos) as #ty
                                        #return_line
                                    }

                                    #[inline]
                                    #[allow(non_snake_case)]
                                    pub fn #set_n(self: &mut Self, val: #raw_type)
                                    {
                                        let mask = #raw_type::MAX >> (#bfield_size - #fsize) << #fpos;
                                        let tmp = mask ^ self.raw;
                                        self.raw = tmp | (val << #fpos);
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
                    pub raw: #raw_type,
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

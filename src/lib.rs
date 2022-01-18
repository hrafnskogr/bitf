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
use syn::{Type, Ident};
use syn::__private::TokenStream2;

use bitfield::{Strukt, BitField};
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
    // Extract attributes for quote! final code generation
    let attrs = strukt.attrs.clone();
    // Extract the visibility modifier of the struct
    let vis = strukt.vis.clone();
    //
    let map = strukt.map.clone();

    // Bitfield size failsafe
    if !strukt.is_large_enough(bfield_size)
    {
        panic!("Selected size for bitfield is not large enough to hold every field");
    }

    // if Endianness enum is set on Most Significant Bit (MSB)
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

    // Generate code for each declared field in the bitfield 
    let fields = generate_impl_code(&bfields, &raw_type, bfield_size);

    // Generat pretty print code
    let mut pprint = quote!{};
    if params.pprint
    {
        pprint = generate_pretty_print(&name, map, bfield_size);
    }

    // Generate the impl code for the pretty print
    // do_stuff();

    // Generate full code
    // Struct redefinition
    // Implementation of Default
    // Implementation of each bitfield method
    TokenStream::from(
        quote! {
                #(#attrs)* 
                #vis struct #name
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

                #pprint
            }
        )
}

fn generate_impl_code(bitfields: &Vec<BitField>, raw_type: &Ident, bfield_size: usize) -> Vec<TokenStream2>
{
    bitfields.iter()
           .map(|field| 
               {
                    // Quote! variables formating for correct interpolation
                    let fname = format_ident!("{}", field.name);
                    let set_n = format_ident!("set_{}", field.name);
                    let fsize = field.bsize;
                    let fpos = field.pos;
                    let vis = &field.vis;
                                

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
                        #vis fn #fname(self: &Self) -> #ty
                        {
                            let mask = #raw_type::MAX >> (#bfield_size - #fsize) << #fpos;
                            #return_line
                        }

                        #[inline]
                        #[allow(non_snake_case)]
                        #vis fn #set_n(self: &mut Self, val: #raw_type)
                        {
                            let mask = #raw_type::MAX >> (#bfield_size - #fsize) << #fpos;
                            let tmp = !mask & self.raw;
                            self.raw = tmp | (val << #fpos);
                        }
                    }
               })
           .collect()
}

fn generate_pretty_print(struct_name: &Ident, map: Vec<(usize, usize)>, field_size: usize) -> TokenStream2
{
    //let format_string = format_ident!("{{:0{}b}}", field_size);
    let format_string = format!("{{:0{}b}}", field_size);

    let quoted_map: Vec<TokenStream2> = map.iter()
                                           .map(|t|
                                                {
                                                    let size = t.0;
                                                    let access = t.1;

                                                    quote!
                                                    {
                                                        (#size,#access)
                                                    }
                                                })
                                            .collect();

    quote!
    {
        impl #struct_name
        {
            pub fn pprint(self: &Self)
            {
                let map = vec![#(#quoted_map),*];
                //let raw_bin = format!(#format_string, &raw);
                let raw_bin = format!(#format_string, self.raw);

                self.print_scale(&map, raw_bin.len());
                self.print_line(&map, &raw_bin, ("┌", "┬", "┐"), false);
                self.print_line(&map, &raw_bin, ("│", " │", " │"), true);
                self.print_line(&map, &raw_bin, ("└", "┴", "┘"), false);
            }

            fn print_scale(self: &Self, map: &Vec<(usize, usize)>, size: usize)
            {
                let mut start = 0;
                for (val, _) in map
                {
                    let w = 3 + val;
                    let scale = format!("{}", size - start);

                    print!("{}", scale);
                    print!("{}", " ".repeat(w - scale.len()));

                    start += val;
                }

                println!("0");
            }

            fn print_line(self: &Self, map: &Vec<(usize, usize)>, raw_bin: &String, syms: (&str, &str, &str), core: bool)
            {
                print!("{}", syms.0);

                let mut iter = map.iter().peekable();

                let mut start = 0;
                while let Some((val, access)) = iter.next()
                { 
                    let bin_val = String::from(&raw_bin[start..start + val]);

                    if core
                    {
                        let mut dsp = bin_val; 
                        if *access == 0
                        {
                            dsp = "r".repeat(*val);
                        }
                        print!(" {}", dsp); 
                    }
                    else
                    {
                        for _ in 0..(val+2)
                        {
                            print!("─");
                        }
                    }
                    
                    if iter.peek().is_none()
                    {
                        print!("{}", syms.2);
                    }
                    else
                    {
                        print!("{}", syms.1);
                    }
                    start += val;
                }
                
                println!("");
            }
        }
    }
}

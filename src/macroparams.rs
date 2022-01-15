/*
 *
 * This source file holds the data structures and logic
 * for parsing the parameters of the attribute
 *
 */

use syn::parse::{Parse, ParseBuffer};
use syn::{Ident, Token};
use syn::punctuated::Punctuated;
use proc_macro2::Span;


#[derive(Debug)]
pub struct MacroParams
{
    pub bitfield_size:  usize,
    pub endianness:     Endianness,
    pub ty:             Ident,
    pub no_pub:         bool,
}

impl Default for MacroParams
{
    fn default() -> Self
    {
        MacroParams
        {
            bitfield_size:  0,
            endianness:     Endianness::Lsb,
            ty:             Ident::new("pub", Span::call_site()),
            no_pub:         false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Endianness
{
    Lsb,
    Msb,
}

impl Parse for MacroParams
{
    fn parse(input: &ParseBuffer) -> syn::Result<Self>
    {
        let params = Punctuated::<Ident, Token![,]>::parse_terminated(input).unwrap();
       
        let mut ret_struct = MacroParams::default();
        
        for p in &params
        {
            let val = p.to_string();
            match val.as_ref()
            {
                "u8" | "u16" | "u32" | "u64" | "u128"   => 
                {
                    let size = &val[1..];
                    ret_struct.bitfield_size = size.parse::<usize>().unwrap(); 
                    ret_struct.ty = p.clone();
                },
                "lsb"   => ret_struct.endianness = Endianness::Lsb,
                "msb"   => ret_struct.endianness = Endianness::Msb,
                "no_pub"=> ret_struct.no_pub = true,
                _ => { return Err(syn::Error::new(p.span(), "Wrong parameter supplied. Parameters can be: 'u8' / 'u16' / 'u32' / 'u64' / 'u128' for size of bitfield.\n 'lsb' / 'msb' for the order of field declaration.\n 'no_pub' to specify by hand which field should be declared as public.")) }
            }
        }

        if ret_struct.bitfield_size == 0
        {
            panic!("Error: no size specified. Please specify a size for the bitfield, with one of the following parameter: 'u8' / 'u16' / 'u32' / 'u64' / 'u128'");
        }

        Ok( ret_struct )
    }
}


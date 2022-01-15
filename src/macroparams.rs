/*
 *
 * This source file holds the data structures and logic
 * for parsing the parameters of the attribute
 *
 */

use syn::parse::{Parse, ParseBuffer};
use syn::{Ident, Token};
use syn::punctuated::Punctuated;


#[derive(Debug)]
pub struct MacroParams
{
    pub bitfield_size:  usize,
    pub endianness:     Endianness,
    pub ty:             Ident,
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
        
        let bsize:   usize;      
        let mut endness: Endianness = Endianness::Lsb;

        match params[0].to_string().as_ref()
        {
            "u8"    => bsize = 8,
            "u16"   => bsize = 16,
            "u32"   => bsize = 32,
            "u64"   => bsize = 64,
            "u128"  => bsize = 128,
            _ => { return Err(syn::Error::new(params[0].span(), "Wrong size, use one of the following: u8 / u16 / u32 / u64 / u128")); }

        }

        if params.len() > 1
        {
            match params[1].to_string().as_ref()
            {
                "lsb" => endness = Endianness::Lsb, 
                "msb" => endness = Endianness::Msb,
                _    => { return Err(syn::Error::new(params[1].span(), "Wrong endianness, use on of the following: lsb / msb")); }
            }
        }

        Ok(
            MacroParams 
            {
                bitfield_size:  bsize,
                endianness:     endness,
                ty:             params[0].clone(),
            }
        )
    }
}


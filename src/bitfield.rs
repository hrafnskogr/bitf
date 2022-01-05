/*
 *
 * This source file holds the data structures and logic
 * for parsing structures where the attribute is applied
 *
 */

use std::convert::TryFrom;
use quote::ToTokens;
use syn::{ItemStruct, Field, Ident};
use syn::parse::{Parse, ParseBuffer};


static ERR_FORMAT: &str = "Expected format: any_field_name_intSize";

#[derive(Debug)]
pub struct Strukt
{
    pub name:    Ident,
    pub bfields: Vec<BitField>,
}

impl Strukt
{
    pub fn is_large_enough(self: &Self, bitfield_size: usize) -> bool
    {
        let total_size: usize = self.bfields.iter()
                                            .map(|x| x.bsize)
                                            .sum();

        total_size <= bitfield_size
    }
}

impl Parse for Strukt
{
    fn parse(input: &ParseBuffer) -> syn::Result<Self>
    {
        let strukt = input.parse::<ItemStruct>()?;
        let mut fields = Vec::new();

        let mut pos: usize = 0;

        for field in strukt.fields
        {
            let mut bfield = BitField::try_from(&field)?;

            bfield.update_pos(pos);
            pos += bfield.bsize;

            fields.push(bfield)
        }

        Ok( Self
            {
                name: strukt.ident,
                bfields: fields,
            })
    }
}

#[derive(Debug, Clone)]
pub struct BitField
{
    pub name:   String,
    pub bsize:  usize,
    pub pos:    usize,
    //pub ty:     Type,
}

impl BitField
{
    fn update_pos(self: &mut Self, position: usize)
    {
        self.pos = position;
    }
}

impl TryFrom<&Field> for BitField
{
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error>
    {
        let ident = field.ident
                        .as_ref()
                        .ok_or_else(|| {
                            syn::Error::new_spanned(field.to_token_stream(), "Expected a structure with named fields. Unnamed field given") } )?;

        let ident_str = ident.to_string();
        //let split: Vec<&str> = ident_str.split("_").collect();
        let split = rsplit(&ident_str)?;

        let name: String;
        let bsize: usize;

        if split.len() == 2
        {
            name = String::from(split[0]);
            bsize = split[1].parse::<usize>()
                                      .or_else(|x| 
                                               { 
                                                 Err( syn::Error::new_spanned(field.to_token_stream(),
                                                                              format!("{}: {}. {}", x, ident_str, ERR_FORMAT)) )
                                               })?;
        }
        else
        {
            return Err( syn::Error::new_spanned(field.to_token_stream(), format!("Wrong field name format. {}.", ERR_FORMAT)) );
        }

        Ok(BitField { name, bsize, pos: 0 })
    }
}

fn rsplit(field: &str) -> Result<Vec<&str>, syn::Error>
{
    let idx: usize;

    match field.rfind("_")
    {
        Some(x) => idx = x,
        None => return Err( syn::Error::new_spanned(field.to_token_stream(), format!("Could not find size in field name {}. {}.", field, ERR_FORMAT)) ),
    }

    Ok( vec![ &field[0..idx], &field[idx+1..field.len()] ] )
}

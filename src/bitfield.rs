/*
 *
 * This source file holds the data structures and logic
 * for parsing structures where the attribute is applied
 *
 */

use std::convert::TryFrom;
use quote::ToTokens;
use syn::{ItemStruct, Field, Ident, Type};
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

            if bfield.skip
            {
                continue;
            }
           
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
    pub skip:   bool,
    pub ty:     Type,
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

        // Extract name and size from field declaration
        // First a split made only on the right part of the field name
        let ident_str = ident.to_string();
        let split = rsplit(&ident_str)?;

        let name: String;
        let bsize: usize;

        // If the field has been effectively split in 2 parts
        // Then we can take the first part as the name
        // And try to convert the second part as a number
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
            // If we don't have 2 parts it probably means that the format is wrong
            return Err( syn::Error::new_spanned(field.to_token_stream(), format!("Wrong field name format. {}.", ERR_FORMAT)) );
        }

        // If the name is "_reserved, we set the skip value of the BitField struct as false
        // This field will not be implemented
        let skip: bool = &name == "_reserved";

        // Conversion of the return type
        /*let ty: Ident = match field.ty
        {
            syn::Tuple = field.ty as Ident,
            syn::Path => field.ty.segments
            _ => return Err(syn::Error::new_spanned(field.to_token_stream(), format!("Unrecognized field type")),
        }*/
        /*
        match &field.ty
        {
            Type::Path(x) => println!("Segments : {:?}", x),
            Type::Tuple(x) => println!("{:?}", x),
            _ => println!("{:?}", field.ty),
        }
        */
        Ok(BitField { name, bsize, pos: 0, skip, ty: field.ty.clone() })
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

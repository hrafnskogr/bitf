/*
 *
 * Unit test file
 *
 */

use bitf::*;

#[allow(dead_code)]
#[derive(Debug)]
struct CustomStr(u128);

impl From<u8> for CustomStr
{
    fn from(val: u8) -> CustomStr
    {
        CustomStr(val as u128)
    }
}

impl From<u16> for CustomStr
{
    fn from(val: u16) -> CustomStr 
    {
        CustomStr(val as u128)
    }
}

impl From<CustomStr> for u8
{
    fn from(val: CustomStr) -> u8
    {
        val.0 as u8
    }
}

#[repr(C)]
#[bitf(u8, lsb)]
struct MyBitf
{
    field_a_1:    u8,
    fieldB_1:     CustomStr,
    fieldC_2:     u16,
    fieldD_2:     (),
    _reserved_1:  (),
    fieldE_1:     u8,
}

#[test]
fn read_set()
{
    let mut bitf = MyBitf::default();
    let mut cst = CustomStr(400);

    bitf.set_field_a(1);
    bitf.set_fieldD(3);

    bitf.set_fieldE(1);

    // Should have 0000 0001
    assert_eq!(bitf.field_a(), 1);
    
    // Should have 0000 0011
    assert_eq!(bitf.fieldD(), 3);

    // Should have 1011 001
    assert_eq!(bitf.raw, 177);

    bitf.set_fieldE(cst.into());

    bitf.set_fieldB(1);
    let mut cst = bitf.fieldB();
    assert_eq!(cst.0, 1);
}

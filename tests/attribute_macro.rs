/*
 *
 * Unit test file
 *
 */

use bitf::*;

#[repr(C)]
#[allow(dead_code)]
#[bitf(u8, lsb)]
#[derive(Debug)]
struct MyBitf
{
    field_a_1:    u8,
    fieldB_1:     (),
    fieldC_2:     u16,
    fieldD_2:     (),
    _reserved_1:  (),
    fieldE_1:     u8,
}

#[test]
fn read_set()
{
    let mut bitf = MyBitf::default();

    bitf.set_field_a(1);
    bitf.set_fieldD(3);
    bitf.set_fieldE(1);

    // Should have 0000 0001
    assert_eq!(bitf.field_a(), 1);
    
    // Should have 0000 0011
    assert_eq!(bitf.fieldD(), 3);

    // Should have 1011 001
    assert_eq!(bitf.raw, 177);

}

#[repr(C)]
#[bitf(u8)]
struct MyOtherBitf
{
    field1_3:   (),
    field2_2:   (),
    field3_3:   (),
}

#[test]
fn simple_bitf()
{
    let mut obitf = MyOtherBitf::default();
    
    obitf.set_field1(2);
    obitf.set_field2(1);
    obitf.set_field3(4);

    // Should have 0000 0010
    assert_eq!(obitf.field1(), 2);
    // Should have 0000 0001
    assert_eq!(obitf.field2(), 1);
    // Should have 0000 0100
    assert_eq!(obitf.field3(), 4);
    // Should have 1000 1010
    // Or 
    // 100 01 010
    assert_eq!(obitf.raw, 138);
}

#[bitf(u8)]
#[repr(C)]
struct CstBitf
{
    fieldA_4:   (),
    fieldB_4:   CustomStr,
}

#[test]
fn custom_bitf()
{
    let mut cbitf = CstBitf::default();
    let cst = CustomStr(10);
    
    cbitf.set_fieldA(14);
    cbitf.set_fieldB(cst.into());

    let other_cst = cbitf.fieldB();

    assert_eq!(cbitf.fieldA(), 14);
    assert_eq!(other_cst.0, 10);

}

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

impl From<CustomStr> for u8
{
    fn from(val: CustomStr) -> u8
    {
        val.0 as u8
    }
}


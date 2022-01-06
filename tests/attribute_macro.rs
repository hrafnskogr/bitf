/*
 *
 * Unit test file
 *
 */

use bitf::*;
use syn::Type;
struct CustomStr(u128);


#[repr(C)]
#[bitf(u16, lsb)]
struct MyStruct
{
    field_a_1:    u16,
    fieldB_1:     u8,
    fieldC_2:     u8,
    fieldD_2:     u16,
    _reserved_1:  u8,
    _res_1:       u8,
}

#[test]
fn read_set()
{
    let mut m = MyStruct::default();
    
    m.set_field_a(1);
    m.set_fieldD(3);

    m.set__res(1);

    // Should have 0000 0001
    assert_eq!(m.field_a(), 1);
    
    // Should have 0000 0011
    assert_eq!(m.fieldD(), 3);

    // Should have 1011 001
    assert_eq!(m.raw, 177);
}

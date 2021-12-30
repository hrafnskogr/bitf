/*
 *
 * Unit test file
 *
 */

use bitf::*;


#[repr(C)]
#[bitf(u8, lsb)]
struct MyStruct
{
    fieldA_1:    (),
    fieldB_1:    (),
    fieldC_2:    (),
    fieldD_2:    (),
    reserved_2:  (),
}

#[test]
fn read_set()
{
    let mut m = MyStruct::default();
   
    m.set_fieldA(1);
    m.set_fieldD(3);

    println!("{:#010b}", m.raw);

    assert_eq!(m.fieldA(), 1);
    assert_eq!(m.fieldD(), 3);
    assert_eq!(m.raw, 0x31);
}

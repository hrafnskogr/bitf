# bitf
Rust procedural macro to quickly generate bitfield from a structure.

## Usage and syntax
The macro can be used as following:
```text
#[bitf(size, order)]

Where size can be:
    u8
    u16
    u32
    u64
    u128

And order can be 'lsb' or 'msb'
```

The size parameter will constrain the total size of the bitfield.
The order parameter will alter the order in which the fields are declared.
When setting the order parameter to msb, the first declared field of the struct will be set on the most significant bit, and the other way around when using the lsb mode.

Hence, the size and position of the field is based on the field declaration :
```rust
use bitf::bitf;

#[bitf(u8,lsb)]
struct Example
{
    any_case_name_2: (),        // () is used to specify to use the raw type defined in the attribute (here is u8)
    _reserved_4:     (),        // This field will not be implemented as the name is _reserved
    name_B_2:        u16,	// Return type override. The `get` method implemented will return a u16
    				// Custom types can be used, be will need to implement the From trait
				// Please see the test file in "test/attribute_macro.rs" for an example
}

// The internal, full value of the field can be accessed as :

let e = Example::default();
println!("{}", e.raw);

```
## Skipping the implementation of a field
You can use the following syntax when declaring a field to skip its implementation.
`_reserved_intSize`

In the previous example, the field `_reserved_4` will not have its 4 bits implemented.
No accessor will be generated for this field.


## Example

Considering the following bitfield:

```text
7             0
0 0 0 0 0 0 0 0
| | | | | | | |_ field_a    - Size 1
| | | | | | |___ fieldB     - Size 1
| | | | | |_____ fieldC     - Size 1
| |  \|/________ reserved   - Size 3
\ /_____________ field_D    - Size 2

```     
It can be achieved with the following declaration and macro usage

```rust
use bitf::bitf;

#[bitf(u8, lsb)]
struct MyStruct
{
    field_a_1:  (),
    fieldB_1:   (),
    FieldC_1:   (),
    reserved_3: (),
    Field_D_2:  (),
}
```

This will generate the following structure and associated methods

```rust
struct MyStruct
{
  pub raw:  u8,
}

impl MyStruct
{
    pub fn field_a(self: &Self) -> u8 { /* bitwise logic */ 0 }
    pub fn set_field_a(self: &Self, val: u8) { /* bitwise logic */ }
    pub fn fieldB(self: &Self) -> u8 { /* bitwise logic */ 0 }
    pub fn set_fieldB(self: &Self, val: u8) { /* bitwise logic */ }
    /*
     * And so on...
     */
    
}

impl Default for MyStruct 
{ 
    fn default() -> Self
    {
        MyStruct { raw: 0x0 }
    } 
}

//So you can easily set and read values of each defined bitfield:

let mut bf = MyStruct::default();

bf.set_field_a(1);
bf.set_fieldB(1);
println!("{:#010b}", bf.field_a());

```

# TODO
- [x] A short-sighted decision made it that currently the macro is assuming that the format of the declared field is of the form CamelCaseName_Size. Would be better to implement the form Any_Case_Size
- [x] Generate proper rust documentation
- [ ] Implement a pretty print for easy bitfield reading
- [X] Skip the implementation of the fields defined as reserved (or not?). Done: you can mark a field as reserved using the naming convention `_reserved_intSize
- [x] Implement a check to fail if the bitfield is too small to hold every declared field
- [ ] Add lsb/msb as optional param, make lsb default
- [ ] Add visibility modifier param. Either all declared field are implemented as pub (default) or specified by user
- [x] Add custom return type for each declared field

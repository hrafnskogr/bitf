# bitf
Rust procedural macro to quickly generate bitfield from a structure.

Features:
- Any size from 8 to 128 bits
- Auto implementation of _getters_ and _setters_, and Default.
- Supports the use of other attribute on the structure
- Declaration of fields either from the Least Significant Bit or the Most Significant Bit
- Supports custom return types (primitives and custom types)
- Supports custom visibility for each field
- Skip implementation of fields marked as reserved
- Failsafe to prevent declaring more field than the bitfield can contain
- Implementation of a Pretty Print associated function: pprint()


_By default:_
 - _starts declaration of fields from the Least Significant Bit;_
 - _declares all fields as public;_
 - _does not implement the pretty print function;_


## Usage and syntax
The macro can be used as following:
```text
#[bitf(size, opt_arg1, opt_arg2, opt_arg3)]

Where size can be:
    u8
    u16
    u32
    u64
    u128

There are 3 optional parameters:
Order:  can be 'lsb' or 'msb'
Visibility: 'no_pub'
Pretty Print: 'pp'

```
#### Size
The `size` parameter will constrain the total size of the bitfield.

#### Order
The `order` parameter is optional and will alter the order in which the fields are declared.
By default this parameter is set to `lsb`.
When setting the order parameter to `msb`, the first declared field of the struct will be set on the most significant bit, and the other way around when using the lsb mode.

#### Visibility
The `visibility` parameter is optional and will alter the visibility of the declared field. It can be set only to `no_pub`.
By default, all fields are declared as public, using the flag `no_pub` will deactivate this behaviour and rely on the visibility declared by the user.



Hence, the size and position of the field is based on the field declaration :
```rust
use bitf::bitf;

#[bitf(u8, lsb, pp)]
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

// and the representation of the bitfield can be accessed via
e.pprint();

```

When combined to other attributes, make sure to implement it **BEFORE** any `#[derive(..)]` attribute, or the expansion order might (will) fail. 

```rust
use bitf::bitf;

#[repr(C)]
#[allow(dead_code)]
#[bitf(u8)]
#[derive(Debug)]
struct MyStruct
{
    fieldA_4:	(),
    fieldB_4:	(),
}
```

#### Pretty Print
The `Pretty Print` parameter is set throught the `pp` switch.
This switch will implement an associated set of functions on the structure, accessible through `pprint()`.
This function will produce the following output (for a 128 bits bitfield):

```text

64     60  59   57  56                 40      35         27     23   21    18  17           7   6     3   2    0
┌──────┬───┬────┬───┬──────────────────┬───────┬──────────┬──────┬────┬─────┬───┬────────────┬───┬─────┬───┬────┐
│ 1111 │ 1 │ 01 │ 0 │ rrrrrrrrrrrrrrrr │ 01101 │ 11110101 │ 0110 │ 00 │ 010 │ 0 │ 1000110101 │ 0 │ 110 │ 0 │ 10 │
└──────┴───┴────┴───┴──────────────────┴───────┴──────────┴──────┴────┴─────┴───┴────────────┴───┴─────┴───┴────┘

```
The field noted as "rrrrrrrr..." symbolizes a reserved field. Such fields are defined when declared with the name `_reserved_usize`

_Please note that there is not any mechanism of paging or any clever system to adapt the output to the shell size.
Hence, it will probably fail if you try to print a bitfield of 128 1-byte wide fields, unless you have an exceptionnaly wide screen_


## Reserved fields: skipping the implementation of a field
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

#[bitf(u8)]
struct MyStruct
{
    field_a_1:  (),
    fieldB_1:   (),
    FieldC_1:   (),
    _reserved_3: (),
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
- [x] Implement a pretty print for easy bitfield reading
- [X] Skip the implementation of the fields defined as reserved (or not?). Done: you can mark a field as reserved using the naming convention `_reserved_intSize`
- [x] Implement a check to fail if the bitfield is too small to hold every declared field
- [x] Add lsb/msb as optional param, make lsb default
- [x] Add visibility modifier param. Either all declared field are implemented as pub (default) or specified by user
- [x] Add custom return type for each declared field
- [x] Support the addition of attribute to the structure
- [ ] ???

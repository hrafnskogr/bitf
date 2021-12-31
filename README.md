# bitf
Rust procedural macro to quickly generate bitfield from a structure.

## Usage
The macro can be used as following:
```rust
#[bitf(size, order)]

// Where size can be:
u8
u16
u32
u64
u128

// And order can be lsb or msb
```
The size parameter will constrain the total size of the bitfield.
The order parameter will alter the order in which the fields are declared.
When setting the order parameter to msb, the first declared field of the struct will be set on the most significant bit, and the other way around when using the lsb mode.

The size and position of the field is based on the field declaration :
``` rust
struct Example
{
  any_case_name_intBitfieldSize: BogusType,
}
```

Finally, the internal, full value of the field can be accessed as :
``` rust
struct Example
{
  any_case_name_intBitfieldSize: BogusType,
}

let e = Example::default();
println!("{}", e.raw);

```


## Example

Considering the following bitfield:

```
7             0
0 0 0 0 0 0 0 0
| | | | | | | |_ field1    - Size 1
| | | | | | |___ field2    - Size 1
| | | | | |_____ field3    - Size 1
| |  \|/________ reserved  - Size 3
\ /_____________ field4    - Size 2

```     
It can be achieved with the following declaration and macro usage

```rust
#[bitf(u8, lsb]
struct MyStruct
{
  field_a_1:   (),
  fieldA_1:   (),
  FieldA_1:   (),
  reserved_3: (),
  Field_A_2:   (),
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
  fn field_a() -> u8 { ... }
  fn set_field_a(val: u8) { ... }
}

impl Default for MyStruct { ... }
```

So you can easily set and read values of each defined bitfield:

```rust
let mut bf = MyStruct::default;
bf.set_field1(1);
println!("{:#010b}", bf.field1());
```

# TODO
- [x] A short-sighted decision made it that currently the macro is assuming that the format of the declared field is of the form CamelCaseName_Size. Would be better to implement the form Any_Case_Size
- [ ] Generate proper rust documentation
- [ ] Implement a pretty print for easy bitfield reading
- [ ] Skip the implementation of the fields defined as reserved (or not?)
- [ ] Implement a check to fail if the bitfield is too small to hold every declared field



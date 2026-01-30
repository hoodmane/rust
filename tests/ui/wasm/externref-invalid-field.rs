//@ only-wasm32
//@ compile-flags: -C target-feature=+reference-types

// Test that externref cannot be used as a field in struct/enum.
// WebAssembly externref values cannot be stored in linear memory.

#![feature(wasm_reference_types)]

use core::arch::wasm32::externref;

// Invalid: externref in struct field
struct MyStruct {
    value: externref, //~ ERROR `externref` cannot be used as a field
}

// Invalid: externref in tuple struct
struct TupleStruct(externref); //~ ERROR `externref` cannot be used as a field

// Invalid: externref in enum variant
enum MyEnum {
    WithRef { r: externref }, //~ ERROR `externref` cannot be used as a field
    WithTuple(externref), //~ ERROR `externref` cannot be used as a field
}

// Invalid: externref in union (also can't be in linear memory)
union MyUnion {
    value: externref, //~ ERROR `externref` cannot be used as a field
}

// Invalid: externref nested in tuple field
struct StructWithTupleField {
    tuple: (i32, externref), //~ ERROR `externref` cannot be used as a field
}

// Invalid: externref in array field
struct StructWithArrayField {
    arr: [externref; 2], //~ ERROR `externref` cannot be used as a field
}

fn main() {}

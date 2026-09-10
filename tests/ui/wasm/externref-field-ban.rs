// An externref place is a WebAssembly table slot, not linear memory, so
// `externref` (and anything containing one in its own storage) cannot be a
// field of an aggregate. Pointers to externref are ordinary data (slot
// indices) and are fine.

//@ only-wasm32
//@ edition: 2021

#![feature(wasm_externref)]
#![crate_type = "lib"]

use core::ffi::externref;

struct Direct {
    x: externref, //~ ERROR `externref` cannot be part of a field of a struct
}

struct InArray {
    x: [externref; 4], //~ ERROR `externref` cannot be part of a field of a struct
}

struct InTuple {
    x: (u32, externref), //~ ERROR `externref` cannot be part of a field of a struct
}

struct InGenericArg {
    x: Option<externref>, //~ ERROR `externref` cannot be part of a field of a struct
}

enum InEnum {
    A(externref), //~ ERROR `externref` cannot be part of a field of a variant
    B,
}

union InUnion {
    x: core::mem::ManuallyDrop<externref>, //~ ERROR `externref` cannot be part of a field of a union
}

// Pointers to externref are ordinary data: all of these are allowed.
struct Pointers {
    a: *mut externref,
    b: *const externref,
    c: &'static externref,
    d: *mut [externref; 8],
    e: Option<&'static externref>,
    f: unsafe extern "C" fn(externref) -> externref,
}

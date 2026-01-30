//@ only-wasm32
//@ compile-flags: -C target-feature=+reference-types

// Test that pointers and references to externref are rejected.
// WebAssembly externref values cannot be stored in linear memory,
// so pointers/references to them are not valid.

#![feature(wasm_reference_types)]

use core::arch::wasm32::externref;

// Invalid: raw pointer to externref
fn takes_ptr_externref(_: *const externref) {}
//~^ ERROR cannot create a pointer to `externref`

fn takes_mut_ptr_externref(_: *mut externref) {}
//~^ ERROR cannot create a pointer to `externref`

// Invalid: reference to externref
fn takes_ref_externref(_: &externref) {}
//~^ ERROR cannot create a reference to `externref`

fn takes_mut_ref_externref(_: &mut externref) {}
//~^ ERROR cannot create a reference to `externref`

// Invalid: returning pointer to externref
fn returns_ptr_externref() -> *const externref {
//~^ ERROR cannot create a pointer to `externref`
    core::ptr::null()
}

// Invalid: nested in tuple
fn takes_tuple_with_ptr(_: (i32, *const externref)) {}
//~^ ERROR cannot create a pointer to `externref`

fn main() {}

//@ only-wasm32
//@ compile-flags: -C target-feature=+reference-types

// Test that the wasm_reference_types feature gate is required.

use std::arch::wasm32::externref;
//~^ ERROR use of unstable library feature `wasm_reference_types`

fn takes_externref(_: externref) {}
//~^ ERROR use of unstable library feature `wasm_reference_types`

fn returns_externref() -> externref {
//~^ ERROR use of unstable library feature `wasm_reference_types`
    panic!()
}

fn main() {}

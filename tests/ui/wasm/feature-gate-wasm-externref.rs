//@ only-wasm32
//@ edition: 2021
#![crate_type = "lib"]

extern "C" {
    fn f1(x: core::ffi::externref); //~ ERROR use of unstable library feature `wasm_externref`
}

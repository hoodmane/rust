//! Verify that externref uses the correct LLVM representation (pointer in address space 10).

//@ compile-flags: -Copt-level=3 -Ctarget-feature=+reference-types
//@ only-wasm32
//@ needs-llvm-components: webassembly

#![crate_type = "lib"]
#![no_std]
#![feature(wasm_reference_types)]

use core::arch::wasm32::externref;

// Check that externref is represented as a pointer in address space 10
#[no_mangle]
pub extern "C" fn pass_externref(r: externref) -> externref {
    // CHECK-LABEL: @pass_externref(
    // CHECK-SAME: ptr addrspace(10)
    // CHECK-SAME: %r
    // CHECK: ret ptr addrspace(10) %r
    r
}

// TODO: Test Option<externref> once niche optimization is implemented
// Option<externref> should use nullable externref representation

// Check extern functions with externref parameters
extern "C" {
    fn extern_takes_externref(r: externref);
    fn extern_returns_externref() -> externref;
}

#[no_mangle]
pub extern "C" fn call_extern_with_externref(r: externref) {
    // CHECK-LABEL: @call_extern_with_externref(
    // CHECK: call void @extern_takes_externref(ptr addrspace(10)
    unsafe { extern_takes_externref(r) };
}

#[no_mangle]
pub extern "C" fn get_extern_externref() -> externref {
    // CHECK-LABEL: @get_extern_externref(
    // CHECK: call{{.*}}ptr addrspace(10) @extern_returns_externref()
    unsafe { extern_returns_externref() }
}

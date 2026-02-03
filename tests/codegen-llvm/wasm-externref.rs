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

// Check that Option<externref> has the same representation as externref (niche optimization)
// None is represented as ref.null extern
#[no_mangle]
pub extern "C" fn pass_option_externref(r: Option<externref>) -> Option<externref> {
    // CHECK-LABEL: @pass_option_externref(
    // CHECK-SAME: ptr addrspace(10)
    // CHECK-SAME: %r
    // CHECK: ret ptr addrspace(10)
    r
}

#[no_mangle]
#[target_feature(enable = "reference-types")]
pub unsafe extern "C" fn create_none_externref() -> Option<externref> {
    // CHECK-LABEL: @create_none_externref(
    // CHECK: call ptr addrspace(10) @llvm.wasm.ref.null.extern()
    // CHECK: ret ptr addrspace(10)
    None
}

#[no_mangle]
pub extern "C" fn create_some_externref(r: externref) -> Option<externref> {
    // CHECK-LABEL: @create_some_externref(
    // CHECK-SAME: ptr addrspace(10)
    // CHECK: ret ptr addrspace(10)
    Some(r)
}

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

// Check that externref::null() calls the LLVM intrinsic
#[no_mangle]
#[target_feature(enable = "reference-types")]
pub unsafe extern "C" fn create_null_externref() -> externref {
    // CHECK-LABEL: @create_null_externref(
    // CHECK: call ptr addrspace(10) @llvm.wasm.ref.null.extern()
    // CHECK: ret ptr addrspace(10)
    externref::null()
}

// Check that externref::is_null() calls the LLVM intrinsic
#[no_mangle]
#[target_feature(enable = "reference-types")]
pub unsafe extern "C" fn check_externref_is_null(r: externref) -> bool {
    // CHECK-LABEL: @check_externref_is_null(
    // CHECK: call i32 @llvm.wasm.ref.is_null.extern(ptr addrspace(10)
    r.is_null()
}

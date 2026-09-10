// Verify the codegen contract for `core::ffi::externref`: a place of that
// type is one WebAssembly externref table slot, a pointer to it is the slot
// index, and reads/writes of it are loads/stores of the LLVM
// `target("wasm.externref")` reference type (which the WebAssembly backend
// lowers to `table.get`/`table.set`). Raw `__externref_t` values appear only
// in `extern "C"` signatures; the conversions happen at the loads and stores
// around the call, so Rust itself only ever holds table indices.

//@ only-wasm32
//@ edition: 2021
//@ compile-flags: -C target-feature=+reference-types

#![crate_type = "lib"]
#![feature(wasm_externref)]

use core::ffi::externref;

extern "C" {
    // void f1(__externref_t);
    fn f1(x: externref);
    // void f2(__externref_t*);
    fn f2(x: *mut externref);
    // __externref_t f3(void);
    fn f3() -> externref;
    // __externref_t* f4(void);
    fn f4() -> *mut externref;
}

// A raw-externref argument position: the argument operand is loaded from its
// slot (a table.get) and passed by value.
// CHECK-LABEL: @call_f1
// CHECK: %[[V1:.+]] = load target("wasm.externref"), ptr %slot, align 1
// CHECK: call void @f1(target("wasm.externref") {{.*}}%[[V1]])
#[unsafe(no_mangle)]
pub unsafe fn call_f1(slot: *mut externref) {
    f1(*slot);
}

// An `__externref_t*` position: the table pointer is passed through unchanged.
// CHECK-LABEL: @call_f2
// CHECK-NOT: load target("wasm.externref")
// CHECK: call void @f2(ptr {{.*}}%slot)
#[unsafe(no_mangle)]
pub unsafe fn call_f2(slot: *mut externref) {
    f2(slot);
}

// A raw-externref return: the returned value is stored into the destination
// slot (a table.set).
// CHECK-LABEL: @call_f3
// CHECK: %[[V3:.+]] = {{.*}}call {{.*}}target("wasm.externref") @f3()
// CHECK: store target("wasm.externref") %[[V3]], ptr %slot, align 1
#[unsafe(no_mangle)]
pub unsafe fn call_f3(slot: *mut externref) {
    *slot = f3();
}

// An `__externref_t*` return: an ordinary pointer (slot index) comes back.
// CHECK-LABEL: @call_f4
// CHECK: {{.*}}call {{.*}}ptr @f4()
#[unsafe(no_mangle)]
pub unsafe fn call_f4() -> *mut externref {
    f4()
}

// An address-taken externref local is a *typed* alloca of the reference type
// (never a byte array); the RefTypeMem2Local backend pass recognizes it by
// that type and assigns it an externref-table stack slot. A local whose
// address is not taken stays in SSA and becomes a plain wasm local.
// CHECK-LABEL: @local_slot
// CHECK: alloca target("wasm.externref"), align 1
#[unsafe(no_mangle)]
pub unsafe fn local_slot() {
    let mut x: externref = f3();
    f2(&raw mut x);
    f1(x);
}

// An array of externref is a run of table slots: element size is one slot, so
// pointer arithmetic is in slot units (a GEP with stride 1, i.e. no scaling
// of the index).
// CHECK-LABEL: @slot_arithmetic
// CHECK-NOT: mul
// CHECK: getelementptr inbounds{{.*}}, ptr %base, i32 %i
#[unsafe(no_mangle)]
pub unsafe fn slot_arithmetic(base: *mut externref, i: usize) -> *mut externref {
    base.add(i)
}

// CHECK-LABEL: @local_array
// CHECK: alloca [3 x target("wasm.externref")], align 1
#[unsafe(no_mangle)]
pub unsafe fn local_array() {
    let mut a: [externref; 3] = [f3(), f3(), f3()];
    a[1] = f3();
    f1(a[1]);
    f2(&raw mut a[2]);
}

// A Rust-defined extern "C" function with a raw externref parameter gets it
// by value, like the C side would.
// CHECK-LABEL: @callback
// CHECK-SAME: (target("wasm.externref") {{.*}}%x)
#[unsafe(no_mangle)]
pub extern "C" fn callback(x: externref) {
    unsafe { f1(x) }
}

// Copying between externref places is a slot copy: a load and a store of the
// reference type (table.get + table.set), never a byte copy.
// CHECK-LABEL: @copy_slot
// CHECK: %[[C:.+]] = load target("wasm.externref"), ptr %src, align 1
// CHECK: store target("wasm.externref") %[[C]], ptr %dst, align 1
// CHECK-NOT: memcpy
#[unsafe(no_mangle)]
pub unsafe fn copy_slot(dst: *mut externref, src: *mut externref) {
    *dst = *src;
}

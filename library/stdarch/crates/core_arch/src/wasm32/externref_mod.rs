//! WebAssembly reference types.
//!
//! This module provides support for WebAssembly's `externref` type, which
//! represents an opaque reference to a host object.

#[cfg(test)]
use stdarch_test::assert_instr;

/// WebAssembly `externref` type - an opaque reference to a host object.
///
/// This type represents a reference to an object managed by the WebAssembly host
/// environment (e.g., a JavaScript object in a browser). It cannot be forged,
/// inspected, or stored in linear memory - it can only be passed around as an
/// opaque handle.
///
/// # Example
///
/// ```ignore
/// #![feature(wasm_reference_types)]
/// use core::arch::wasm32::externref;
///
/// #[no_mangle]
/// pub extern "C" fn identity(obj: externref) -> externref {
///     obj
/// }
///
/// #[no_mangle]
/// pub extern "C" fn check_null(obj: externref) -> bool {
///     obj.is_null()
/// }
/// ```
///
/// # Note on `Option<externref>`
///
/// Currently, `Option<externref>` uses `ref.null extern` to represent `None`.
/// This means that JavaScript `null` values will be indistinguishable from
/// Rust `None`. A future version may use wasm-gc features to distinguish
/// these cases.
#[lang = "wasm_externref"]
#[derive(Copy, Clone)]
#[unstable(feature = "wasm_reference_types", issue = "128511")]
#[allow(non_camel_case_types)]
pub struct externref {
    // This is a compiler-magic type. The actual representation is
    // handled specially by the compiler as a pointer in address space 10.
    // The field is private and zero-sized; the actual value is entirely
    // managed by the compiler's special layout for this lang item.
    _private: (),
}

// LLVM intrinsics for WebAssembly reference types.
// These must use externref directly since it has a special ABI (ptr addrspace(10)).
#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
    #[link_name = "llvm.wasm.ref.null.extern"]
    fn llvm_ref_null_extern() -> externref;

    #[link_name = "llvm.wasm.ref.is_null.extern"]
    fn llvm_ref_is_null_extern(r: externref) -> i32;
}

impl externref {
    /// Creates a null `externref`.
    ///
    /// In a JavaScript host, this corresponds to the `null` value.
    ///
    /// # Safety
    ///
    /// This function requires the `reference-types` WebAssembly feature to be
    /// enabled at compile time.
    #[inline]
    #[cfg_attr(test, assert_instr("ref.null extern"))]
    #[target_feature(enable = "reference-types")]
    #[unstable(feature = "wasm_reference_types", issue = "128511")]
    pub unsafe fn null() -> Self {
        llvm_ref_null_extern()
    }

    /// Returns `true` if this `externref` is null.
    ///
    /// # Safety
    ///
    /// This function requires the `reference-types` WebAssembly feature to be
    /// enabled at compile time.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # #![feature(wasm_reference_types)]
    /// # use core::arch::wasm32::externref;
    /// unsafe {
    ///     let null_ref = externref::null();
    ///     assert!(null_ref.is_null());
    /// }
    /// ```
    #[inline]
    #[cfg_attr(test, assert_instr("ref.is_null"))]
    #[target_feature(enable = "reference-types")]
    #[unstable(feature = "wasm_reference_types", issue = "128511")]
    pub unsafe fn is_null(self) -> bool {
        llvm_ref_is_null_extern(self) != 0
    }
}

#[unstable(feature = "wasm_reference_types", issue = "128511")]
impl core::fmt::Debug for externref {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Safety: is_null requires reference-types feature, but for Debug
        // we just show a placeholder
        f.write_str("externref")
    }
}

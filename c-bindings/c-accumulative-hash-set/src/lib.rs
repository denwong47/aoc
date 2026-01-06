use accumulative_hash::AccumulativeHashSet;
use std::os::raw::{c_uint, c_ulong, c_ushort};

pub type CItem = c_uint;
pub type CBool = c_ushort;

#[derive(Default)]
pub struct CAccumulativeHashSetU64 {
    pub rust_inner: AccumulativeHashSet<u64>,
}

#[unsafe(no_mangle)]
pub extern "C" fn new_hash_set() -> *mut CAccumulativeHashSetU64 {
    // Heap-allocate the set and return a raw pointer
    Box::into_raw(Box::new(Default::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_state(ptr: *const CAccumulativeHashSetU64) -> c_ulong {
    let set = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    *set.rust_inner.hasher_state()
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_visited_count(ptr: *const CAccumulativeHashSetU64) -> c_ulong {
    let set = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    set.rust_inner.visited_count() as c_ulong
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_contains_path_to(
    ptr: *const CAccumulativeHashSetU64,
    state: c_ulong,
) -> CBool {
    let set = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    set.rust_inner.contains_path_to(state as u64).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_transverse_to(
    ptr: *mut CAccumulativeHashSetU64,
    element: CItem,
) -> CBool {
    let set = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    set.rust_inner.transverse_to(element as u64).into()
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_backtrack(ptr: *mut CAccumulativeHashSetU64) -> CBool {
    let set = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    set.rust_inner.backtrack().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_set_visit_and_backtrack(ptr: *mut CAccumulativeHashSetU64) -> CBool {
    let set = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    set.rust_inner.visit_and_backtrack().into()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_hash_set(ptr: *mut CAccumulativeHashSetU64) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Take ownership back from C and let Box drop it
        drop(Box::from_raw(ptr));
    }
}

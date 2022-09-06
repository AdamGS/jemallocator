#![cfg_attr(feature = "alloc_trait", feature(allocator_api))]

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

#[test]
#[cfg(feature = "alloc_trait")]
fn shrink_in_place() {
    unsafe {
        use core::alloc::{Allocator, Layout};

        // allocate 7 bytes which end up in the 8 byte size-class as long as
        // jemalloc's default size classes are used:
        let orig_sz = 7;
        let orig_l = Layout::from_size_align(orig_sz, 1).unwrap();
        let ptr = Jemalloc.allocate(orig_l).unwrap();
        let ptr = ptr.as_non_null_ptr();

        // try to grow it in place by 1 byte - it should grow without problems:
        let new_sz = orig_sz + 1;
        let new_l = Layout::from_size_align(new_sz, 1).unwrap();
        assert!(Jemalloc.grow(ptr, orig_l, new_l).is_ok());
        let new_l = Layout::from_size_align(orig_sz + 1, 1).unwrap();

        // trying to do it again fails because it would require moving the
        // allocation to a different size class which jemalloc's xallocx does not
        // do:
        let new_sz = new_sz + 1;
        let new_new_l = Layout::from_size_align(new_sz, 1).unwrap();
        assert!(Jemalloc.grow(ptr, new_l, new_new_l).is_err());

        Jemalloc.deallocate(ptr, new_l)
    }
}

#![feature(allocator_api)]
#![feature(box_syntax)]
#![feature(coerce_unsized)]
#![feature(core_intrinsics)]
#![feature(dispatch_from_dyn)]
#![feature(dropck_eyepatch)]
#![feature(exact_size_is_empty)]
#![feature(fn_traits)]
#![feature(generator_trait)]
#![feature(iter_nth_back)]
#![feature(ptr_internals)]
#![feature(raw_vec_internals)]
#![feature(receiver_trait)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(unsized_locals)]

extern crate alloc as rust_alloc;

pub mod alloc;
pub mod boxed;
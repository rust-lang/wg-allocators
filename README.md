# Rust Allocators Working Group

This WG aims to:

* Define traits and other APIs for memory allocators,
  with the unstable [`std::alloc::Alloc`] trait as a starting point.

* Make collections such as `Vec<T>` able to use any allocator,
  most likely through an additional type parameter.
  
Discussion has historically happened in (at least) [RFC #1398], [tracking issue #32838], and [tracking issue #42774].
However individual GitHub theards become hard to navigate as they get longer.
This repository’s issue tracker allows discussing and resolving various topics separately.


## Getting involved

To be notified of every issue and pull request in this repository, try GitHub’s *Watch* button.

For lower-latency discussion, we have the [`t-libs/wg-allocators`] stream on Zulip.


[`std::alloc::Alloc`]: https://doc.rust-lang.org/1.34.0/std/alloc/trait.Alloc.html
[RFC #1398]: https://github.com/rust-lang/rfcs/pull/1398 "Allocators, take III"
[tracking issue #32838]: https://github.com/rust-lang/rust/issues/32838 "Allocator traits and std::heap"
[tracking issue #42774]: https://github.com/rust-lang/rust/issues/42774 "Tracking issue for custom allocators in standard collections"
[`t-libs/wg-allocators`]: https://rust-lang.zulipchat.com/#narrow/stream/197181-t-libs.2Fwg-allocators

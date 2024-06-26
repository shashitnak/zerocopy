// Copyright 2018 The Fuchsia Authors
//
// Licensed under the 2-Clause BSD License <LICENSE-BSD or
// https://opensource.org/license/bsd-2-clause>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// After updating the following doc comment, make sure to run the following
// command to update `README.md` based on its contents:
//
//   cargo -q run --manifest-path tools/Cargo.toml -p generate-readme > README.md

//! *<span style="font-size: 100%; color:grey;">Need more out of zerocopy?
//! Submit a [customer request issue][customer-request-issue]!</span>*
//!
//! ***<span style="font-size: 140%">Fast, safe, <span
//! style="color:red;">compile error</span>. Pick two.</span>***
//!
//! Zerocopy makes zero-cost memory manipulation effortless. We write `unsafe`
//! so you don't have to.
//!
//! [customer-request-issue]: https://github.com/google/zerocopy/issues/new/choose
//!
//! # Overview
//!
//! ##### Conversion Traits
//!
//! Zerocopy provides four derivable traits for zero-cost conversions:
//! - [`TryFromBytes`] indicates that a type may safely be converted from
//!   certain byte sequences (conditional on runtime checks)
//! - [`FromZeros`] indicates that a sequence of zero bytes represents a valid
//!   instance of a type
//! - [`FromBytes`] indicates that a type may safely be converted from an
//!   arbitrary byte sequence
//! - [`IntoBytes`] indicates that a type may safely be converted *to* a byte
//!   sequence
//!
//! ##### Marker Traits
//!
//! Zerocopy provides three derivable marker traits that do not provide any
//! functionality themselves, but are required to call certain methods provided
//! by the conversion traits:
//! - [`KnownLayout`] indicates that zerocopy can reason about certain layout
//!   qualities of a type
//! - [`Immutable`] indicates that a type is free from interior mutability,
//!   except by ownership or an exclusive (`&mut`) borrow
//! - [`Unaligned`] indicates that a type's alignment requirement is 1
//!
//! You should generally derive these marker traits whenever possible.
//!
//! ##### Conversion Macros
//!
//! Zerocopy provides three macros for safe, zero-cost casting between types:
//!
//! - [`transmute`] converts a value of one type to a value of another type of
//!   the same size
//! - [`transmute_mut`] converts a mutable reference of one type to a mutable
//!   reference of another type of the same size
//! - [`transmute_ref`] converts transmutes a mutable or immutable reference
//!   of one type to an immutable reference of another type of the same size
//!
//! These macros perform *compile-time* alignment and size checks, but cannot be
//! used in generic contexts. For generic conversions, use the methods defined
//! by the [conversion traits](#conversion-traits).
//!
//! ##### Byteorder-Aware Numerics
//!
//! Zerocopy provides byte-order aware integer types that support these
//! conversions; see the [`byteorder`] module. These types are especially useful
//! for network parsing.
//!
//! # Cargo Features
//!
//! - **`alloc`**   
//!   By default, `zerocopy` is `no_std`. When the `alloc` feature is enabled,
//!   the `alloc` crate is added as a dependency, and some allocation-related
//!   functionality is added.
//!
//! - **`derive`**   
//!   Provides derives for the core marker traits via the `zerocopy-derive`
//!   crate. These derives are re-exported from `zerocopy`, so it is not
//!   necessary to depend on `zerocopy-derive` directly.   
//!
//!   However, you may experience better compile times if you instead directly
//!   depend on both `zerocopy` and `zerocopy-derive` in your `Cargo.toml`,
//!   since doing so will allow Rust to compile these crates in parallel. To do
//!   so, do *not* enable the `derive` feature, and list both dependencies in
//!   your `Cargo.toml` with the same leading non-zero version number; e.g:
//!
//!   ```toml
//!   [dependencies]
//!   zerocopy = "0.X"
//!   zerocopy-derive = "0.X"
//!   ```
//!
//! - **`simd`**   
//!   When the `simd` feature is enabled, `FromZeros`, `FromBytes`, and
//!   `IntoBytes` impls are emitted for all stable SIMD types which exist on the
//!   target platform. Note that the layout of SIMD types is not yet stabilized,
//!   so these impls may be removed in the future if layout changes make them
//!   invalid. For more information, see the Unsafe Code Guidelines Reference
//!   page on the [layout of packed SIMD vectors][simd-layout].
//!
//! - **`simd-nightly`**   
//!   Enables the `simd` feature and adds support for SIMD types which are only
//!   available on nightly. Since these types are unstable, support for any type
//!   may be removed at any point in the future.
//!
//! [simd-layout]: https://rust-lang.github.io/unsafe-code-guidelines/layout/packed-simd-vectors.html
//!
//! # Security Ethos
//!
//! Zerocopy is expressly designed for use in security-critical contexts. We
//! strive to ensure that that zerocopy code is sound under Rust's current
//! memory model, and *any future memory model*. We ensure this by:
//! - **...not 'guessing' about Rust's semantics.**   
//!   We annotate `unsafe` code with a precise rationale for its soundness that
//!   cites a relevant section of Rust's official documentation. When Rust's
//!   documented semantics are unclear, we work with the Rust Operational
//!   Semantics Team to clarify Rust's documentation.
//! - **...rigorously testing our implementation.**   
//!   We run tests using [Miri], ensuring that zerocopy is sound across a wide
//!   array of supported target platforms of varying endianness and pointer
//!   width, and across both current and experimental memory models of Rust.
//! - **...formally proving the correctness of our implementation.**   
//!   We apply formal verification tools like [Kani][kani] to prove zerocopy's
//!   correctness.
//!
//! For more information, see our full [soundness policy].
//!
//! [Miri]: https://github.com/rust-lang/miri
//! [Kani]: https://github.com/model-checking/kani
//! [soundness policy]: https://github.com/google/zerocopy/blob/main/POLICIES.md#soundness
//!
//! # Relationship to Project Safe Transmute
//!
//! [Project Safe Transmute] is an official initiative of the Rust Project to
//! develop language-level support for safer transmutation. The Project consults
//! with crates like zerocopy to identify aspects of safer transmutation that
//! would benefit from compiler support, and has developed an [experimental,
//! compiler-supported analysis][mcp-transmutability] which determines whether,
//! for a given type, any value of that type may be soundly transmuted into
//! another type. Once this functionality is sufficiently mature, zerocopy
//! intends to replace its internal transmutability analysis (implemented by our
//! custom derives) with the compiler-supported one. This change will likely be
//! an implementation detail that is invisible to zerocopy's users.
//!
//! Project Safe Transmute will not replace the need for most of zerocopy's
//! higher-level abstractions. The experimental compiler analysis is a tool for
//! checking the soundness of `unsafe` code, not a tool to avoid writing
//! `unsafe` code altogether. For the foreseeable future, crates like zerocopy
//! will still be required in order to provide higher-level abstractions on top
//! of the building block provided by Project Safe Transmute.
//!
//! [Project Safe Transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html
//! [mcp-transmutability]: https://github.com/rust-lang/compiler-team/issues/411
//!
//! # MSRV
//!
//! See our [MSRV policy].
//!
//! [MSRV policy]: https://github.com/google/zerocopy/blob/main/POLICIES.md#msrv
//!
//! # Changelog
//!
//! Zerocopy uses [GitHub Releases].
//!
//! [GitHub Releases]: https://github.com/google/zerocopy/releases

// Sometimes we want to use lints which were added after our MSRV.
// `unknown_lints` is `warn` by default and we deny warnings in CI, so without
// this attribute, any unknown lint would cause a CI failure when testing with
// our MSRV.
#![allow(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![deny(
    anonymous_parameters,
    deprecated_in_future,
    late_bound_lifetime_arguments,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![cfg_attr(
    __INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS,
    deny(fuzzy_provenance_casts, lossy_provenance_casts)
)]
#![deny(
    clippy::all,
    clippy::alloc_instead_of_core,
    clippy::arithmetic_side_effects,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
    clippy::as_conversions,
    clippy::correctness,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::double_must_use,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::missing_inline_in_public_items,
    clippy::missing_safety_doc,
    clippy::must_use_candidate,
    clippy::must_use_unit,
    clippy::obfuscated_if_else,
    clippy::perf,
    clippy::print_stdout,
    clippy::return_self_not_must_use,
    clippy::std_instead_of_core,
    clippy::style,
    clippy::suspicious,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unwrap_used,
    clippy::use_debug
)]
#![allow(clippy::type_complexity)]
#![deny(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_intra_doc_links
)]
// In test code, it makes sense to weight more heavily towards concise, readable
// code over correct or debuggable code.
#![cfg_attr(any(test, kani), allow(
    // In tests, you get line numbers and have access to source code, so panic
    // messages are less important. You also often unwrap a lot, which would
    // make expect'ing instead very verbose.
    clippy::unwrap_used,
    // In tests, there's no harm to "panic risks" - the worst that can happen is
    // that your test will fail, and you'll fix it. By contrast, panic risks in
    // production code introduce the possibly of code panicking unexpectedly "in
    // the field".
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(
    all(feature = "simd-nightly", any(target_arch = "x86", target_arch = "x86_64")),
    feature(stdarch_x86_avx512)
)]
#![cfg_attr(
    all(feature = "simd-nightly", target_arch = "arm"),
    feature(stdarch_arm_dsp, stdarch_arm_neon_intrinsics)
)]
#![cfg_attr(
    all(feature = "simd-nightly", any(target_arch = "powerpc", target_arch = "powerpc64")),
    feature(stdarch_powerpc)
)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(
    __INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS,
    feature(layout_for_ptr, strict_provenance)
)]

// This is a hack to allow zerocopy-derive derives to work in this crate. They
// assume that zerocopy is linked as an extern crate, so they access items from
// it as `zerocopy::Xxx`. This makes that still work.
#[cfg(any(feature = "derive", test))]
extern crate self as zerocopy;

#[macro_use]
mod macros;

pub mod byteorder;
mod deprecated;
// This module is `pub` so that zerocopy's error types and error handling
// documentation is grouped together in a cohesive module. In practice, we
// expect most users to use the re-export of `error`'s items to avoid identifier
// stuttering.
pub mod error;
#[doc(hidden)]
pub mod layout;
#[doc(hidden)]
pub mod macro_util;
#[doc(hidden)]
pub mod pointer;
mod r#ref;
mod util;
// TODO(#252): If we make this pub, come up with a better name.
mod wrappers;

pub use crate::byteorder::*;
pub use crate::error::*;
pub use crate::r#ref::*;
pub use crate::wrappers::*;

use core::{
    cell::{self, RefMut, UnsafeCell},
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::Hasher,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
    },
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32,
        AtomicU8, AtomicUsize,
    },
};

use crate::pointer::{invariant, BecauseExclusive, BecauseImmutable};

#[cfg(any(feature = "alloc", test))]
extern crate alloc;
#[cfg(any(feature = "alloc", test))]
use alloc::{boxed::Box, vec::Vec};

#[cfg(any(feature = "alloc", test, kani))]
use core::alloc::Layout;

// Used by `TryFromBytes::is_bit_valid`.
#[doc(hidden)]
pub use crate::pointer::{Maybe, MaybeAligned, Ptr};
// Used by `KnownLayout`.
#[doc(hidden)]
pub use crate::layout::*;

// For each trait polyfill, as soon as the corresponding feature is stable, the
// polyfill import will be unused because method/function resolution will prefer
// the inherent method/function over a trait method/function. Thus, we suppress
// the `unused_imports` warning.
//
// See the documentation on `util::polyfills` for more information.
#[allow(unused_imports)]
use crate::util::polyfills::{self, NonNullExt as _, NumExt as _};

#[rustversion::nightly]
#[cfg(all(test, not(__INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)))]
const _: () = {
    #[deprecated = "some tests may be skipped due to missing RUSTFLAGS=\"--cfg __INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS\""]
    const _WARNING: () = ();
    #[warn(deprecated)]
    _WARNING
};

// These exist so that code which was written against the old names will get
// less confusing error messages when they upgrade to a more recent version of
// zerocopy. On our MSRV toolchain, the error messages read, for example:
//
//   error[E0603]: trait `FromZeroes` is private
//       --> examples/deprecated.rs:1:15
//        |
//   1    | use zerocopy::FromZeroes;
//        |               ^^^^^^^^^^ private trait
//        |
//   note: the trait `FromZeroes` is defined here
//       --> /Users/josh/workspace/zerocopy/src/lib.rs:1845:5
//        |
//   1845 | use FromZeros as FromZeroes;
//        |     ^^^^^^^^^^^^^^^^^^^^^^^
//
// The "note" provides enough context to make it easy to figure out how to fix
// the error.
#[allow(unused)]
use {FromZeros as FromZeroes, IntoBytes as AsBytes, Ref as LayoutVerified};

/// Implements [`KnownLayout`].
///
/// This derive analyzes various aspects of a type's layout that are needed for
/// some of zerocopy's APIs. It can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::KnownLayout;
/// #[derive(KnownLayout)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// enum MyEnum {
/// #   V00,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// # Limitations
///
/// This derive cannot currently be applied to unsized structs without an
/// explicit `repr` attribute.
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::KnownLayout;

/// Indicates that zerocopy can reason about certain aspects of a type's layout.
///
/// This trait is required by many of zerocopy's APIs.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(KnownLayout)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::KnownLayout;
/// #[derive(KnownLayout)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// enum MyEnum {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated analysis to deduce the layout
/// characteristics of types. You **must** implement this trait via the derive.
///
/// # Safety
///
/// This trait does not convey any safety guarantees to code outside this crate.
///
/// You must not rely on the `#[doc(hidden)]` internals of `KnownLayout`. Future
/// releases of zerocopy may make backwards-breaking changes to these items,
/// including changes that only affect soundness, which may cause code which
/// uses those items to silently become unsound.
///
#[cfg_attr(feature = "derive", doc = "[derive]: zerocopy_derive::KnownLayout")]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.KnownLayout.html"),
)]
pub unsafe trait KnownLayout {
    // The `Self: Sized` bound makes it so that `KnownLayout` can still be
    // object safe. It's not currently object safe thanks to `const LAYOUT`, and
    // it likely won't be in the future, but there's no reason not to be
    // forwards-compatible with object safety.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;

    /// The type of metadata stored in a pointer to `Self`.
    ///
    /// This is `()` for sized types and `usize` for slice DSTs.
    type PointerMetadata: PointerMetadata;

    /// The layout of `Self`.
    ///
    /// # Safety
    ///
    /// Callers may assume that `LAYOUT` accurately reflects the layout of
    /// `Self`. In particular:
    /// - `LAYOUT.align` is equal to `Self`'s alignment
    /// - If `Self: Sized`, then `LAYOUT.size_info == SizeInfo::Sized { size }`
    ///   where `size == mem::size_of::<Self>()`
    /// - If `Self` is a slice DST, then `LAYOUT.size_info ==
    ///   SizeInfo::SliceDst(slice_layout)` where:
    ///   - The size, `size`, of an instance of `Self` with `elems` trailing
    ///     slice elements is equal to `slice_layout.offset +
    ///     slice_layout.elem_size * elems` rounded up to the nearest multiple
    ///     of `LAYOUT.align`
    ///   - For such an instance, any bytes in the range `[slice_layout.offset +
    ///     slice_layout.elem_size * elems, size)` are padding and must not be
    ///     assumed to be initialized
    #[doc(hidden)]
    const LAYOUT: DstLayout;

    /// SAFETY: The returned pointer has the same address and provenance as
    /// `bytes`. If `Self` is a DST, the returned pointer's referent has `elems`
    /// elements in its trailing slice.
    #[doc(hidden)]
    fn raw_from_ptr_len(bytes: NonNull<u8>, meta: Self::PointerMetadata) -> NonNull<Self>;

    /// Extracts the metadata from a pointer to `Self`.
    ///
    /// # Safety
    ///
    /// `pointer_to_metadata` always returns the correct metadata stored in
    /// `ptr`.
    #[doc(hidden)]
    fn pointer_to_metadata(ptr: NonNull<Self>) -> Self::PointerMetadata;

    /// Computes the length of the byte range addressed by `ptr`.
    ///
    /// Returns `None` if the resulting length would not fit in an `usize`.
    ///
    /// # Safety
    ///
    /// Callers may assume that `size_of_val_raw` always returns the correct
    /// size.
    ///
    /// Callers may assume that, if `ptr` addresses a byte range whose length
    /// fits in an `usize`, this will return `Some`.
    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    fn size_of_val_raw(ptr: NonNull<Self>) -> Option<usize> {
        let meta = Self::pointer_to_metadata(ptr);
        // SAFETY: `size_for_metadata` promises to only return `None` if the
        // resulting size would not fit in a `usize`.
        meta.size_for_metadata(Self::LAYOUT)
    }
}

/// The metadata associated with a [`KnownLayout`] type.
#[doc(hidden)]
pub trait PointerMetadata: Copy + Eq + Debug {
    /// Constructs a `Self` from an element count.
    ///
    /// If `Self = ()`, this returns `()`. If `Self = usize`, this returns
    /// `elems`. No other types are currently supported.
    fn from_elem_count(elems: usize) -> Self;

    /// Computes the size of the object with the given layout and pointer
    /// metadata.
    ///
    /// # Panics
    ///
    /// If `Self = ()`, `layout` must describe a sized type. If `Self = usize`,
    /// `layout` must describe a slice DST. Otherwise, `size_for_metadata` may
    /// panic.
    ///
    /// # Safety
    ///
    /// `size_for_metadata` promises to only return `None` if the resulting size
    /// would not fit in a `usize`.
    fn size_for_metadata(&self, layout: DstLayout) -> Option<usize>;
}

impl PointerMetadata for () {
    #[inline]
    #[allow(clippy::unused_unit)]
    fn from_elem_count(_elems: usize) -> () {}

    #[inline]
    fn size_for_metadata(&self, layout: DstLayout) -> Option<usize> {
        match layout.size_info {
            SizeInfo::Sized { size } => Some(size),
            // NOTE: This branch is unreachable, but we return `None` rather
            // than `unreachable!()` to avoid generating panic paths.
            SizeInfo::SliceDst(_) => None,
        }
    }
}

impl PointerMetadata for usize {
    #[inline]
    fn from_elem_count(elems: usize) -> usize {
        elems
    }

    #[inline]
    fn size_for_metadata(&self, layout: DstLayout) -> Option<usize> {
        match layout.size_info {
            SizeInfo::SliceDst(TrailingSliceLayout { offset, elem_size }) => {
                let slice_len = elem_size.checked_mul(*self)?;
                let without_padding = offset.checked_add(slice_len)?;
                without_padding.checked_add(util::padding_needed_for(without_padding, layout.align))
            }
            // NOTE: This branch is unreachable, but we return `None` rather
            // than `unreachable!()` to avoid generating panic paths.
            SizeInfo::Sized { .. } => None,
        }
    }
}

// SAFETY: Delegates safety to `DstLayout::for_slice`.
unsafe impl<T> KnownLayout for [T] {
    #[allow(clippy::missing_inline_in_public_items)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }

    type PointerMetadata = usize;

    const LAYOUT: DstLayout = DstLayout::for_slice::<T>();

    // SAFETY: `.cast` preserves address and provenance. The returned pointer
    // refers to an object with `elems` elements by construction.
    #[inline(always)]
    fn raw_from_ptr_len(data: NonNull<u8>, elems: usize) -> NonNull<Self> {
        // TODO(#67): Remove this allow. See NonNullExt for more details.
        #[allow(unstable_name_collisions)]
        NonNull::slice_from_raw_parts(data.cast::<T>(), elems)
    }

    #[inline(always)]
    fn pointer_to_metadata(ptr: NonNull<[T]>) -> usize {
        #[allow(clippy::as_conversions)]
        let slc = ptr.as_ptr() as *const [()];

        // SAFETY:
        // - `()` has alignment 1, so `slc` is trivially aligned.
        // - `slc` was derived from a non-null pointer.
        // - The size is 0 regardless of the length, so it is sound to
        //   materialize a reference regardless of location.
        // - By invariant, `self.ptr` has valid provenance.
        let slc = unsafe { &*slc };

        // This is correct because the preceding `as` cast preserves the number
        // of slice elements. Per
        // https://doc.rust-lang.org/nightly/reference/expressions/operator-expr.html#slice-dst-pointer-to-pointer-cast:
        //
        //   For slice types like `[T]` and `[U]`, the raw pointer types `*const
        //   [T]`, `*mut [T]`, `*const [U]`, and `*mut [U]` encode the number of
        //   elements in this slice. Casts between these raw pointer types
        //   preserve the number of elements. Note that, as a consequence, such
        //   casts do *not* necessarily preserve the size of the pointer's
        //   referent (e.g., casting `*const [u16]` to `*const [u8]` will result
        //   in a raw pointer which refers to an object of half the size of the
        //   original). The same holds for `str` and any compound type whose
        //   unsized tail is a slice type, such as struct `Foo(i32, [u8])` or
        //   `(u64, Foo)`.
        //
        // TODO(#429),
        // TODO(https://github.com/rust-lang/reference/pull/1417): Once this
        // text is available on the Stable docs, cite those instead of the
        // Nightly docs.
        slc.len()
    }
}

#[rustfmt::skip]
impl_known_layout!(
    (),
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64,
    bool, char,
    NonZeroU8, NonZeroI8, NonZeroU16, NonZeroI16, NonZeroU32, NonZeroI32,
    NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize,
    AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU8, AtomicUsize
);
#[rustfmt::skip]
impl_known_layout!(
    T         => Option<T>,
    T: ?Sized => PhantomData<T>,
    T         => Wrapping<T>,
    T         => MaybeUninit<T>,
    T: ?Sized => *const T,
    T: ?Sized => *mut T,
    T         => AtomicPtr<T>
);
impl_known_layout!(const N: usize, T => [T; N]);

safety_comment! {
    /// SAFETY:
    /// `str`, `ManuallyDrop<[T]>` [1], and `UnsafeCell<T>` [2] have the same
    /// representations as `[u8]`, `[T]`, and `T` repsectively. `str` has
    /// different bit validity than `[u8]`, but that doesn't affect the
    /// soundness of this impl.
    ///
    /// [1] Per https://doc.rust-lang.org/nightly/core/mem/struct.ManuallyDrop.html:
    ///
    ///   `ManuallyDrop<T>` is guaranteed to have the same layout and bit
    ///   validity as `T`
    ///
    /// [2] Per https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html#memory-layout:
    ///
    ///   `UnsafeCell<T>` has the same in-memory representation as its inner
    ///   type `T`.
    ///
    /// TODO(#429):
    /// -  Add quotes from docs.
    /// -  Once [1] (added in
    /// https://github.com/rust-lang/rust/pull/115522) is available on stable,
    /// quote the stable docs instead of the nightly docs.
    unsafe_impl_known_layout!(#[repr([u8])] str);
    unsafe_impl_known_layout!(T: ?Sized + KnownLayout => #[repr(T)] ManuallyDrop<T>);
    unsafe_impl_known_layout!(T: ?Sized + KnownLayout => #[repr(T)] UnsafeCell<T>);
}

/// Analyzes whether a type is [`FromZeros`].
///
/// This derive analyzes, at compile time, whether the annotated type satisfies
/// the [safety conditions] of `FromZeros` and implements `FromZeros` if it is
/// sound to do so. This derive can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{FromZeros, Immutable};
/// #[derive(FromZeros)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// [safety conditions]: trait@FromZeros#safety
///
/// # Analysis
///
/// *This section describes, roughly, the analysis performed by this derive to
/// determine whether it is sound to implement `FromZeros` for a given type.
/// Unless you are modifying the implementation of this derive, or attempting to
/// manually implement `FromZeros` for a type yourself, you don't need to read
/// this section.*
///
/// If a type has the following properties, then this derive can implement
/// `FromZeros` for that type:
///
/// - If the type is a struct, all of its fields must be `FromZeros`.
/// - If the type is an enum, it must be C-like (meaning that all variants have
///   no fields) and it must have a variant with a discriminant of `0`. See [the
///   reference] for a description of how discriminant values are chosen.
///
/// This analysis is subject to change. Unsafe code may *only* rely on the
/// documented [safety conditions] of `FromZeros`, and must *not* rely on the
/// implementation details of this derive.
///
/// [the reference]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations
///
/// ## Why isn't an explicit representation required for structs?
///
/// Neither this derive, nor the [safety conditions] of `FromZeros`, requires
/// that structs are marked with `#[repr(C)]`.
///
/// Per the [Rust reference](reference),
///
/// > The representation of a type can change the padding between fields, but
/// does not change the layout of the fields themselves.
///
/// [reference]: https://doc.rust-lang.org/reference/type-layout.html#representations
///
/// Since the layout of structs only consists of padding bytes and field bytes,
/// a struct is soundly `FromZeros` if:
/// 1. its padding is soundly `FromZeros`, and
/// 2. its fields are soundly `FromZeros`.
///
/// The answer to the first question is always yes: padding bytes do not have
/// any validity constraints. A [discussion] of this question in the Unsafe Code
/// Guidelines Working Group concluded that it would be virtually unimaginable
/// for future versions of rustc to add validity constraints to padding bytes.
///
/// [discussion]: https://github.com/rust-lang/unsafe-code-guidelines/issues/174
///
/// Whether a struct is soundly `FromZeros` therefore solely depends on whether
/// its fields are `FromZeros`.
// TODO(#146): Document why we don't require an enum to have an explicit `repr`
// attribute.
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::FromZeros;

/// Analyzes whether a type is [`Immutable`].
///
/// This derive analyzes, at compile time, whether the annotated type satisfies
/// the [safety conditions] of `Immutable` and implements `Immutable` if it is
/// sound to do so. This derive can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::Immutable;
/// #[derive(Immutable)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// # Analysis
///
/// *This section describes, roughly, the analysis performed by this derive to
/// determine whether it is sound to implement `Immutable` for a given type.
/// Unless you are modifying the implementation of this derive, you don't need
/// to read this section.*
///
/// If a type has the following properties, then this derive can implement
/// `Immutable` for that type:
///
/// - All fields must be `Immutable`.
///
/// This analysis is subject to change. Unsafe code may *only* rely on the
/// documented [safety conditions] of `Immutable`, and must *not* rely on the
/// implementation details of this derive.
///
/// [safety conditions]: trait@Immutable#safety
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::Immutable;

/// Types which are free from interior mutability.
///
/// `T: Immutable` indicates that `T` does not permit interior mutability, except
/// by ownership or an exclusive (`&mut`) borrow.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(Immutable)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::Immutable;
/// #[derive(Immutable)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// enum MyEnum {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `Immutable`.
///
/// # Safety
///
/// Unsafe code outside of this crate must not make any assumptions about `T`
/// based on `T: Immutable`. We reserve the right to relax the requirements for
/// `Immutable` in the future, and if unsafe code outside of this crate makes
/// assumptions based on `T: Immutable`, future relaxations may cause that code
/// to become unsound.
///
// # Safety (Internal)
//
// If `T: Immutable`, unsafe code *inside of this crate* may assume that, given
// `t: &T`, `t` does not contain any [`UnsafeCell`]s at any byte location
// within the byte range addressed by `t`. This includes ranges of length 0
// (e.g., `UnsafeCell<()>` and `[UnsafeCell<u8>; 0]`). If a type implements
// `Immutable` which violates this assumptions, it may cause this crate to
// exhibit [undefined behavior].
//
// [`UnsafeCell`]: core::cell::UnsafeCell
// [undefined behavior]: https://raphlinus.github.io/programming/rust/2018/08/17/undefined-behavior.html
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::Immutable",
    doc = "[derive-analysis]: zerocopy_derive::Immutable#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Immutable.html"),
    doc = concat!("[derive-analysis]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Immutable.html#analysis"),
)]
pub unsafe trait Immutable {
    // The `Self: Sized` bound makes it so that `Immutable` is still object
    // safe.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
}

/// Implements [`TryFromBytes`].
///
/// This derive synthesizes the runtime checks required to check whether a
/// sequence of initialized bytes corresponds to a valid instance of a type.
/// This derive can be applied to structs, enums, and unions; e.g.:
///
/// ```
/// # use zerocopy_derive::{TryFromBytes, Immutable};
/// #[derive(TryFromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// [safety conditions]: trait@TryFromBytes#safety
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::TryFromBytes;

/// Types for which some bit patterns are valid.
///
/// A memory region of the appropriate length which contains initialized bytes
/// can be viewed as a `TryFromBytes` type so long as the runtime value of those
/// bytes corresponds to a [*valid instance*] of that type. For example,
/// [`bool`] is `TryFromBytes`; zerocopy can transmute a `[u8]` into a [`bool`]
/// so long as it first checks that the value of the `[u8]` is `[0]` or `[1]`.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(TryFromBytes)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{TryFromBytes, Immutable};
/// #[derive(TryFromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive ensures that the runtime check of whether bytes correspond to a
/// valid instance is sound. You **must** implement this trait via the derive.
///
/// # What is a "valid instance"?
///
/// In Rust, each type has *bit validity*, which refers to the set of bit
/// patterns which may appear in an instance of that type. It is impossible for
/// safe Rust code to produce values which violate bit validity (ie, values
/// outside of the "valid" set of bit patterns). If `unsafe` code produces an
/// invalid value, this is considered [undefined behavior].
///
/// Rust's bit validity rules are currently being decided, which means that some
/// types have three classes of bit patterns: those which are definitely valid,
/// and whose validity is documented in the language; those which may or may not
/// be considered valid at some point in the future; and those which are
/// definitely invalid.
///
/// Zerocopy takes a conservative approach, and only considers a bit pattern to
/// be valid if its validity is a documenteed guarantee provided by the
/// language.
///
/// For most use cases, Rust's current guarantees align with programmers'
/// intuitions about what ought to be valid. As a result, zerocopy's
/// conservatism should not affect most users. One notable exception is unions,
/// whose bit validity is very up in the air; zerocopy does not permit
/// implementing `TryFromBytes` for any union type.
///
/// If you are negatively affected by lack of support for a particular type,
/// we encourage you to let us know by [filing an issue][github-repo].
///
/// # `TryFromBytes` is not symmetrical with [`IntoBytes`]
///
/// There are some types which implement both `TryFromBytes` and [`IntoBytes`],
/// but for which `TryFromBytes` is not guaranteed to accept all byte sequences
/// produced by `IntoBytes`. In other words, for some `T: TryFromBytes +
/// IntoBytes`, there exist values of `t: T` such that
/// `TryFromBytes::try_ref_from(t.as_bytes()) == None`. Code should not
/// generally assume that values produced by `IntoBytes` will necessarily be
/// accepted as valid by `TryFromBytes`.
///
/// # Safety
///
/// On its own, `T: TryFromBytes` does not make any guarantees about the layout
/// or representation of `T`. It merely provides the ability to perform a
/// validity check at runtime via methods like [`try_ref_from`].
///
/// You must not rely on the `#[doc(hidden)]` internals of `TryFromBytes`.
/// Future releases of zerocopy may make backwards-breaking changes to these
/// items, including changes that only affect soundness, which may cause code
/// which uses those items to silently become unsound.
///
/// [undefined behavior]: https://raphlinus.github.io/programming/rust/2018/08/17/undefined-behavior.html
/// [github-repo]: https://github.com/google/zerocopy
/// [`try_ref_from`]: TryFromBytes::try_ref_from
/// [*valid instance*]: #what-is-a-valid-instance
#[cfg_attr(feature = "derive", doc = "[derive]: zerocopy_derive::TryFromBytes")]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.TryFromBytes.html"),
)]
pub unsafe trait TryFromBytes {
    // The `Self: Sized` bound makes it so that `TryFromBytes` is still object
    // safe.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;

    /// Does a given memory range contain a valid instance of `Self`?
    ///
    /// # Safety
    ///
    /// Unsafe code may assume that, if `is_bit_valid(candidate)` returns true,
    /// `*candidate` contains a valid `Self`.
    ///
    /// # Panics
    ///
    /// `is_bit_valid` may panic. Callers are responsible for ensuring that any
    /// `unsafe` code remains sound even in the face of `is_bit_valid`
    /// panicking. (We support user-defined validation routines; so long as
    /// these routines are not required to be `unsafe`, there is no way to
    /// ensure that these do not generate panics.)
    ///
    /// Besides user-defined validation routines panicking, `is_bit_valid` will
    /// either panic or fail to compile if called on a pointer with [`Shared`]
    /// aliasing when `Self: !Immutable`.
    ///
    /// [`UnsafeCell`]: core::cell::UnsafeCell
    /// [`Shared`]: invariant::Shared
    #[doc(hidden)]
    fn is_bit_valid<A: invariant::Aliasing + invariant::AtLeast<invariant::Shared>>(
        candidate: Maybe<'_, Self, A>,
    ) -> bool;

    /// Attempts to interpret the given `candidate` as a `&Self` without
    /// copying.
    ///
    /// If the bytes of `candidate` are a valid instance of `Self`, this method
    /// returns a reference to those bytes interpreted as a `Self`. If
    /// `candidate.len() < size_of::<Self>()` or `candidate` is not aligned to
    /// `align_of::<Self>()` or the bytes are not a valid instance of `Self`,
    /// this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the byte sequence `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    ///
    /// let packet = Packet::try_ref_from(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0x70, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    /// assert!(Packet::try_ref_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from(candidate: &[u8]) -> Result<&Self, TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        util::assert_dst_is_not_zst::<Self>();
        match Ptr::from_ref(candidate).try_cast_into_no_leftover::<Self, BecauseImmutable>(None) {
            Ok(candidate) => {
                // This call may panic. If that happens, it doesn't cause any soundness
                // issues, as we have not generated any invalid state which we need to
                // fix before returning.
                //
                // Note that one panic or post-monomorphization error condition is
                // calling `try_into_valid` (and thus `is_bit_valid`) with a shared
                // pointer when `Self: !Immutable`. Since `Self: Immutable`, this panic
                // condition will not happen.
                match candidate.try_into_valid() {
                    Ok(valid) => Ok(valid.as_ref()),
                    Err(e) => {
                        Err(e.map_src(|src| src.as_bytes::<BecauseImmutable>().as_ref()).into())
                    }
                }
            }
            Err(e) => Err(e.map_src(Ptr::as_ref).into()),
        }
    }

    /// Attempts to interpret the prefix of the given `candidate` as a `&Self`
    /// without copying.
    ///
    /// If the first `size_of::<Self>()` bytes of `candidate` are a valid
    /// instance of `Self`, this method returns both a reference to those bytes
    /// interpreted as a `Self`, and a reference to the remaining bytes. If
    /// `candidate.len() < size_of::<Self>()` or `candidate` is not aligned to
    /// `align_of::<Self>()` or the bytes are not a valid instance of `Self`,
    /// this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from_prefix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    ///
    /// let (packet, excess) = Packet::try_ref_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    /// assert_eq!(excess, &[6u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0x70, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_ref_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_prefix(candidate: &[u8]) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        util::assert_dst_is_not_zst::<Self>();
        try_ref_from_prefix_suffix(candidate, CastType::Prefix, None)
    }

    /// Attempts to interpret the suffix of the given `candidate` as a `&Self`
    /// without copying.
    ///
    /// If the last `size_of::<Self>()` bytes of `candidate` are a valid
    /// instance of `Self`, this method returns both a reference to those bytes
    /// interpreted as a `Self`, and a reference to the preceding bytes. If
    /// `candidate.len() < size_of::<Self>()` or the suffix of `candidate` is
    /// not aligned to `align_of::<Self>()` or the suffix of `candidate` is not
    /// a valid instance of `Self`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from_suffix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (excess, packet) = Packet::try_ref_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(excess, &[0u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0x70][..];
    /// assert!(Packet::try_ref_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_suffix(candidate: &[u8]) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        util::assert_dst_is_not_zst::<Self>();
        try_ref_from_prefix_suffix(candidate, CastType::Suffix, None).map(swap)
    }

    /// Attempts to interpret the given `candidate` as a `&mut Self` without
    /// copying.
    ///
    /// If the bytes of `candidate` are a valid instance of `Self`, this method
    /// returns a reference to those bytes interpreted as a `Self`. If
    /// `candidate.len() < size_of::<Self>()` or `candidate` is not aligned to
    /// `align_of::<Self>()` or the bytes are not a valid instance of `Self`,
    /// this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    ///
    /// let packet = Packet::try_mut_from(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x70, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_mut_from(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from(bytes: &mut [u8]) -> Result<&mut Self, TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        match Ptr::from_mut(bytes).try_cast_into_no_leftover::<Self, BecauseExclusive>(None) {
            Ok(candidate) => {
                // This call may panic. If that happens, it doesn't cause any soundness
                // issues, as we have not generated any invalid state which we need to
                // fix before returning.
                //
                // Note that one panic or post-monomorphization error condition is
                // calling `try_into_valid` (and thus `is_bit_valid`) with a shared
                // pointer when `Self: !Immutable`. Since `Self: Immutable`, this panic
                // condition will not happen.
                match candidate.try_into_valid() {
                    Ok(candidate) => Ok(candidate.as_mut()),
                    Err(e) => {
                        Err(e.map_src(|src| src.as_bytes::<BecauseExclusive>().as_mut()).into())
                    }
                }
            }
            Err(e) => Err(e.map_src(Ptr::as_mut).into()),
        }
    }

    /// Attempts to interpret the prefix of the given `candidate` as a `&mut
    /// Self` without copying.
    ///
    /// If the first `size_of::<Self>()` bytes of `candidate` are a valid
    /// instance of `Self`, this method returns both a reference to those bytes
    /// interpreted as a `Self`, and a reference to the remaining bytes. If
    /// `candidate.len() < size_of::<Self>()` or `candidate` is not aligned to
    /// `align_of::<Self>()` or the bytes are not a valid instance of `Self`,
    /// this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from_prefix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    ///
    /// let (packet, excess) = Packet::try_mut_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    /// assert_eq!(excess, &[6u8][..]);
    ///
    /// packet.temperature = 111;
    /// excess[0] = 222;
    ///
    /// assert_eq!(bytes, [0xC0, 0xC0, 240, 111, 0, 1, 2, 3, 4, 5, 222]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x70, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_mut_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_prefix(
        candidate: &mut [u8],
    ) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        try_mut_from_prefix_suffix(candidate, CastType::Prefix, None)
    }

    /// Attempts to interpret the suffix of the given `candidate` as a `&mut
    /// Self` without copying.
    ///
    /// If the last `size_of::<Self>()` bytes of `candidate` are a valid
    /// instance of `Self`, this method returns both a reference to those bytes
    /// interpreted as a `Self`, and a reference to the preceding bytes. If
    /// `candidate.len() < size_of::<Self>()` or the suffix of `candidate` is
    /// not aligned to `align_of::<Self>()` or the suffix of `candidate` is not
    /// a valid instance of `Self`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from_suffix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &mut [0, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (excess, packet) = Packet::try_mut_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(excess, &[0u8][..]);
    ///
    /// excess[0] = 111;
    /// packet.temperature = 222;
    ///
    /// assert_eq!(bytes, [111, 0xC0, 0xC0, 240, 222, 2, 3, 4, 5, 6, 7]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0x70][..];
    /// assert!(Packet::try_mut_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_suffix(
        candidate: &mut [u8],
    ) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        try_mut_from_prefix_suffix(candidate, CastType::Suffix, None).map(swap)
    }

    /// Attempts to read the given `candidate` as a `Self`.
    ///
    /// If the bytes of `candidate` are a valid instance of `Self`, reads those
    /// bytes as `Self`. If `candidate.len() < size_of::<Self>()` or the bytes
    /// are not a valid instance of `Self`, this returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77][..];
    ///
    /// let packet = Packet::try_read_from(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x70, 0xC0, 240, 77][..];
    /// assert!(Packet::try_read_from(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_read_from(bytes: &[u8]) -> Result<Self, TryReadError<&[u8], Self>>
    where
        Self: Sized,
    {
        // Note that we have to call `is_bit_valid` on an exclusive-aliased
        // pointer since we don't require `Self: Immutable`. That's why we do `let
        // mut` and `Ptr::from_mut` here. See the doc comment on `is_bit_valid`
        // and the implementation of `TryFromBytes` for `UnsafeCell` for more
        // details.
        let mut candidate = match MaybeUninit::<Self>::read_from(bytes) {
            Ok(candidate) => candidate,
            Err(e) => {
                return Err(TryReadError::Size(e.with_dst()));
            }
        };
        let c_ptr = Ptr::from_mut(&mut candidate);
        let c_ptr = c_ptr.transparent_wrapper_into_inner();
        // SAFETY: `c_ptr` has no uninitialized sub-ranges because it derived
        // from `candidate`, which in turn derives from `bytes: &[u8]`.
        let c_ptr = unsafe { c_ptr.assume_validity::<invariant::Initialized>() };

        // This call may panic. If that happens, it doesn't cause any soundness
        // issues, as we have not generated any invalid state which we need to
        // fix before returning.
        //
        // Note that one panic or post-monomorphization error condition is
        // calling `try_into_valid` (and thus `is_bit_valid`) with a shared
        // pointer when `Self: !Immutable`. Since `Self: Immutable`, this panic
        // condition will not happen.
        if !Self::is_bit_valid(c_ptr.forget_aligned()) {
            return Err(ValidityError::new(bytes).into());
        }

        // SAFETY: We just validated that `candidate` contains a valid `Self`.
        Ok(unsafe { candidate.assume_init() })
    }
}

#[inline(always)]
fn try_ref_from_prefix_suffix<T: TryFromBytes + KnownLayout + Immutable + ?Sized>(
    candidate: &[u8],
    cast_type: CastType,
    meta: Option<T::PointerMetadata>,
) -> Result<(&T, &[u8]), TryCastError<&[u8], T>> {
    match Ptr::from_ref(candidate).try_cast_into::<T, BecauseImmutable>(cast_type, meta) {
        Ok((candidate, prefix_suffix)) => {
            // This call may panic. If that happens, it doesn't cause any soundness
            // issues, as we have not generated any invalid state which we need to
            // fix before returning.
            //
            // Note that one panic or post-monomorphization error condition is
            // calling `try_into_valid` (and thus `is_bit_valid`) with a shared
            // pointer when `Self: !Immutable`. Since `Self: Immutable`, this panic
            // condition will not happen.
            match candidate.try_into_valid() {
                Ok(valid) => Ok((valid.as_ref(), prefix_suffix.as_ref())),
                Err(e) => Err(e.map_src(|src| src.as_bytes::<BecauseImmutable>().as_ref()).into()),
            }
        }
        Err(e) => Err(e.map_src(Ptr::as_ref).into()),
    }
}

#[inline(always)]
fn try_mut_from_prefix_suffix<T: TryFromBytes + KnownLayout + ?Sized>(
    candidate: &mut [u8],
    cast_type: CastType,
    meta: Option<T::PointerMetadata>,
) -> Result<(&mut T, &mut [u8]), TryCastError<&mut [u8], T>> {
    match Ptr::from_mut(candidate).try_cast_into::<T, BecauseExclusive>(cast_type, meta) {
        Ok((candidate, prefix_suffix)) => {
            // This call may panic. If that happens, it doesn't cause any soundness
            // issues, as we have not generated any invalid state which we need to
            // fix before returning.
            //
            // Note that one panic or post-monomorphization error condition is
            // calling `try_into_valid` (and thus `is_bit_valid`) with a shared
            // pointer when `Self: !Immutable`. Since `Self: Immutable`, this panic
            // condition will not happen.
            match candidate.try_into_valid() {
                Ok(valid) => Ok((valid.as_mut(), prefix_suffix.as_mut())),
                Err(e) => Err(e.map_src(|src| src.as_bytes::<BecauseExclusive>().as_mut()).into()),
            }
        }
        Err(e) => Err(e.map_src(Ptr::as_mut).into()),
    }
}

#[inline(always)]
fn swap<T, U>((t, u): (T, U)) -> (U, T) {
    (u, t)
}

/// Types for which a sequence of bytes all set to zero represents a valid
/// instance of the type.
///
/// Any memory region of the appropriate length which is guaranteed to contain
/// only zero bytes can be viewed as any `FromZeros` type with no runtime
/// overhead. This is useful whenever memory is known to be in a zeroed state,
/// such memory returned from some allocation routines.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(FromZeros)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{FromZeros, Immutable};
/// #[derive(FromZeros)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `FromZeros`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: FromZeros`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `FromZeros` manually, and you don't plan on writing unsafe code that
/// operates on `FromZeros` types, then you don't need to read this section.*
///
/// If `T: FromZeros`, then unsafe code may assume that it is sound to produce a
/// `T` whose bytes are all initialized to zero. If a type is marked as
/// `FromZeros` which violates this contract, it may cause undefined behavior.
///
/// `#[derive(FromZeros)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::FromZeros",
    doc = "[derive-analysis]: zerocopy_derive::FromZeros#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromZeros.html"),
    doc = concat!("[derive-analysis]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromZeros.html#analysis"),
)]
pub unsafe trait FromZeros: TryFromBytes {
    // The `Self: Sized` bound makes it so that `FromZeros` is still object
    // safe.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;

    /// Overwrites `self` with zeros.
    ///
    /// Sets every byte in `self` to 0. While this is similar to doing `*self =
    /// Self::new_zeroed()`, it differs in that `zero` does not semantically
    /// drop the current value and replace it with a new one - it simply
    /// modifies the bytes of the existing value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerocopy::FromZeros;
    /// # use zerocopy_derive::*;
    /// #
    /// #[derive(FromZeros)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let mut header = PacketHeader {
    ///     src_port: 100u16.to_be_bytes(),
    ///     dst_port: 200u16.to_be_bytes(),
    ///     length: 300u16.to_be_bytes(),
    ///     checksum: 400u16.to_be_bytes(),
    /// };
    ///
    /// header.zero();
    ///
    /// assert_eq!(header.src_port, [0, 0]);
    /// assert_eq!(header.dst_port, [0, 0]);
    /// assert_eq!(header.length, [0, 0]);
    /// assert_eq!(header.checksum, [0, 0]);
    /// ```
    #[inline(always)]
    fn zero(&mut self) {
        let slf: *mut Self = self;
        let len = mem::size_of_val(self);
        // SAFETY:
        // - `self` is guaranteed by the type system to be valid for writes of
        //   size `size_of_val(self)`.
        // - `u8`'s alignment is 1, and thus `self` is guaranteed to be aligned
        //   as required by `u8`.
        // - Since `Self: FromZeros`, the all-zeros instance is a valid instance
        //   of `Self.`
        //
        // TODO(#429): Add references to docs and quotes.
        unsafe { ptr::write_bytes(slf.cast::<u8>(), 0, len) };
    }

    /// Creates an instance of `Self` from zeroed bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerocopy::FromZeros;
    /// # use zerocopy_derive::*;
    /// #
    /// #[derive(FromZeros)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header: PacketHeader = FromZeros::new_zeroed();
    ///
    /// assert_eq!(header.src_port, [0, 0]);
    /// assert_eq!(header.dst_port, [0, 0]);
    /// assert_eq!(header.length, [0, 0]);
    /// assert_eq!(header.checksum, [0, 0]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn new_zeroed() -> Self
    where
        Self: Sized,
    {
        // SAFETY: `FromZeros` says that the all-zeros bit pattern is legal.
        unsafe { mem::zeroed() }
    }

    /// Creates a `Box<Self>` from zeroed bytes.
    ///
    /// This function is useful for allocating large values on the heap and
    /// zero-initializing them, without ever creating a temporary instance of
    /// `Self` on the stack. For example, `<[u8; 1048576]>::new_box_zeroed()`
    /// will allocate `[u8; 1048576]` directly on the heap; it does not require
    /// storing `[u8; 1048576]` in a temporary variable on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_box_zeroed` (or related functions) may
    /// have performance benefits.
    ///
    /// Note that `Box<Self>` can be converted to `Arc<Self>` and other
    /// container types without reallocation.
    ///
    /// # Panics
    ///
    /// Panics if allocation of `size_of::<Self>()` bytes fails.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(any(feature = "alloc", test))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline]
    fn new_box_zeroed() -> Box<Self>
    where
        Self: Sized,
    {
        // If `T` is a ZST, then return a proper boxed instance of it. There is
        // no allocation, but `Box` does require a correct dangling pointer.
        let layout = Layout::new::<Self>();
        if layout.size() == 0 {
            return Box::new(Self::new_zeroed());
        }

        // TODO(#429): Add a "SAFETY" comment and remove this `allow`.
        #[allow(clippy::undocumented_unsafe_blocks)]
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout).cast::<Self>() };
        if ptr.is_null() {
            alloc::alloc::handle_alloc_error(layout);
        }
        // TODO(#429): Add a "SAFETY" comment and remove this `allow`.
        #[allow(clippy::undocumented_unsafe_blocks)]
        unsafe {
            Box::from_raw(ptr)
        }
    }

    /// Creates a `Box<[Self]>` (a boxed slice) from zeroed bytes.
    ///
    /// This function is useful for allocating large values of `[Self]` on the
    /// heap and zero-initializing them, without ever creating a temporary
    /// instance of `[Self; _]` on the stack. For example,
    /// `u8::new_box_slice_zeroed(1048576)` will allocate the slice directly on
    /// the heap; it does not require storing the slice on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_box_slice_zeroed` may have performance
    /// benefits.
    ///
    /// If `Self` is a zero-sized type, then this function will return a
    /// `Box<[Self]>` that has the correct `len`. Such a box cannot contain any
    /// actual information, but its `len()` property will report the correct
    /// value.
    ///
    /// # Panics
    ///
    /// * Panics if `size_of::<Self>() * len` overflows.
    /// * Panics if allocation of `size_of::<Self>() * len` bytes fails.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline]
    fn new_box_slice_zeroed(len: usize) -> Box<[Self]>
    where
        Self: Sized,
    {
        let size = mem::size_of::<Self>()
            .checked_mul(len)
            .expect("mem::size_of::<Self>() * len overflows `usize`");
        let align = mem::align_of::<Self>();
        // On stable Rust versions <= 1.64.0, `Layout::from_size_align` has a
        // bug in which sufficiently-large allocations (those which, when
        // rounded up to the alignment, overflow `isize`) are not rejected,
        // which can cause undefined behavior. See #64 for details.
        //
        // TODO(#67): Once our MSRV is > 1.64.0, remove this assertion.
        #[allow(clippy::as_conversions)]
        let max_alloc = (isize::MAX as usize).saturating_sub(align);
        assert!(size <= max_alloc);
        // TODO(https://github.com/rust-lang/rust/issues/55724): Use
        // `Layout::repeat` once it's stabilized.
        let layout =
            Layout::from_size_align(size, align).expect("total allocation size overflows `isize`");

        let ptr = if layout.size() != 0 {
            // TODO(#429): Add a "SAFETY" comment and remove this `allow`.
            #[allow(clippy::undocumented_unsafe_blocks)]
            let ptr = unsafe { alloc::alloc::alloc_zeroed(layout).cast::<Self>() };
            if ptr.is_null() {
                alloc::alloc::handle_alloc_error(layout);
            }
            ptr
        } else {
            // `Box<[T]>` does not allocate when `T` is zero-sized or when `len`
            // is zero, but it does require a non-null dangling pointer for its
            // allocation.
            NonNull::<Self>::dangling().as_ptr()
        };

        // TODO(#429): Add a "SAFETY" comment and remove this `allow`.
        #[allow(clippy::undocumented_unsafe_blocks)]
        unsafe {
            Box::from_raw(slice::from_raw_parts_mut(ptr, len))
        }
    }

    /// Creates a `Vec<Self>` from zeroed bytes.
    ///
    /// This function is useful for allocating large values of `Vec`s and
    /// zero-initializing them, without ever creating a temporary instance of
    /// `[Self; _]` (or many temporary instances of `Self`) on the stack. For
    /// example, `u8::new_vec_zeroed(1048576)` will allocate directly on the
    /// heap; it does not require storing intermediate values on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_vec_zeroed` may have performance benefits.
    ///
    /// If `Self` is a zero-sized type, then this function will return a
    /// `Vec<Self>` that has the correct `len`. Such a `Vec` cannot contain any
    /// actual information, but its `len()` property will report the correct
    /// value.
    ///
    /// # Panics
    ///
    /// * Panics if `size_of::<Self>() * len` overflows.
    /// * Panics if allocation of `size_of::<Self>() * len` bytes fails.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn new_vec_zeroed(len: usize) -> Vec<Self>
    where
        Self: Sized,
    {
        Self::new_box_slice_zeroed(len).into()
    }
}

/// Analyzes whether a type is [`FromBytes`].
///
/// This derive analyzes, at compile time, whether the annotated type satisfies
/// the [safety conditions] of `FromBytes` and implements `FromBytes` if it is
/// sound to do so. This derive can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{FromBytes, FromZeros, Immutable};
/// #[derive(FromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V0A, V0B, V0C, V0D, V0E,
/// #   V0F, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V1A, V1B, V1C, V1D,
/// #   V1E, V1F, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V2A, V2B, V2C,
/// #   V2D, V2E, V2F, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V3A, V3B,
/// #   V3C, V3D, V3E, V3F, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V4A,
/// #   V4B, V4C, V4D, V4E, V4F, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59,
/// #   V5A, V5B, V5C, V5D, V5E, V5F, V60, V61, V62, V63, V64, V65, V66, V67, V68,
/// #   V69, V6A, V6B, V6C, V6D, V6E, V6F, V70, V71, V72, V73, V74, V75, V76, V77,
/// #   V78, V79, V7A, V7B, V7C, V7D, V7E, V7F, V80, V81, V82, V83, V84, V85, V86,
/// #   V87, V88, V89, V8A, V8B, V8C, V8D, V8E, V8F, V90, V91, V92, V93, V94, V95,
/// #   V96, V97, V98, V99, V9A, V9B, V9C, V9D, V9E, V9F, VA0, VA1, VA2, VA3, VA4,
/// #   VA5, VA6, VA7, VA8, VA9, VAA, VAB, VAC, VAD, VAE, VAF, VB0, VB1, VB2, VB3,
/// #   VB4, VB5, VB6, VB7, VB8, VB9, VBA, VBB, VBC, VBD, VBE, VBF, VC0, VC1, VC2,
/// #   VC3, VC4, VC5, VC6, VC7, VC8, VC9, VCA, VCB, VCC, VCD, VCE, VCF, VD0, VD1,
/// #   VD2, VD3, VD4, VD5, VD6, VD7, VD8, VD9, VDA, VDB, VDC, VDD, VDE, VDF, VE0,
/// #   VE1, VE2, VE3, VE4, VE5, VE6, VE7, VE8, VE9, VEA, VEB, VEC, VED, VEE, VEF,
/// #   VF0, VF1, VF2, VF3, VF4, VF5, VF6, VF7, VF8, VF9, VFA, VFB, VFC, VFD, VFE,
/// #   VFF,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// [safety conditions]: trait@FromBytes#safety
///
/// # Analysis
///
/// *This section describes, roughly, the analysis performed by this derive to
/// determine whether it is sound to implement `FromBytes` for a given type.
/// Unless you are modifying the implementation of this derive, or attempting to
/// manually implement `FromBytes` for a type yourself, you don't need to read
/// this section.*
///
/// If a type has the following properties, then this derive can implement
/// `FromBytes` for that type:
///
/// - If the type is a struct, all of its fields must be `FromBytes`.
/// - If the type is an enum:
///   - It must be a C-like enum (meaning that all variants have no fields).
///   - It must have a defined representation (`repr`s `C`, `u8`, `u16`, `u32`,
///     `u64`, `usize`, `i8`, `i16`, `i32`, `i64`, or `isize`).
///   - The maximum number of discriminants must be used (so that every possible
///     bit pattern is a valid one). Be very careful when using the `C`,
///     `usize`, or `isize` representations, as their size is
///     platform-dependent.
///
/// This analysis is subject to change. Unsafe code may *only* rely on the
/// documented [safety conditions] of `FromBytes`, and must *not* rely on the
/// implementation details of this derive.
///
/// ## Why isn't an explicit representation required for structs?
///
/// Neither this derive, nor the [safety conditions] of `FromBytes`, requires
/// that structs are marked with `#[repr(C)]`.
///
/// Per the [Rust reference](reference),
///
/// > The representation of a type can change the padding between fields, but
/// does not change the layout of the fields themselves.
///
/// [reference]: https://doc.rust-lang.org/reference/type-layout.html#representations
///
/// Since the layout of structs only consists of padding bytes and field bytes,
/// a struct is soundly `FromBytes` if:
/// 1. its padding is soundly `FromBytes`, and
/// 2. its fields are soundly `FromBytes`.
///
/// The answer to the first question is always yes: padding bytes do not have
/// any validity constraints. A [discussion] of this question in the Unsafe Code
/// Guidelines Working Group concluded that it would be virtually unimaginable
/// for future versions of rustc to add validity constraints to padding bytes.
///
/// [discussion]: https://github.com/rust-lang/unsafe-code-guidelines/issues/174
///
/// Whether a struct is soundly `FromBytes` therefore solely depends on whether
/// its fields are `FromBytes`.
// TODO(#146): Document why we don't require an enum to have an explicit `repr`
// attribute.
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::FromBytes;

/// Types for which any bit pattern is valid.
///
/// Any memory region of the appropriate length which contains initialized bytes
/// can be viewed as any `FromBytes` type with no runtime overhead. This is
/// useful for efficiently parsing bytes as structured data.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(FromBytes)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{FromBytes, Immutable};
/// #[derive(FromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V0A, V0B, V0C, V0D, V0E,
/// #   V0F, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V1A, V1B, V1C, V1D,
/// #   V1E, V1F, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V2A, V2B, V2C,
/// #   V2D, V2E, V2F, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V3A, V3B,
/// #   V3C, V3D, V3E, V3F, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V4A,
/// #   V4B, V4C, V4D, V4E, V4F, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59,
/// #   V5A, V5B, V5C, V5D, V5E, V5F, V60, V61, V62, V63, V64, V65, V66, V67, V68,
/// #   V69, V6A, V6B, V6C, V6D, V6E, V6F, V70, V71, V72, V73, V74, V75, V76, V77,
/// #   V78, V79, V7A, V7B, V7C, V7D, V7E, V7F, V80, V81, V82, V83, V84, V85, V86,
/// #   V87, V88, V89, V8A, V8B, V8C, V8D, V8E, V8F, V90, V91, V92, V93, V94, V95,
/// #   V96, V97, V98, V99, V9A, V9B, V9C, V9D, V9E, V9F, VA0, VA1, VA2, VA3, VA4,
/// #   VA5, VA6, VA7, VA8, VA9, VAA, VAB, VAC, VAD, VAE, VAF, VB0, VB1, VB2, VB3,
/// #   VB4, VB5, VB6, VB7, VB8, VB9, VBA, VBB, VBC, VBD, VBE, VBF, VC0, VC1, VC2,
/// #   VC3, VC4, VC5, VC6, VC7, VC8, VC9, VCA, VCB, VCC, VCD, VCE, VCF, VD0, VD1,
/// #   VD2, VD3, VD4, VD5, VD6, VD7, VD8, VD9, VDA, VDB, VDC, VDD, VDE, VDF, VE0,
/// #   VE1, VE2, VE3, VE4, VE5, VE6, VE7, VE8, VE9, VEA, VEB, VEC, VED, VEE, VEF,
/// #   VF0, VF1, VF2, VF3, VF4, VF5, VF6, VF7, VF8, VF9, VFA, VFB, VFC, VFD, VFE,
/// #   VFF,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `FromBytes`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: FromBytes`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `FromBytes` manually, and you don't plan on writing unsafe code that
/// operates on `FromBytes` types, then you don't need to read this section.*
///
/// If `T: FromBytes`, then unsafe code may assume that it is sound to produce a
/// `T` whose bytes are initialized to any sequence of valid `u8`s (in other
/// words, any byte value which is not uninitialized). If a type is marked as
/// `FromBytes` which violates this contract, it may cause undefined behavior.
///
/// `#[derive(FromBytes)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::FromBytes",
    doc = "[derive-analysis]: zerocopy_derive::FromBytes#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromBytes.html"),
    doc = concat!("[derive-analysis]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromBytes.html#analysis"),
)]
pub unsafe trait FromBytes: FromZeros {
    // The `Self: Sized` bound makes it so that `FromBytes` is still object
    // safe.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;

    /// Interprets the given `bytes` as a `&Self` without copying.
    ///
    /// If `bytes.len()` does not correspond to a valid length for `Self`, or if
    /// `bytes` is not aligned to `Self`'s alignment requirement, this returns
    /// `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     header: PacketHeader,
    ///     body: [u8],
    /// }
    ///
    /// // These bytes encode a `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11][..];
    ///
    /// let packet = Packet::ref_from(bytes).unwrap();
    ///
    /// assert_eq!(packet.header.src_port, [0, 1]);
    /// assert_eq!(packet.header.dst_port, [2, 3]);
    /// assert_eq!(packet.header.length, [4, 5]);
    /// assert_eq!(packet.header.checksum, [6, 7]);
    /// assert_eq!(packet.body, [8, 9, 10, 11]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from(bytes: &[u8]) -> Result<&Self, CastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        util::assert_dst_is_not_zst::<Self>();
        match Ptr::from_ref(bytes).try_cast_into_no_leftover::<_, BecauseImmutable>(None) {
            Ok(ptr) => Ok(ptr.bikeshed_recall_valid().as_ref()),
            Err(err) => Err(err.map_src(|src| src.as_ref())),
        }
    }

    /// Interprets the prefix of the given `bytes` as a `&Self` without copying.
    ///
    /// This method returns both a reference to the first `size_of::<Self>()`
    /// bytes of `bytes` interpreted as a `Self`, and a reference to the remaining
    /// bytes. If `bytes.len() < size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`ref_from_prefix_with_trailing_elements`],
    /// which does support such types. Attempting to use this method on such
    /// types results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from_prefix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// [`ref_from_prefix_with_trailing_elements`]: FromBytes::ref_from_prefix_with_trailing_elements
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     header: PacketHeader,
    ///     body: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14][..];
    ///
    /// let (packet, excess) = Packet::ref_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.header.src_port, [0, 1]);
    /// assert_eq!(packet.header.dst_port, [2, 3]);
    /// assert_eq!(packet.header.length, [4, 5]);
    /// assert_eq!(packet.header.checksum, [6, 7]);
    /// assert_eq!(packet.body, [[8, 9], [10, 11], [12, 13]]);
    /// assert_eq!(excess, &[14u8][..]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_prefix(bytes: &[u8]) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        util::assert_dst_is_not_zst::<Self>();
        ref_from_prefix_suffix(bytes, None, CastType::Prefix)
    }

    /// Interprets the suffix of the given `bytes` as a `&Self` without copying.
    ///
    /// This method returns both a reference to the last `size_of::<Self>()`
    /// bytes of `bytes` interpreted as a `Self`, and a reference to the preceding
    /// bytes. If `bytes.len() < size_of::<Self>()` or the suffix of `bytes` is
    /// not aligned to `align_of::<Self>()`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`ref_from_suffix_with_trailing_elements`],
    /// which does support such types. Attempting to use this method on such
    /// types results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from_suffix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// [`ref_from_suffix_with_trailing_elements`]: FromBytes::ref_from_suffix_with_trailing_elements
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, trailer) = PacketTrailer::ref_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(prefix, &[0, 1, 2, 3, 4, 5][..]);
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_suffix(bytes: &[u8]) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
    where
        Self: Immutable + KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        ref_from_prefix_suffix(bytes, None, CastType::Suffix).map(swap)
    }

    /// Interprets the given `bytes` as a `&mut Self` without copying.
    ///
    /// If `bytes.len() != size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`mut_from_prefix_with_trailing_elements`],
    /// which does support such types. Attempting to use this method on such
    /// types results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// [`mut_from_prefix_with_trailing_elements`]: FromBytes::mut_from_prefix_with_trailing_elements
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These bytes encode a `PacketHeader`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let header = PacketHeader::mut_from(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    ///
    /// header.checksum = [0, 0];
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 0, 0]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from(bytes: &mut [u8]) -> Result<&mut Self, CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        match Ptr::from_mut(bytes).try_cast_into_no_leftover::<_, BecauseExclusive>(None) {
            Ok(ptr) => Ok(ptr.bikeshed_recall_valid().as_mut()),
            Err(err) => Err(err.map_src(|src| src.as_mut())),
        }
    }

    /// Interprets the prefix of the given `bytes` as a `&mut Self` without
    /// copying.
    ///
    /// This method returns both a reference to the first `size_of::<Self>()`
    /// bytes of `bytes` interpreted as a `Self`, and a reference to the remaining
    /// bytes. If `bytes.len() < size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`mut_from_suffix_with_trailing_elements`],
    /// which does support such types. Attempting to use this method on such
    /// types results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from_prefix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// [`mut_from_suffix_with_trailing_elements`]: FromBytes::mut_from_suffix_with_trailing_elements
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketHeader`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (header, body) = PacketHeader::mut_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// assert_eq!(body, &[8, 9][..]);
    ///
    /// header.checksum = [0, 0];
    /// body.fill(1);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 0, 0, 1, 1]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_prefix(
        bytes: &mut [u8],
    ) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        mut_from_prefix_suffix(bytes, None, CastType::Prefix)
    }

    /// Interprets the suffix of the given `bytes` as a `&mut Self` without
    /// copying.
    ///
    /// This method returns both a reference to the last `size_of::<Self>()`
    /// bytes of `bytes` interpreted as a `Self`, and a reference to the preceding
    /// bytes. If `bytes.len() < size_of::<Self>()` or the suffix of `bytes` is
    /// not aligned to `align_of::<Self>()`, this returns `Err`.
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from_suffix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, trailer) = PacketTrailer::mut_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(prefix, &[0u8, 1, 2, 3, 4, 5][..]);
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    ///
    /// prefix.fill(0);
    /// trailer.frame_check_sequence.fill(1);
    ///
    /// assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_suffix(
        bytes: &mut [u8],
    ) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        util::assert_dst_is_not_zst::<Self>();
        mut_from_prefix_suffix(bytes, None, CastType::Suffix).map(swap)
    }

    /// Interprets the given `bytes` as a `&Self` with a DST length equal to
    /// `count`.
    ///
    /// This method verifies that `bytes.len() == size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let pixels = <[Pixel]>::ref_from_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let zsty = ZSTy::ref_from_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from`]: FromBytes::ref_from
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_with_trailing_elements(
        bytes: &[u8],
        count: usize,
    ) -> Result<&Self, CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        let bytes = Ptr::from_ref(bytes);
        let maybe_slf = bytes.try_cast_into_no_leftover::<_, BecauseImmutable>(Some(count));
        match maybe_slf {
            Ok(slf) => Ok(slf.bikeshed_recall_valid().as_ref()),
            Err(err) => Err(err.map_src(|s| s.as_ref())),
        }
    }

    /// Interprets the prefix of the given `bytes` as a `&[Self]` with length
    /// equal to `count` without copying.
    ///
    /// This method verifies that `bytes.len() >= size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It reinterprets the first
    /// `size_of::<T>() * count` bytes from `bytes` to construct a `&[Self]`,
    /// and returns the remaining bytes to the caller. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (pixels, rest) = <[Pixel]>::ref_from_prefix_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// assert_eq!(rest, &[8, 9]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let (zsty, _) = ZSTy::ref_from_prefix_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from_prefix`]: FromBytes::ref_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_prefix_with_trailing_elements(
        bytes: &[u8],
        count: usize,
    ) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        ref_from_prefix_suffix(bytes, Some(count), CastType::Prefix)
    }

    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::from_prefix_with_trailing_elements`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline]
    fn slice_from_prefix(bytes: &[u8], count: usize) -> Option<(&[Self], &[u8])>
    where
        Self: Sized + Immutable,
    {
        <[Self]>::ref_from_prefix_with_trailing_elements(bytes, count).ok()
    }

    /// Interprets the suffix of the given `bytes` as a `&[Self]` with length
    /// equal to `count` without copying.
    ///
    /// This method verifies that `bytes.len() >= size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It reinterprets the last
    /// `size_of::<T>() * count` bytes from `bytes` to construct a `&[Self]`,
    /// and returns the preceding bytes to the caller. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (rest, pixels) = <[Pixel]>::ref_from_suffix_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(rest, &[0, 1]);
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 2, g: 3, b: 4, a: 5 },
    ///     Pixel { r: 6, g: 7, b: 8, a: 9 },
    /// ]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from_suffix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let (_, zsty) = ZSTy::ref_from_suffix_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from_suffix`]: FromBytes::ref_from_suffix
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_suffix_with_trailing_elements(
        bytes: &[u8],
        count: usize,
    ) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        ref_from_prefix_suffix(bytes, Some(count), CastType::Suffix).map(swap)
    }

    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::from_prefix_with_trailing_elements`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline]
    fn slice_from_suffix(bytes: &[u8], count: usize) -> Option<(&[u8], &[Self])>
    where
        Self: Sized + Immutable,
    {
        <[Self]>::ref_from_suffix_with_trailing_elements(bytes, count).ok()
    }

    #[deprecated(since = "0.8.0", note = "`FromBytes::mut_from` now supports slices")]
    #[must_use = "has no side effects"]
    #[doc(hidden)]
    #[inline]
    fn mut_slice_from(bytes: &mut [u8]) -> Option<&mut [Self]>
    where
        Self: Sized + IntoBytes,
    {
        <[Self]>::mut_from(bytes).ok()
    }

    /// Interprets the given `bytes` as a `&mut Self` with a DST length equal to
    /// `count`.
    ///
    /// This method verifies that `bytes.len() == size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let pixels = <[Pixel]>::mut_from_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 0, 0, 0, 0]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from`] which
    /// do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let zsty = ZSTy::mut_from_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from`]: FromBytes::mut_from
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_with_trailing_elements(
        bytes: &mut [u8],
        count: usize,
    ) -> Result<&mut Self, CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize> + Immutable,
    {
        let bytes = Ptr::from_mut(bytes);
        let maybe_slf = bytes.try_cast_into_no_leftover::<_, BecauseImmutable>(Some(count));
        match maybe_slf {
            Ok(slf) => Ok(slf.bikeshed_recall_valid().as_mut()),
            Err(err) => Err(err.map_src(|s| s.as_mut())),
        }
    }

    /// Interprets the prefix of the given `bytes` as a `&mut [Self]` with
    /// length equal to `count` without copying.
    ///
    /// This method verifies that `bytes.len() >= size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It reinterprets the first
    /// `size_of::<T>() * count` bytes from `bytes` to construct a `&[Self]`,
    /// and returns the remaining bytes to the caller. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (pixels, rest) = <[Pixel]>::mut_from_prefix_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// assert_eq!(rest, &[8, 9]);
    ///
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    /// rest.fill(1);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 0, 0, 0, 0, 1, 1]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let (zsty, _) = ZSTy::mut_from_prefix_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from_prefix`]: FromBytes::mut_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_prefix_with_trailing_elements(
        bytes: &mut [u8],
        count: usize,
    ) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize>,
    {
        mut_from_prefix_suffix(bytes, Some(count), CastType::Prefix)
    }

    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::mut_from_prefix_with_trailing_elements`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_slice_from_prefix(bytes: &mut [u8], count: usize) -> Option<(&mut [Self], &mut [u8])>
    where
        Self: Sized + IntoBytes,
    {
        <[Self]>::mut_from_prefix_with_trailing_elements(bytes, count).ok()
    }

    /// Interprets the suffix of the given `bytes` as a `&mut [Self]` with length
    /// equal to `count` without copying.
    ///
    /// This method verifies that `bytes.len() >= size_of::<T>() * count` and
    /// that `bytes` is aligned to `align_of::<T>()`. It reinterprets the last
    /// `size_of::<T>() * count` bytes from `bytes` to construct a `&[Self]`,
    /// and returns the preceding bytes to the caller. It also ensures that
    /// `sizeof::<T>() * count` does not overflow a `usize`. If any of the
    /// length, alignment, or overflow checks fail, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (rest, pixels) = <[Pixel]>::mut_from_suffix_with_trailing_elements(bytes, 2).unwrap();
    ///
    /// assert_eq!(rest, &[0, 1]);
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 2, g: 3, b: 4, a: 5 },
    ///     Pixel { r: 6, g: 7, b: 8, a: 9 },
    /// ]);
    ///
    /// rest.fill(9);
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    ///
    /// assert_eq!(bytes, [9, 9, 2, 3, 4, 5, 0, 0, 0, 0]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from_suffix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let (_, zsty) = ZSTy::mut_from_suffix_with_trailing_elements(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from_suffix`]: FromBytes::mut_from_suffix
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_suffix_with_trailing_elements(
        bytes: &mut [u8],
        count: usize,
    ) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize>,
    {
        mut_from_prefix_suffix(bytes, Some(count), CastType::Suffix).map(swap)
    }

    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::mut_from_suffix_with_trailing_elements`"
    )]
    #[doc(hidden)]
    #[inline]
    fn mut_slice_from_suffix(bytes: &mut [u8], count: usize) -> Option<(&mut [u8], &mut [Self])>
    where
        Self: Sized + IntoBytes,
    {
        <[Self]>::mut_from_suffix_with_trailing_elements(bytes, count).ok()
    }

    /// Reads a copy of `Self` from `bytes`.
    ///
    /// If `bytes.len() != size_of::<Self>()`, `read_from` returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These bytes encode a `PacketHeader`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let header = PacketHeader::read_from(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from(bytes: &[u8]) -> Result<Self, SizeError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from(bytes) {
            Ok(r) => Ok(r.read().into_inner()),
            Err(CastError::Size(e)) => Err(e.with_dst()),
            Err(CastError::Alignment(_)) => unreachable!(),
            Err(CastError::Validity(i)) => match i {},
        }
    }

    /// Reads a copy of `Self` from the prefix of `bytes`.
    ///
    /// `read_from_prefix` reads a `Self` from the first `size_of::<Self>()`
    /// bytes of `bytes`. If `bytes.len() < size_of::<Self>()`, it returns
    /// `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketHeader`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let header = PacketHeader::read_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from_prefix(bytes: &[u8]) -> Result<Self, SizeError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from_prefix(bytes) {
            Ok((r, _)) => Ok(r.read().into_inner()),
            Err(CastError::Size(e)) => Err(e.with_dst()),
            Err(CastError::Alignment(_)) => unreachable!(),
            Err(CastError::Validity(i)) => match i {},
        }
    }

    /// Reads a copy of `Self` from the suffix of `bytes`.
    ///
    /// `read_from_suffix` reads a `Self` from the last `size_of::<Self>()`
    /// bytes of `bytes`. If `bytes.len() < size_of::<Self>()`, it returns
    /// `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let trailer = PacketTrailer::read_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from_suffix(bytes: &[u8]) -> Result<Self, CastError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from_suffix(bytes) {
            Ok((_, r)) => Ok(r.read().into_inner()),
            Err(CastError::Size(e)) => Err(CastError::Size(e.with_dst())),
            Err(CastError::Alignment(_)) => unreachable!(),
            Err(CastError::Validity(i)) => match i {},
        }
    }

    #[deprecated(since = "0.8.0", note = "`FromBytes::ref_from` now supports slices")]
    #[allow(clippy::must_use_candidate)]
    #[doc(hidden)]
    #[inline]
    fn slice_from(bytes: &[u8]) -> Option<&[Self]>
    where
        Self: Sized + Immutable,
    {
        <[Self]>::ref_from(bytes).ok()
    }
}

#[inline(always)]
fn ref_from_prefix_suffix<T: FromBytes + KnownLayout + Immutable + ?Sized>(
    bytes: &[u8],
    meta: Option<T::PointerMetadata>,
    cast_type: CastType,
) -> Result<(&T, &[u8]), CastError<&[u8], T>> {
    let (slf, prefix_suffix) = Ptr::from_ref(bytes)
        .try_cast_into::<_, BecauseImmutable>(cast_type, meta)
        .map_err(|err| err.map_src(|s| s.as_ref()))?;
    Ok((slf.bikeshed_recall_valid().as_ref(), prefix_suffix.as_ref()))
}

#[inline(always)]
fn mut_from_prefix_suffix<T: FromBytes + KnownLayout + ?Sized>(
    bytes: &mut [u8],
    meta: Option<T::PointerMetadata>,
    cast_type: CastType,
) -> Result<(&mut T, &mut [u8]), CastError<&mut [u8], T>> {
    let (slf, prefix_suffix) = Ptr::from_mut(bytes)
        .try_cast_into::<_, BecauseExclusive>(cast_type, meta)
        .map_err(|err| err.map_src(|s| s.as_mut()))?;
    Ok((slf.bikeshed_recall_valid().as_mut(), prefix_suffix.as_mut()))
}

/// Analyzes whether a type is [`IntoBytes`].
///
/// This derive analyzes, at compile time, whether the annotated type satisfies
/// the [safety conditions] of `IntoBytes` and implements `IntoBytes` if it is
/// sound to do so. This derive can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::{IntoBytes};
/// #[derive(IntoBytes)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(IntoBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(IntoBytes)]
/// #[repr(C)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// [safety conditions]: trait@IntoBytes#safety
///
/// # Error Messages
///
/// Due to the way that the custom derive for `IntoBytes` is implemented, you
/// may get an error like this:
///
/// ```text
/// error[E0277]: the trait bound `HasPadding<Foo, true>: ShouldBe<false>` is not satisfied
///   --> lib.rs:23:10
///    |
///  1 | #[derive(IntoBytes)]
///    |          ^^^^^^^ the trait `ShouldBe<false>` is not implemented for `HasPadding<Foo, true>`
///    |
///    = help: the trait `ShouldBe<VALUE>` is implemented for `HasPadding<T, VALUE>`
/// ```
///
/// This error indicates that the type being annotated has padding bytes, which
/// is illegal for `IntoBytes` types. Consider reducing the alignment of some
/// fields by using types in the [`byteorder`] module, adding explicit struct
/// fields where those padding bytes would be, or using `#[repr(packed)]`. See
/// the Rust Reference's page on [type layout] for more information about type
/// layout and padding.
///
/// [type layout]: https://doc.rust-lang.org/reference/type-layout.html
///
/// # Analysis
///
/// *This section describes, roughly, the analysis performed by this derive to
/// determine whether it is sound to implement `IntoBytes` for a given type.
/// Unless you are modifying the implementation of this derive, or attempting to
/// manually implement `IntoBytes` for a type yourself, you don't need to read
/// this section.*
///
/// If a type has the following properties, then this derive can implement
/// `IntoBytes` for that type:
///
/// - If the type is a struct:
///   - It must have a defined representation (`repr(C)`, `repr(transparent)`,
///     or `repr(packed)`).
///   - All of its fields must be `IntoBytes`.
///   - Its layout must have no padding. This is always true for
///     `repr(transparent)` and `repr(packed)`. For `repr(C)`, see the layout
///     algorithm described in the [Rust Reference].
/// - If the type is an enum:
///   - It must be a C-like enum (meaning that all variants have no fields).
///   - It must have a defined representation (`repr`s `C`, `u8`, `u16`, `u32`,
///     `u64`, `usize`, `i8`, `i16`, `i32`, `i64`, or `isize`).
///
/// This analysis is subject to change. Unsafe code may *only* rely on the
/// documented [safety conditions] of `FromBytes`, and must *not* rely on the
/// implementation details of this derive.
///
/// [Rust Reference]: https://doc.rust-lang.org/reference/type-layout.html
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::IntoBytes;

/// Types that can be converted to an immutable slice of initialized bytes.
///
/// Any `IntoBytes` type can be converted to a slice of initialized bytes of the
/// same size. This is useful for efficiently serializing structured data as raw
/// bytes.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(IntoBytes)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::IntoBytes;
/// #[derive(IntoBytes)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(IntoBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(IntoBytes)]
/// #[repr(C)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `IntoBytes`. See the [derive
/// documentation][derive] for guidance on how to interpret error messages
/// produced by the derive's analysis.
///
/// # Safety
///
/// *This section describes what is required in order for `T: IntoBytes`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `IntoBytes` manually, and you don't plan on writing unsafe code that
/// operates on `IntoBytes` types, then you don't need to read this section.*
///
/// If `T: IntoBytes`, then unsafe code may assume that it is sound to treat any
/// `t: T` as an immutable `[u8]` of length `size_of_val(t)`. If a type is
/// marked as `IntoBytes` which violates this contract, it may cause undefined
/// behavior.
///
/// `#[derive(IntoBytes)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::IntoBytes",
    doc = "[derive-analysis]: zerocopy_derive::IntoBytes#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.IntoBytes.html"),
    doc = concat!("[derive-analysis]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.IntoBytes.html#analysis"),
)]
pub unsafe trait IntoBytes {
    // The `Self: Sized` bound makes it so that this function doesn't prevent
    // `IntoBytes` from being object safe. Note that other `IntoBytes` methods
    // prevent object safety, but those provide a benefit in exchange for object
    // safety. If at some point we remove those methods, change their type
    // signatures, or move them out of this trait so that `IntoBytes` is object
    // safe again, it's important that this function not prevent object safety.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;

    /// Gets the bytes of this value.
    ///
    /// `as_bytes` provides access to the bytes of this value as an immutable
    /// byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let bytes = header.as_bytes();
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn as_bytes(&self) -> &[u8]
    where
        Self: Immutable,
    {
        // Note that this method does not have a `Self: Sized` bound;
        // `size_of_val` works for unsized values too.
        let len = mem::size_of_val(self);
        let slf: *const Self = self;

        // SAFETY:
        // - `slf.cast::<u8>()` is valid for reads for `len *
        //   mem::size_of::<u8>()` many bytes because...
        //   - `slf` is the same pointer as `self`, and `self` is a reference
        //     which points to an object whose size is `len`. Thus...
        //     - The entire region of `len` bytes starting at `slf` is contained
        //       within a single allocation.
        //     - `slf` is non-null.
        //   - `slf` is trivially aligned to `align_of::<u8>() == 1`.
        // - `Self: IntoBytes` ensures that all of the bytes of `slf` are
        //   initialized.
        // - Since `slf` is derived from `self`, and `self` is an immutable
        //   reference, the only other references to this memory region that
        //   could exist are other immutable references, and those don't allow
        //   mutation. `Self: Immutable` prohibits types which contain
        //   `UnsafeCell`s, which are the only types for which this rule
        //   wouldn't be sufficient.
        // - The total size of the resulting slice is no larger than
        //   `isize::MAX` because no allocation produced by safe code can be
        //   larger than `isize::MAX`.
        //
        // TODO(#429): Add references to docs and quotes.
        unsafe { slice::from_raw_parts(slf.cast::<u8>(), len) }
    }

    /// Gets the bytes of this value mutably.
    ///
    /// `as_mut_bytes` provides access to the bytes of this value as a mutable
    /// byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Eq, PartialEq, Debug)]
    /// #[derive(FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let mut header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let bytes = header.as_mut_bytes();
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    ///
    /// bytes.reverse();
    ///
    /// assert_eq!(header, PacketHeader {
    ///     src_port: [7, 6],
    ///     dst_port: [5, 4],
    ///     length: [3, 2],
    ///     checksum: [1, 0],
    /// });
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn as_mut_bytes(&mut self) -> &mut [u8]
    where
        Self: FromBytes,
    {
        // Note that this method does not have a `Self: Sized` bound;
        // `size_of_val` works for unsized values too.
        let len = mem::size_of_val(self);
        let slf: *mut Self = self;

        // SAFETY:
        // - `slf.cast::<u8>()` is valid for reads and writes for `len *
        //   mem::size_of::<u8>()` many bytes because...
        //   - `slf` is the same pointer as `self`, and `self` is a reference
        //     which points to an object whose size is `len`. Thus...
        //     - The entire region of `len` bytes starting at `slf` is contained
        //       within a single allocation.
        //     - `slf` is non-null.
        //   - `slf` is trivially aligned to `align_of::<u8>() == 1`.
        // - `Self: IntoBytes` ensures that all of the bytes of `slf` are
        //   initialized.
        // - `Self: FromBytes` ensures that no write to this memory region
        //   could result in it containing an invalid `Self`.
        // - Since `slf` is derived from `self`, and `self` is a mutable
        //   reference, no other references to this memory region can exist.
        // - The total size of the resulting slice is no larger than
        //   `isize::MAX` because no allocation produced by safe code can be
        //   larger than `isize::MAX`.
        //
        // TODO(#429): Add references to docs and quotes.
        unsafe { slice::from_raw_parts_mut(slf.cast::<u8>(), len) }
    }

    /// Writes a copy of `self` to `bytes`.
    ///
    /// If `bytes.len() != size_of_val(self)`, `write_to` returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    ///
    /// If too many or too few target bytes are provided, `write_to` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut excessive_bytes = &mut [0u8; 128][..];
    ///
    /// let write_result = header.write_to(excessive_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(excessive_bytes, [0u8; 128]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    fn write_to(&self, bytes: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        if bytes.len() != mem::size_of_val(self) {
            return Err(SizeError::new(self));
        }
        bytes.copy_from_slice(self.as_bytes());
        Ok(())
    }

    /// Writes a copy of `self` to the prefix of `bytes`.
    ///
    /// `write_to_prefix` writes `self` to the first `size_of_val(self)` bytes
    /// of `bytes`. If `bytes.len() < size_of_val(self)`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to_prefix(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7, 0, 0]);
    /// ```
    ///
    /// If insufficient target bytes are provided, `write_to_prefix` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut insufficent_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficent_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficent_bytes, [0, 0]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    fn write_to_prefix(&self, bytes: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        let size = mem::size_of_val(self);
        match bytes.get_mut(..size) {
            Some(bytes) => {
                bytes.copy_from_slice(self.as_bytes());
                Ok(())
            }
            None => Err(SizeError::new(self)),
        }
    }

    /// Writes a copy of `self` to the suffix of `bytes`.
    ///
    /// `write_to_suffix` writes `self` to the last `size_of_val(self)` bytes of
    /// `bytes`. If `bytes.len() < size_of_val(self)`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to_suffix(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 0, 0, 1, 2, 3, 4, 5, 6, 7]);
    ///
    /// let mut insufficent_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficent_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficent_bytes, [0, 0]);
    /// ```
    ///
    /// If insufficient target bytes are provided, `write_to_suffix` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut insufficent_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficent_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficent_bytes, [0, 0]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    fn write_to_suffix(&self, bytes: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        let start = if let Some(start) = bytes.len().checked_sub(mem::size_of_val(self)) {
            start
        } else {
            return Err(SizeError::new(self));
        };
        let bytes = if let Some(bytes) = bytes.get_mut(start..) {
            bytes
        } else {
            // get_mut() should never return None here. We return a `SizeError`
            // rather than .unwrap() because in the event the branch is not
            // optimized away, returning a value is generally lighter-weight
            // than panicking.
            return Err(SizeError::new(self));
        };
        bytes.copy_from_slice(self.as_bytes());
        Ok(())
    }

    #[deprecated(since = "0.8.0", note = "`IntoBytes::as_bytes_mut` was renamed to `as_mut_bytes`")]
    #[doc(hidden)]
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8]
    where
        Self: FromBytes,
    {
        self.as_mut_bytes()
    }
}

/// Analyzes whether a type is [`Unaligned`].
///
/// This derive analyzes, at compile time, whether the annotated type satisfies
/// the [safety conditions] of `Unaligned` and implements `Unaligned` if it is
/// sound to do so. This derive can be applied to structs, enums, and unions;
/// e.g.:
///
/// ```
/// # use zerocopy_derive::Unaligned;
/// #[derive(Unaligned)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(packed)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// # Analysis
///
/// *This section describes, roughly, the analysis performed by this derive to
/// determine whether it is sound to implement `Unaligned` for a given type.
/// Unless you are modifying the implementation of this derive, or attempting to
/// manually implement `Unaligned` for a type yourself, you don't need to read
/// this section.*
///
/// If a type has the following properties, then this derive can implement
/// `Unaligned` for that type:
///
/// - If the type is a struct or union:
///   - If `repr(align(N))` is provided, `N` must equal 1.
///   - If the type is `repr(C)` or `repr(transparent)`, all fields must be
///     [`Unaligned`].
///   - If the type is not `repr(C)` or `repr(transparent)`, it must be
///     `repr(packed)` or `repr(packed(1))`.
/// - If the type is an enum:
///   - If `repr(align(N))` is provided, `N` must equal 1.
///   - It must be a C-like enum (meaning that all variants have no fields).
///   - It must be `repr(i8)` or `repr(u8)`.
///
/// [safety conditions]: trait@Unaligned#safety
#[cfg(any(feature = "derive", test))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use zerocopy_derive::Unaligned;

/// Types with no alignment requirement.
///
/// If `T: Unaligned`, then `align_of::<T>() == 1`.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(Unaligned)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::Unaligned;
/// #[derive(Unaligned)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(packed)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `Unaligned`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: Unaligned`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `Unaligned` manually, and you don't plan on writing unsafe code that
/// operates on `Unaligned` types, then you don't need to read this section.*
///
/// If `T: Unaligned`, then unsafe code may assume that it is sound to produce a
/// reference to `T` at any memory location regardless of alignment. If a type
/// is marked as `Unaligned` which violates this contract, it may cause
/// undefined behavior.
///
/// `#[derive(Unaligned)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::Unaligned",
    doc = "[derive-analysis]: zerocopy_derive::Unaligned#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Unaligned.html"),
    doc = concat!("[derive-analysis]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Unaligned.html#analysis"),
)]
pub unsafe trait Unaligned {
    // The `Self: Sized` bound makes it so that `Unaligned` is still object
    // safe.
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
}

safety_comment! {
    /// SAFETY:
    /// Per the reference [1], "the unit tuple (`()`) ... is guaranteed as a
    /// zero-sized type to have a size of 0 and an alignment of 1."
    /// - `Immutable`: `()` self-evidently does not contain any `UnsafeCell`s.
    /// - `TryFromBytes` (with no validator), `FromZeros`, `FromBytes`: There is
    ///   only one possible sequence of 0 bytes, and `()` is inhabited.
    /// - `IntoBytes`: Since `()` has size 0, it contains no padding bytes.
    /// - `Unaligned`: `()` has alignment 1.
    ///
    /// [1] https://doc.rust-lang.org/reference/type-layout.html#tuple-layout
    unsafe_impl!((): Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
    assert_unaligned!(());
}

safety_comment! {
    /// SAFETY:
    /// - `Immutable`: These types self-evidently do not contain any
    ///   `UnsafeCell`s.
    /// - `TryFromBytes` (with no validator), `FromZeros`, `FromBytes`: all bit
    ///   patterns are valid for numeric types [1]
    /// - `IntoBytes`: numeric types have no padding bytes [1]
    /// - `Unaligned` (`u8` and `i8` only): The reference [2] specifies the size
    ///   of `u8` and `i8` as 1 byte. We also know that:
    ///   - Alignment is >= 1 [3]
    ///   - Size is an integer multiple of alignment [4]
    ///   - The only value >= 1 for which 1 is an integer multiple is 1
    ///   Therefore, the only possible alignment for `u8` and `i8` is 1.
    ///
    /// [1] Per https://doc.rust-lang.org/beta/reference/types/numeric.html#bit-validity:
    ///
    ///     For every numeric type, `T`, the bit validity of `T` is equivalent to
    ///     the bit validity of `[u8; size_of::<T>()]`. An uninitialized byte is
    ///     not a valid `u8`.
    ///
    /// TODO(https://github.com/rust-lang/reference/pull/1392): Once this text
    /// is available on the Stable docs, cite those instead.
    ///
    /// [2] https://doc.rust-lang.org/reference/type-layout.html#primitive-data-layout
    ///
    /// [3] Per https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment:
    ///
    ///     Alignment is measured in bytes, and must be at least 1.
    ///
    /// [4] Per https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment:
    ///
    ///     The size of a value is always a multiple of its alignment.
    ///
    /// TODO(#278): Once we've updated the trait docs to refer to `u8`s rather
    /// than bits or bytes, update this comment, especially the reference to
    /// [1].
    unsafe_impl!(u8: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
    unsafe_impl!(i8: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
    assert_unaligned!(u8, i8);
    unsafe_impl!(u16: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(i16: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(u32: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(i32: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(u64: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(i64: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(u128: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(i128: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(usize: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(isize: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(f32: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(f64: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes);
}

safety_comment! {
    /// SAFETY:
    /// - `Immutable`: `bool` self-evidently does not contain any `UnsafeCell`s.
    /// - `FromZeros`: Valid since "[t]he value false has the bit pattern 0x00"
    ///   [1].
    /// - `IntoBytes`: Since "the boolean type has a size and alignment of 1
    ///   each" and "The value false has the bit pattern 0x00 and the value true
    ///   has the bit pattern 0x01" [1]. Thus, the only byte of the bool is
    ///   always initialized.
    /// - `Unaligned`: Per the reference [1], "[a]n object with the boolean type
    ///   has a size and alignment of 1 each."
    ///
    /// [1] https://doc.rust-lang.org/reference/types/boolean.html
    unsafe_impl!(bool: Immutable, FromZeros, IntoBytes, Unaligned);
    assert_unaligned!(bool);
    /// SAFETY:
    /// - The safety requirements for `unsafe_impl!` with an `is_bit_valid`
    ///   closure:
    ///   - Given `t: *mut bool` and `let r = *mut u8`, `r` refers to an object
    ///     of the same size as that referred to by `t`. This is true because
    ///     `bool` and `u8` have the same size (1 byte) [1]. Neither `r` nor `t`
    ///     contain `UnsafeCell`s because neither `bool` nor `u8` do [4].
    ///   - Since the closure takes a `&u8` argument, given a `Maybe<'a,
    ///     bool>` which satisfies the preconditions of
    ///     `TryFromBytes::<bool>::is_bit_valid`, it must be guaranteed that the
    ///     memory referenced by that `MaybeValid` always contains a valid `u8`.
    ///     Since `bool`'s single byte is always initialized, `is_bit_valid`'s
    ///     precondition requires that the same is true of its argument. Since
    ///     `u8`'s only bit validity invariant is that its single byte must be
    ///     initialized, this memory is guaranteed to contain a valid `u8`.
    ///   - The impl must only return `true` for its argument if the original
    ///     `Maybe<bool>` refers to a valid `bool`. We only return true if
    ///     the `u8` value is 0 or 1, and both of these are valid values for
    ///     `bool`. [3]
    ///
    /// [1] Per https://doc.rust-lang.org/reference/type-layout.html#primitive-data-layout:
    ///
    ///   The size of most primitives is given in this table.
    ///
    ///   | Type      | `size_of::<Type>() ` |
    ///   |-----------|----------------------|
    ///   | `bool`    | 1                    |
    ///   | `u8`/`i8` | 1                    |
    ///
    /// [2] Per https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment:
    ///
    ///   The size of a value is always a multiple of its alignment.
    ///
    /// [3] Per https://doc.rust-lang.org/reference/types/boolean.html:
    ///
    ///   The value false has the bit pattern 0x00 and the value true has the
    ///   bit pattern 0x01.
    ///
    /// [4] TODO(#429): Justify this claim.
    unsafe_impl!(bool: TryFromBytes; |byte: MaybeAligned<u8>| *byte.unaligned_as_ref() < 2);
}
safety_comment! {
    /// SAFETY:
    /// - `Immutable`: `char` self-evidently does not contain any `UnsafeCell`s.
    /// - `FromZeros`: Per reference [1], "[a] value of type char is a Unicode
    ///   scalar value (i.e. a code point that is not a surrogate), represented
    ///   as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to
    ///   0x10FFFF range" which contains 0x0000.
    /// - `IntoBytes`: `char` is per reference [1] "represented as a 32-bit
    ///   unsigned word" (`u32`) which is `IntoBytes`. Note that unlike `u32`,
    ///   not all bit patterns are valid for `char`.
    ///
    /// [1] https://doc.rust-lang.org/reference/types/textual.html
    unsafe_impl!(char: Immutable, FromZeros, IntoBytes);
    /// SAFETY:
    /// - The safety requirements for `unsafe_impl!` with an `is_bit_valid`
    ///   closure:
    ///   - Given `t: *mut char` and `let r = *mut u32`, `r` refers to an object
    ///     of the same size as that referred to by `t`. This is true because
    ///     `char` and `u32` have the same size [1]. Neither `r` nor `t` contain
    ///     `UnsafeCell`s because neither `char` nor `u32` do [4].
    ///   - Since the closure takes a `&u32` argument, given a `Maybe<'a,
    ///     char>` which satisfies the preconditions of
    ///     `TryFromBytes::<char>::is_bit_valid`, it must be guaranteed that the
    ///     memory referenced by that `MaybeValid` always contains a valid
    ///     `u32`. Since `char`'s bytes are always initialized [2],
    ///     `is_bit_valid`'s precondition requires that the same is true of its
    ///     argument. Since `u32`'s only bit validity invariant is that its
    ///     bytes must be initialized, this memory is guaranteed to contain a
    ///     valid `u32`.
    ///   - The impl must only return `true` for its argument if the original
    ///     `Maybe<char>` refers to a valid `char`. `char::from_u32`
    ///     guarantees that it returns `None` if its input is not a valid
    ///     `char`. [3]
    ///
    /// [1] Per https://doc.rust-lang.org/nightly/reference/types/textual.html#layout-and-bit-validity:
    ///
    ///   `char` is guaranteed to have the same size and alignment as `u32` on
    ///   all platforms.
    ///
    /// [2] Per https://doc.rust-lang.org/core/primitive.char.html#method.from_u32:
    ///
    ///   Every byte of a `char` is guaranteed to be initialized.
    ///
    /// [3] Per https://doc.rust-lang.org/core/primitive.char.html#method.from_u32:
    ///
    ///   `from_u32()` will return `None` if the input is not a valid value for
    ///   a `char`.
    ///
    /// [4] TODO(#429): Justify this claim.
    unsafe_impl!(char: TryFromBytes; |candidate: MaybeAligned<u32>| {
        let candidate = candidate.read_unaligned();
        char::from_u32(candidate).is_some()
    });
}
safety_comment! {
    /// SAFETY:
    /// Per the Reference [1], `str` has the same layout as `[u8]`.
    /// - `Immutable`: `[u8]` does not contain any `UnsafeCell`s.
    /// - `FromZeros`, `IntoBytes`, `Unaligned`: `[u8]` is `FromZeros`,
    ///   `IntoBytes`, and `Unaligned`.
    ///
    /// Note that we don't `assert_unaligned!(str)` because `assert_unaligned!`
    /// uses `align_of`, which only works for `Sized` types.
    ///
    /// TODO(#429):
    /// - Add quotes from documentation.
    /// - Improve safety proof for `FromZeros` and `IntoBytes`; having the same
    ///   layout as `[u8]` isn't sufficient.
    ///
    /// [1] https://doc.rust-lang.org/reference/type-layout.html#str-layout
    unsafe_impl!(str: Immutable, FromZeros, IntoBytes, Unaligned);
    /// SAFETY:
    /// - The safety requirements for `unsafe_impl!` with an `is_bit_valid`
    ///   closure:
    ///   - Given `t: *mut str` and `let r = *mut [u8]`, `r` refers to an object
    ///     of the same size as that referred to by `t`. This is true because
    ///     `str` and `[u8]` have the same representation. [1] Neither `t` nor
    ///     `r` contain `UnsafeCell`s because `[u8]` doesn't, and both `t` and
    ///     `r` have that representation.
    ///   - Since the closure takes a `&[u8]` argument, given a `Maybe<'a,
    ///     str>` which satisfies the preconditions of
    ///     `TryFromBytes::<str>::is_bit_valid`, it must be guaranteed that the
    ///     memory referenced by that `MaybeValid` always contains a valid
    ///     `[u8]`. Since `str`'s bytes are always initialized [1],
    ///     `is_bit_valid`'s precondition requires that the same is true of its
    ///     argument. Since `[u8]`'s only bit validity invariant is that its
    ///     bytes must be initialized, this memory is guaranteed to contain a
    ///     valid `[u8]`.
    ///   - The impl must only return `true` for its argument if the original
    ///     `Maybe<str>` refers to a valid `str`. `str::from_utf8`
    ///     guarantees that it returns `Err` if its input is not a valid `str`.
    ///     [2]
    ///
    /// [1] Per https://doc.rust-lang.org/reference/types/textual.html:
    ///
    ///   A value of type `str` is represented the same was as `[u8]`.
    ///
    /// [2] Per https://doc.rust-lang.org/core/str/fn.from_utf8.html#errors:
    ///
    ///   Returns `Err` if the slice is not UTF-8.
    unsafe_impl!(str: TryFromBytes; |candidate: MaybeAligned<[u8]>| {
        let candidate = candidate.unaligned_as_ref();
        core::str::from_utf8(candidate).is_ok()
    });
}

safety_comment! {
    // `NonZeroXxx` is `IntoBytes`, but not `FromZeros` or `FromBytes`.
    //
    /// SAFETY:
    /// - `IntoBytes`: `NonZeroXxx` has the same layout as its associated
    ///    primitive. Since it is the same size, this guarantees it has no
    ///    padding - integers have no padding, and there's no room for padding
    ///    if it can represent all of the same values except 0.
    /// - `Unaligned`: `NonZeroU8` and `NonZeroI8` document that
    ///   `Option<NonZeroU8>` and `Option<NonZeroI8>` both have size 1. [1] [2]
    ///   This is worded in a way that makes it unclear whether it's meant as a
    ///   guarantee, but given the purpose of those types, it's virtually
    ///   unthinkable that that would ever change. `Option` cannot be smaller
    ///   than its contained type, which implies that, and `NonZeroX8` are of
    ///   size 1 or 0. `NonZeroX8` can represent multiple states, so they cannot
    ///   be 0 bytes, which means that they must be 1 byte. The only valid
    ///   alignment for a 1-byte type is 1.
    ///
    /// TODO(#429):
    /// - Add quotes from documentation.
    /// - Add safety comment for `Immutable`. How can we prove that `NonZeroXxx`
    ///   doesn't contain any `UnsafeCell`s? It's obviously true, but it's not
    ///   clear how we'd prove it short of adding text to the stdlib docs that
    ///   says so explicitly, which likely wouldn't be accepted.
    ///
    /// [1] https://doc.rust-lang.org/stable/std/num/struct.NonZeroU8.html
    /// [2] https://doc.rust-lang.org/stable/std/num/struct.NonZeroI8.html
    /// TODO(https://github.com/rust-lang/rust/pull/104082): Cite documentation
    /// that layout is the same as primitive layout.
    unsafe_impl!(NonZeroU8: Immutable, IntoBytes, Unaligned);
    unsafe_impl!(NonZeroI8: Immutable, IntoBytes, Unaligned);
    assert_unaligned!(NonZeroU8, NonZeroI8);
    unsafe_impl!(NonZeroU16: Immutable, IntoBytes);
    unsafe_impl!(NonZeroI16: Immutable, IntoBytes);
    unsafe_impl!(NonZeroU32: Immutable, IntoBytes);
    unsafe_impl!(NonZeroI32: Immutable, IntoBytes);
    unsafe_impl!(NonZeroU64: Immutable, IntoBytes);
    unsafe_impl!(NonZeroI64: Immutable, IntoBytes);
    unsafe_impl!(NonZeroU128: Immutable, IntoBytes);
    unsafe_impl!(NonZeroI128: Immutable, IntoBytes);
    unsafe_impl!(NonZeroUsize: Immutable, IntoBytes);
    unsafe_impl!(NonZeroIsize: Immutable, IntoBytes);
    /// SAFETY:
    /// - The safety requirements for `unsafe_impl!` with an `is_bit_valid`
    ///   closure:
    ///   - Given `t: *mut NonZeroXxx` and `let r = *mut xxx`, `r` refers to an
    ///     object of the same size as that referred to by `t`. This is true
    ///     because `NonZeroXxx` and `xxx` have the same size. [1] Neither `r`
    ///     nor `t` refer to any `UnsafeCell`s because neither `NonZeroXxx` [2]
    ///     nor `xxx` do.
    ///   - Since the closure takes a `&xxx` argument, given a `Maybe<'a,
    ///     NonZeroXxx>` which satisfies the preconditions of
    ///     `TryFromBytes::<NonZeroXxx>::is_bit_valid`, it must be guaranteed
    ///     that the memory referenced by that `MabyeValid` always contains a
    ///     valid `xxx`. Since `NonZeroXxx`'s bytes are always initialized [1],
    ///     `is_bit_valid`'s precondition requires that the same is true of its
    ///     argument. Since `xxx`'s only bit validity invariant is that its
    ///     bytes must be initialized, this memory is guaranteed to contain a
    ///     valid `xxx`.
    ///   - The impl must only return `true` for its argument if the original
    ///     `Maybe<NonZeroXxx>` refers to a valid `NonZeroXxx`. The only
    ///     `xxx` which is not also a valid `NonZeroXxx` is 0. [1]
    ///
    /// [1] Per https://doc.rust-lang.org/core/num/struct.NonZeroU16.html:
    ///
    ///   `NonZeroU16` is guaranteed to have the same layout and bit validity as
    ///   `u16` with the exception that `0` is not a valid instance.
    ///
    /// [2] TODO(#896): Write a safety proof for this before the next stable
    ///     release.
    unsafe_impl!(NonZeroU8: TryFromBytes; |n: MaybeAligned<u8>| NonZeroU8::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroI8: TryFromBytes; |n: MaybeAligned<i8>| NonZeroI8::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroU16: TryFromBytes; |n: MaybeAligned<u16>| NonZeroU16::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroI16: TryFromBytes; |n: MaybeAligned<i16>| NonZeroI16::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroU32: TryFromBytes; |n: MaybeAligned<u32>| NonZeroU32::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroI32: TryFromBytes; |n: MaybeAligned<i32>| NonZeroI32::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroU64: TryFromBytes; |n: MaybeAligned<u64>| NonZeroU64::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroI64: TryFromBytes; |n: MaybeAligned<i64>| NonZeroI64::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroU128: TryFromBytes; |n: MaybeAligned<u128>| NonZeroU128::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroI128: TryFromBytes; |n: MaybeAligned<i128>| NonZeroI128::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroUsize: TryFromBytes; |n: MaybeAligned<usize>| NonZeroUsize::new(n.read_unaligned()).is_some());
    unsafe_impl!(NonZeroIsize: TryFromBytes; |n: MaybeAligned<isize>| NonZeroIsize::new(n.read_unaligned()).is_some());
}
safety_comment! {
    /// SAFETY:
    /// - `TryFromBytes` (with no validator), `FromZeros`, `FromBytes`,
    ///   `IntoBytes`: The Rust compiler reuses `0` value to represent `None`,
    ///   so `size_of::<Option<NonZeroXxx>>() == size_of::<xxx>()`; see
    ///   `NonZeroXxx` documentation.
    /// - `Unaligned`: `NonZeroU8` and `NonZeroI8` document that
    ///   `Option<NonZeroU8>` and `Option<NonZeroI8>` both have size 1. [1] [2]
    ///   This is worded in a way that makes it unclear whether it's meant as a
    ///   guarantee, but given the purpose of those types, it's virtually
    ///   unthinkable that that would ever change. The only valid alignment for
    ///   a 1-byte type is 1.
    ///
    /// TODO(#429): Add quotes from documentation.
    ///
    /// [1] https://doc.rust-lang.org/stable/std/num/struct.NonZeroU8.html
    /// [2] https://doc.rust-lang.org/stable/std/num/struct.NonZeroI8.html
    ///
    /// TODO(https://github.com/rust-lang/rust/pull/104082): Cite documentation
    /// for layout guarantees.
    unsafe_impl!(Option<NonZeroU8>: TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
    unsafe_impl!(Option<NonZeroI8>: TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
    assert_unaligned!(Option<NonZeroU8>, Option<NonZeroI8>);
    unsafe_impl!(Option<NonZeroU16>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroI16>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroU32>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroI32>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroU64>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroI64>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroU128>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroI128>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroUsize>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
    unsafe_impl!(Option<NonZeroIsize>: TryFromBytes, FromZeros, FromBytes, IntoBytes);
}

safety_comment! {
    /// SAFETY:
    /// While it's not fully documented, the consensus is that `Box<T>` does not
    /// contain any `UnsafeCell`s for `T: Sized` [1].
    ///
    /// [1] https://github.com/rust-lang/unsafe-code-guidelines/issues/492
    ///
    /// TODO(#896): Write a more complete safety proof before the next stable
    /// release.
    #[cfg(feature = "alloc")]
    unsafe_impl!(
        #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
        T: Sized => Immutable for Box<T>
    );
}

safety_comment! {
    /// SAFETY:
    /// The following types can be transmuted from `[0u8; size_of::<T>()]`. [1]
    ///
    /// [1] Per https://doc.rust-lang.org/nightly/core/option/index.html#representation:
    ///
    ///   Rust guarantees to optimize the following types `T` such that
    ///   [`Option<T>`] has the same size and alignment as `T`. In some of these
    ///   cases, Rust further guarantees that `transmute::<_, Option<T>>([0u8;
    ///   size_of::<T>()])` is sound and produces `Option::<T>::None`. These
    ///   cases are identified by the second column:
    ///
    ///   | `T`                   | `transmute::<_, Option<T>>([0u8; size_of::<T>()])` sound? |
    ///   |-----------------------|-----------------------------------------------------------|
    ///   | [`Box<U>`]            | when `U: Sized`                                           |
    ///   | `&U`                  | when `U: Sized`                                           |
    ///   | `&mut U`              | when `U: Sized`                                           |
    ///   | [`ptr::NonNull<U>`]   | when `U: Sized`                                           |
    ///   | `fn`, `extern "C" fn` | always                                                    |
    ///
    /// TODO(#429), TODO(https://github.com/rust-lang/rust/pull/115333): Cite
    /// the Stable docs once they're available.
    #[cfg(feature = "alloc")]
    unsafe_impl!(
        #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
        T => TryFromBytes for Option<Box<T>>;
        |c: Maybe<Option<Box<T>>>| pointer::is_zeroed(c)
    );
    #[cfg(feature = "alloc")]
    unsafe_impl!(
        #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
        T => FromZeros for Option<Box<T>>
    );
    unsafe_impl!(
        T => TryFromBytes for Option<&'_ T>;
        |c: Maybe<Option<&'_ T>>| pointer::is_zeroed(c)
    );
    unsafe_impl!(T => FromZeros for Option<&'_ T>);
    unsafe_impl!(
            T => TryFromBytes for Option<&'_ mut T>;
            |c: Maybe<Option<&'_ mut T>>| pointer::is_zeroed(c)
    );
    unsafe_impl!(T => FromZeros for Option<&'_ mut T>);
    unsafe_impl!(
        T => TryFromBytes for Option<NonNull<T>>;
        |c: Maybe<Option<NonNull<T>>>| pointer::is_zeroed(c)
    );
    unsafe_impl!(T => FromZeros for Option<NonNull<T>>);
    unsafe_impl_for_power_set!(A, B, C, D, E, F, G, H, I, J, K, L -> M => FromZeros for opt_fn!(...));
    unsafe_impl_for_power_set!(
        A, B, C, D, E, F, G, H, I, J, K, L -> M => TryFromBytes for opt_fn!(...);
        |c: Maybe<Self>| pointer::is_zeroed(c)
    );
    unsafe_impl_for_power_set!(A, B, C, D, E, F, G, H, I, J, K, L -> M => FromZeros for opt_extern_c_fn!(...));
    unsafe_impl_for_power_set!(
        A, B, C, D, E, F, G, H, I, J, K, L -> M => TryFromBytes for opt_extern_c_fn!(...);
        |c: Maybe<Self>| pointer::is_zeroed(c)
    );
}

safety_comment! {
    /// SAFETY:
    /// TODO(#896): Write this safety proof before the next stable release.
    unsafe_impl_for_power_set!(A, B, C, D, E, F, G, H, I, J, K, L -> M => Immutable for opt_fn!(...));
    unsafe_impl_for_power_set!(A, B, C, D, E, F, G, H, I, J, K, L -> M => Immutable for opt_extern_c_fn!(...));
}

macro_rules! impl_traits_for_atomics {
    ($($atomics:ident [$inners:ident]),* $(,)?) => {
        $(
            impl_for_transparent_wrapper!(TryFromBytes for $atomics [UnsafeCell<$inners>]);
            impl_for_transparent_wrapper!(FromZeros for $atomics [UnsafeCell<$inners>]);
            impl_for_transparent_wrapper!(FromBytes for $atomics [UnsafeCell<$inners>]);
            impl_for_transparent_wrapper!(IntoBytes for $atomics [UnsafeCell<$inners>]);
        )*
    };
}

#[rustfmt::skip]
impl_traits_for_atomics!(
    AtomicBool [bool],
    AtomicI16 [i16], AtomicI32 [i32], AtomicI8 [i8], AtomicIsize [isize],
    AtomicU16 [u16], AtomicU32 [u32], AtomicU8 [u8], AtomicUsize [usize],
);

safety_comment! {
    /// SAFETY:
    /// Per [1], `AtomicBool`, `AtomicU8`, and `AtomicI8` have the same size as
    /// `bool`, `u8`, and `i8` respectively. Since a type's alignment cannot be
    /// smaller than 1 [2], and since its alignment cannot be greater than its
    /// size [3], the only possible value for the alignment is 1. Thus, it is
    /// sound to implement `Unaligned`.
    ///
    /// [1] TODO(#896), TODO(https://github.com/rust-lang/rust/pull/121943):
    ///     Cite docs once they've landed.
    ///
    /// [2] Per https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment:
    ///
    ///     Alignment is measured in bytes, and must be at least 1.
    ///
    /// [3] Per https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment:
    ///
    ///     The size of a value is always a multiple of its alignment.
    unsafe_impl!(AtomicBool: Unaligned);
    unsafe_impl!(AtomicU8: Unaligned);
    unsafe_impl!(AtomicI8: Unaligned);
    assert_unaligned!(AtomicBool, AtomicU8, AtomicI8);
}

safety_comment! {
    /// SAFETY:
    /// Per reference [1]:
    /// "For all T, the following are guaranteed:
    /// size_of::<PhantomData<T>>() == 0
    /// align_of::<PhantomData<T>>() == 1".
    /// This gives:
    /// - `Immutable`: `PhantomData` has no fields.
    /// - `TryFromBytes` (with no validator), `FromZeros`, `FromBytes`: There is
    ///   only one possible sequence of 0 bytes, and `PhantomData` is inhabited.
    /// - `IntoBytes`: Since `PhantomData` has size 0, it contains no padding
    ///   bytes.
    /// - `Unaligned`: Per the preceding reference, `PhantomData` has alignment
    ///   1.
    ///
    /// [1] https://doc.rust-lang.org/std/marker/struct.PhantomData.html#layout-1
    unsafe_impl!(T: ?Sized => Immutable for PhantomData<T>);
    unsafe_impl!(T: ?Sized => TryFromBytes for PhantomData<T>);
    unsafe_impl!(T: ?Sized => FromZeros for PhantomData<T>);
    unsafe_impl!(T: ?Sized => FromBytes for PhantomData<T>);
    unsafe_impl!(T: ?Sized => IntoBytes for PhantomData<T>);
    unsafe_impl!(T: ?Sized => Unaligned for PhantomData<T>);
    assert_unaligned!(PhantomData<()>, PhantomData<u8>, PhantomData<u64>);
}

impl_for_transparent_wrapper!(T: Immutable => Immutable for Wrapping<T>);
impl_for_transparent_wrapper!(T: TryFromBytes => TryFromBytes for Wrapping<T>);
impl_for_transparent_wrapper!(T: FromZeros => FromZeros for Wrapping<T>);
impl_for_transparent_wrapper!(T: FromBytes => FromBytes for Wrapping<T>);
impl_for_transparent_wrapper!(T: IntoBytes => IntoBytes for Wrapping<T>);
impl_for_transparent_wrapper!(T: Unaligned => Unaligned for Wrapping<T>);
assert_unaligned!(Wrapping<()>, Wrapping<u8>);

safety_comment! {
    /// SAFETY:
    /// `TryFromBytes` (with no validator), `FromZeros`, `FromBytes`:
    /// `MaybeUninit<T>` has no restrictions on its contents.
    unsafe_impl!(T => TryFromBytes for MaybeUninit<T>);
    unsafe_impl!(T => FromZeros for MaybeUninit<T>);
    unsafe_impl!(T => FromBytes for MaybeUninit<T>);
}

impl_for_transparent_wrapper!(T: Immutable => Immutable for MaybeUninit<T>);
impl_for_transparent_wrapper!(T: Unaligned => Unaligned for MaybeUninit<T>);
assert_unaligned!(MaybeUninit<()>, MaybeUninit<u8>);

impl_for_transparent_wrapper!(T: ?Sized + Immutable => Immutable for ManuallyDrop<T>);
impl_for_transparent_wrapper!(T: ?Sized + TryFromBytes => TryFromBytes for ManuallyDrop<T>);
impl_for_transparent_wrapper!(T: ?Sized + FromZeros => FromZeros for ManuallyDrop<T>);
impl_for_transparent_wrapper!(T: ?Sized + FromBytes => FromBytes for ManuallyDrop<T>);
impl_for_transparent_wrapper!(T: ?Sized + IntoBytes => IntoBytes for ManuallyDrop<T>);
impl_for_transparent_wrapper!(T: ?Sized + Unaligned => Unaligned for ManuallyDrop<T>);
assert_unaligned!(ManuallyDrop<()>, ManuallyDrop<u8>);

// TODO(#5): Implement `FromZeros` and `FromBytes` when `T: ?Sized`.
impl_for_transparent_wrapper!(T: FromZeros => FromZeros for UnsafeCell<T>);
impl_for_transparent_wrapper!(T: FromBytes => FromBytes for UnsafeCell<T>);
impl_for_transparent_wrapper!(T: ?Sized + IntoBytes => IntoBytes for UnsafeCell<T>);
impl_for_transparent_wrapper!(T: ?Sized + Unaligned => Unaligned for UnsafeCell<T>);
assert_unaligned!(UnsafeCell<()>, UnsafeCell<u8>);

// SAFETY: See safety comment in `is_bit_valid` impl.
//
// TODO(#5): Try to add `T: ?Sized` bound.
unsafe impl<T: TryFromBytes> TryFromBytes for UnsafeCell<T> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }

    #[inline]
    fn is_bit_valid<A: invariant::Aliasing + invariant::AtLeast<invariant::Shared>>(
        candidate: Maybe<'_, Self, A>,
    ) -> bool {
        // The only way to implement this function is using an exclusive-aliased
        // pointer. `UnsafeCell`s cannot be read via shared-aliased pointers
        // (other than by using `unsafe` code, which we can't use since we can't
        // guarantee how our users are accessing or modifying the `UnsafeCell`).
        //
        // `is_bit_valid` is documented as panicking or failing to monomorphize
        // if called with a shared-aliased pointer on a type containing an
        // `UnsafeCell`. In practice, it will always be a monorphization error.
        // Since `is_bit_valid` is `#[doc(hidden)]` and only called directly
        // from this crate, we only need to worry about our own code incorrectly
        // calling `UnsafeCell::is_bit_valid`. The post-monomorphization error
        // makes it easier to test that this is truly the case, and also means
        // that if we make a mistake, it will cause downstream code to fail to
        // compile, which will immediately surface the mistake and give us a
        // chance to fix it quickly.
        let c = candidate.into_exclusive_or_post_monomorphization_error();

        // We wrap in `Unalign` here so that we can get a vanilla Rust reference
        // below, which in turn allows us to call `UnsafeCell::get_mut`.
        //
        // SAFETY:
        // - `.cast` preserves address. `Unalign` and `MaybeUninit` both have
        //   the same size as the types they wrap [1]. Thus, this cast will
        //   preserve the size of the pointer. As a result, the cast will
        //   address the same bytes as `c`.
        // - `.cast` preserves provenance.
        // - Since both the source and destination types are wrapped in
        //   `UnsafeCell`, all bytes of both types are inside of `UnsafeCell`s,
        //   and so the byte ranges covered by `UnsafeCell`s are identical in
        //   both types. Since the pointers refer to the same byte ranges,
        //   the same is true of the pointers' referents as well.
        //
        // [1] Per https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#layout-1:
        //
        //   MaybeUninit<T> is guaranteed to have the same size, alignment, and
        //   ABI as T.
        let c = unsafe {
            c.cast_unsized(|c: *mut UnsafeCell<T>| c.cast::<UnsafeCell<Unalign<MaybeUninit<T>>>>())
        };
        // SAFETY: `MaybeUninit` has no validity requirements.
        let c = unsafe { c.assume_valid() };
        let c = c.bikeshed_recall_aligned();
        // This is the crucial step at which we use `UnsafeCell::get_mut` to go
        // from `UnsafeCell<U>` to `U` (where `U = Unalign<MaybeUninit<T>>`).
        // Now that we've gotten rid of the `UnsafeCell`, we can delegate to
        // `T::is_bit_valid`.
        let c: &mut Unalign<MaybeUninit<T>> = c.as_mut().get_mut();
        // This converts from an aligned `Unalign<MaybeUninit<T>>` pointer to an
        // unaligned `MaybeUninit<T>` pointer.
        let c: Ptr<'_, MaybeUninit<T>, _> = Ptr::from_mut(c).transparent_wrapper_into_inner();
        let c: Ptr<'_, T, _> = c.transparent_wrapper_into_inner();

        // SAFETY: The original `candidate` argument has `Initialized` validity.
        // None of the subsequent operations modify the memory itself, and so
        // that guarantee is still upheld.
        let c = unsafe { c.assume_initialized() };
        // Confirm that `Maybe` is a type alias for `Ptr` with the validity
        // invariant `Initialized`. Our safety proof depends upon this
        // invariant, and it might change at some point. If that happens, we
        // want this function to stop compiling.
        let _: Ptr<'_, UnsafeCell<T>, (_, _, invariant::Initialized)> = candidate;

        // SAFETY: Since `UnsafeCell<T>` and `T` have the same layout and bit
        // validity, `UnsafeCell<T>` is bit-valid exactly when its wrapped `T`
        // is. Thus, this is a sound implementation of
        // `UnsafeCell::is_bit_valid`.
        T::is_bit_valid(c.forget_exclusive())
    }
}

safety_comment! {
    /// SAFETY:
    /// Per the reference [1]:
    ///
    ///   An array of `[T; N]` has a size of `size_of::<T>() * N` and the same
    ///   alignment of `T`. Arrays are laid out so that the zero-based `nth`
    ///   element of the array is offset from the start of the array by `n *
    ///   size_of::<T>()` bytes.
    ///
    ///   ...
    ///
    ///   Slices have the same layout as the section of the array they slice.
    ///
    /// In other words, the layout of a `[T]` or `[T; N]` is a sequence of `T`s
    /// laid out back-to-back with no bytes in between. Therefore, `[T]` or `[T;
    /// N]` are `Immutable`, `TryFromBytes`, `FromZeros`, `FromBytes`, and
    /// `IntoBytes` if `T` is (respectively). Furthermore, since an array/slice
    /// has "the same alignment of `T`", `[T]` and `[T; N]` are `Unaligned` if
    /// `T` is.
    ///
    /// Note that we don't `assert_unaligned!` for slice types because
    /// `assert_unaligned!` uses `align_of`, which only works for `Sized` types.
    ///
    /// [1] https://doc.rust-lang.org/reference/type-layout.html#array-layout
    unsafe_impl!(const N: usize, T: Immutable => Immutable for [T; N]);
    unsafe_impl!(const N: usize, T: TryFromBytes => TryFromBytes for [T; N]; |c: Maybe<[T; N]>| {
        // Note that this call may panic, but it would still be sound even if it
        // did. `is_bit_valid` does not promise that it will not panic (in fact,
        // it explicitly warns that it's a possibility), and we have not
        // violated any safety invariants that we must fix before returning.
        <[T] as TryFromBytes>::is_bit_valid(c.as_slice())
    });
    unsafe_impl!(const N: usize, T: FromZeros => FromZeros for [T; N]);
    unsafe_impl!(const N: usize, T: FromBytes => FromBytes for [T; N]);
    unsafe_impl!(const N: usize, T: IntoBytes => IntoBytes for [T; N]);
    unsafe_impl!(const N: usize, T: Unaligned => Unaligned for [T; N]);
    assert_unaligned!([(); 0], [(); 1], [u8; 0], [u8; 1]);
    unsafe_impl!(T: Immutable => Immutable for [T]);
    unsafe_impl!(T: TryFromBytes => TryFromBytes for [T]; |c: Maybe<[T]>| {
        // SAFETY: Per the reference [1]:
        //
        //   An array of `[T; N]` has a size of `size_of::<T>() * N` and the
        //   same alignment of `T`. Arrays are laid out so that the zero-based
        //   `nth` element of the array is offset from the start of the array by
        //   `n * size_of::<T>()` bytes.
        //
        //   ...
        //
        //   Slices have the same layout as the section of the array they slice.
        //
        // In other words, the layout of a `[T] is a sequence of `T`s laid out
        // back-to-back with no bytes in between. If all elements in `candidate`
        // are `is_bit_valid`, so too is `candidate`.
        //
        // Note that any of the below calls may panic, but it would still be
        // sound even if it did. `is_bit_valid` does not promise that it will
        // not panic (in fact, it explicitly warns that it's a possibility), and
        // we have not violated any safety invariants that we must fix before
        // returning.
        c.iter().all(<T as TryFromBytes>::is_bit_valid)
    });
    unsafe_impl!(T: FromZeros => FromZeros for [T]);
    unsafe_impl!(T: FromBytes => FromBytes for [T]);
    unsafe_impl!(T: IntoBytes => IntoBytes for [T]);
    unsafe_impl!(T: Unaligned => Unaligned for [T]);
}
safety_comment! {
    /// SAFETY:
    /// - `Immutable`: Raw pointers do not contain any `UnsafeCell`s.
    /// - `FromZeros`: For thin pointers (note that `T: Sized`), the zero
    ///   pointer is considered "null". [1] No operations which require
    ///   provenance are legal on null pointers, so this is not a footgun.
    /// - `TryFromBytes`: By the same reasoning as for `FromZeroes`, we can
    ///   implement `TryFromBytes` for thin pointers provided that
    ///   [`TryFromByte::is_bit_valid`] only produces `true` for zeroed bytes.
    ///
    /// NOTE(#170): Implementing `FromBytes` and `IntoBytes` for raw pointers
    /// would be sound, but carries provenance footguns. We want to support
    /// `FromBytes` and `IntoBytes` for raw pointers eventually, but we are
    /// holding off until we can figure out how to address those footguns.
    ///
    /// [1] TODO(https://github.com/rust-lang/rust/pull/116988): Cite the
    /// documentation once this PR lands.
    unsafe_impl!(T: ?Sized => Immutable for *const T);
    unsafe_impl!(T: ?Sized => Immutable for *mut T);
    unsafe_impl!(T => TryFromBytes for *const T; |c: Maybe<*const T>| {
        pointer::is_zeroed(c)
    });
    unsafe_impl!(T => FromZeros for *const T);
    unsafe_impl!(T => TryFromBytes for *mut T; |c: Maybe<*const T>| {
        pointer::is_zeroed(c)
    });
    unsafe_impl!(T => FromZeros for *mut T);
}

safety_comment! {
    /// SAFETY:
    ///
    /// TODO(#896): Write this safety proof before the next stable release.
    unsafe_impl!(T: ?Sized => Immutable for NonNull<T>);
}

safety_comment! {
    /// SAFETY:
    /// Reference types do not contain any `UnsafeCell`s.
    unsafe_impl!(T: ?Sized => Immutable for &'_ T);
    unsafe_impl!(T: ?Sized => Immutable for &'_ mut T);
}

safety_comment! {
    /// SAFETY:
    /// `Option` is not `#[non_exhaustive]` [1], which means that the types in
    /// its variants cannot change, and no new variants can be added.
    /// `Option<T>` does not contain any `UnsafeCell`s outside of `T`. [1]
    ///
    /// [1] https://doc.rust-lang.org/core/option/enum.Option.html
    unsafe_impl!(T: Immutable => Immutable for Option<T>);
}

// SIMD support
//
// Per the Unsafe Code Guidelines Reference [1]:
//
//   Packed SIMD vector types are `repr(simd)` homogeneous tuple-structs
//   containing `N` elements of type `T` where `N` is a power-of-two and the
//   size and alignment requirements of `T` are equal:
//
//   ```rust
//   #[repr(simd)]
//   struct Vector<T, N>(T_0, ..., T_(N - 1));
//   ```
//
//   ...
//
//   The size of `Vector` is `N * size_of::<T>()` and its alignment is an
//   implementation-defined function of `T` and `N` greater than or equal to
//   `align_of::<T>()`.
//
//   ...
//
//   Vector elements are laid out in source field order, enabling random access
//   to vector elements by reinterpreting the vector as an array:
//
//   ```rust
//   union U {
//      vec: Vector<T, N>,
//      arr: [T; N]
//   }
//
//   assert_eq!(size_of::<Vector<T, N>>(), size_of::<[T; N]>());
//   assert!(align_of::<Vector<T, N>>() >= align_of::<[T; N]>());
//
//   unsafe {
//     let u = U { vec: Vector<T, N>(t_0, ..., t_(N - 1)) };
//
//     assert_eq!(u.vec.0, u.arr[0]);
//     // ...
//     assert_eq!(u.vec.(N - 1), u.arr[N - 1]);
//   }
//   ```
//
// Given this background, we can observe that:
// - The size and bit pattern requirements of a SIMD type are equivalent to the
//   equivalent array type. Thus, for any SIMD type whose primitive `T` is
//   `Immutable`, `TryFromBytes`, `FromZeros`, `FromBytes`, or `IntoBytes`, that
//   SIMD type is also `Immutable`, `TryFromBytes`, `FromZeros`, `FromBytes`, or
//   `IntoBytes` respectively.
// - Since no upper bound is placed on the alignment, no SIMD type can be
//   guaranteed to be `Unaligned`.
//
// Also per [1]:
//
//   This chapter represents the consensus from issue #38. The statements in
//   here are not (yet) "guaranteed" not to change until an RFC ratifies them.
//
// See issue #38 [2]. While this behavior is not technically guaranteed, the
// likelihood that the behavior will change such that SIMD types are no longer
// `TryFromBytes`, `FromZeros`, `FromBytes`, or `IntoBytes` is next to zero, as
// that would defeat the entire purpose of SIMD types. Nonetheless, we put this
// behavior behind the `simd` Cargo feature, which requires consumers to opt
// into this stability hazard.
//
// [1] https://rust-lang.github.io/unsafe-code-guidelines/layout/packed-simd-vectors.html
// [2] https://github.com/rust-lang/unsafe-code-guidelines/issues/38
#[cfg(feature = "simd")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "simd")))]
mod simd {
    /// Defines a module which implements `TryFromBytes`, `FromZeros`,
    /// `FromBytes`, and `IntoBytes` for a set of types from a module in
    /// `core::arch`.
    ///
    /// `$arch` is both the name of the defined module and the name of the
    /// module in `core::arch`, and `$typ` is the list of items from that module
    /// to implement `FromZeros`, `FromBytes`, and `IntoBytes` for.
    #[allow(unused_macros)] // `allow(unused_macros)` is needed because some
                            // target/feature combinations don't emit any impls
                            // and thus don't use this macro.
    macro_rules! simd_arch_mod {
        (#[cfg $cfg:tt] $arch:ident, $mod:ident, $($typ:ident),*) => {
            #[cfg $cfg]
            #[cfg_attr(doc_cfg, doc(cfg $cfg))]
            mod $mod {
                use core::arch::$arch::{$($typ),*};

                use crate::*;
                impl_known_layout!($($typ),*);
                safety_comment! {
                    /// SAFETY:
                    /// See comment on module definition for justification.
                    $( unsafe_impl!($typ: Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes); )*
                }
            }
        };
    }

    #[rustfmt::skip]
    const _: () = {
        simd_arch_mod!(
            #[cfg(target_arch = "x86")]
            x86, x86, __m128, __m128d, __m128i, __m256, __m256d, __m256i
        );
        simd_arch_mod!(
            #[cfg(all(feature = "simd-nightly", target_arch = "x86"))]
            x86, x86_nightly, __m512bh, __m512, __m512d, __m512i
        );
        simd_arch_mod!(
            #[cfg(target_arch = "x86_64")]
            x86_64, x86_64, __m128, __m128d, __m128i, __m256, __m256d, __m256i
        );
        simd_arch_mod!(
            #[cfg(all(feature = "simd-nightly", target_arch = "x86_64"))]
            x86_64, x86_64_nightly, __m512bh, __m512, __m512d, __m512i
        );
        simd_arch_mod!(
            #[cfg(target_arch = "wasm32")]
            wasm32, wasm32, v128
        );
        simd_arch_mod!(
            #[cfg(all(feature = "simd-nightly", target_arch = "powerpc"))]
            powerpc, powerpc, vector_bool_long, vector_double, vector_signed_long, vector_unsigned_long
        );
        simd_arch_mod!(
            #[cfg(all(feature = "simd-nightly", target_arch = "powerpc64"))]
            powerpc64, powerpc64, vector_bool_long, vector_double, vector_signed_long, vector_unsigned_long
        );
        #[cfg(zerocopy_aarch64_simd)]
        simd_arch_mod!(
            #[cfg(target_arch = "aarch64")]
            aarch64, aarch64, float32x2_t, float32x4_t, float64x1_t, float64x2_t, int8x8_t, int8x8x2_t,
            int8x8x3_t, int8x8x4_t, int8x16_t, int8x16x2_t, int8x16x3_t, int8x16x4_t, int16x4_t,
            int16x8_t, int32x2_t, int32x4_t, int64x1_t, int64x2_t, poly8x8_t, poly8x8x2_t, poly8x8x3_t,
            poly8x8x4_t, poly8x16_t, poly8x16x2_t, poly8x16x3_t, poly8x16x4_t, poly16x4_t, poly16x8_t,
            poly64x1_t, poly64x2_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t, uint8x16_t,
            uint8x16x2_t, uint8x16x3_t, uint8x16x4_t, uint16x4_t, uint16x8_t, uint32x2_t, uint32x4_t,
            uint64x1_t, uint64x2_t
        );
        simd_arch_mod!(
            #[cfg(all(feature = "simd-nightly", target_arch = "arm"))]
            arm, arm, int8x4_t, uint8x4_t
        );
    };
}

/// Safely transmutes a value of one type to a value of another type of the same
/// size.
///
/// This macro behaves like an invocation of this function:
///
/// ```ignore
/// const fn transmute<Src, Dst>(src: Src) -> Dst
/// where
///     Src: IntoBytes,
///     Dst: FromBytes,
///     size_of::<Src>() == size_of::<Dst>(),
/// {
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// However, unlike a function, this macro can only be invoked when the types of
/// `Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
/// inferred from the calling context; they cannot be explicitly specified in
/// the macro invocation.
///
/// Note that the `Src` produced by the expression `$e` will *not* be dropped.
/// Semantically, its bits will be copied into a new value of type `Dst`, the
/// original `Src` will be forgotten, and the value of type `Dst` will be
/// returned.
///
/// # Examples
///
/// ```
/// # use zerocopy::transmute;
/// let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
///
/// let two_dimensional: [[u8; 4]; 2] = transmute!(one_dimensional);
///
/// assert_eq!(two_dimensional, [[0, 1, 2, 3], [4, 5, 6, 7]]);
/// ```
///
/// # Use in `const` contexts
///
/// This macro can be invoked in `const` contexts.
#[macro_export]
macro_rules! transmute {
    ($e:expr) => {{
        // NOTE: This must be a macro (rather than a function with trait bounds)
        // because there's no way, in a generic context, to enforce that two
        // types have the same size. `core::mem::transmute` uses compiler magic
        // to enforce this so long as the types are concrete.

        let e = $e;
        if false {
            // This branch, though never taken, ensures that the type of `e` is
            // `IntoBytes` and that the type of this macro invocation expression
            // is `FromBytes`.

            struct AssertIsIntoBytes<T: $crate::IntoBytes>(T);
            let _ = AssertIsIntoBytes(e);

            struct AssertIsFromBytes<U: $crate::FromBytes>(U);
            #[allow(unused, unreachable_code)]
            let u = AssertIsFromBytes(loop {});
            u.0
        } else {
            // SAFETY: `core::mem::transmute` ensures that the type of `e` and
            // the type of this macro invocation expression have the same size.
            // We know this transmute is safe thanks to the `IntoBytes` and
            // `FromBytes` bounds enforced by the `false` branch.
            //
            // We use this reexport of `core::mem::transmute` because we know it
            // will always be available for crates which are using the 2015
            // edition of Rust. By contrast, if we were to use
            // `std::mem::transmute`, this macro would not work for such crates
            // in `no_std` contexts, and if we were to use
            // `core::mem::transmute`, this macro would not work in `std`
            // contexts in which `core` was not manually imported. This is not a
            // problem for 2018 edition crates.
            let u = unsafe {
                // Clippy:
                // - It's okay to transmute a type to itself.
                // - We can't annotate the types; this macro is designed to
                //   infer the types from the calling context.
                #[allow(clippy::useless_transmute, clippy::missing_transmute_annotations)]
                $crate::macro_util::core_reexport::mem::transmute(e)
            };
            $crate::macro_util::must_use(u)
        }
    }}
}

/// Safely transmutes a mutable or immutable reference of one type to an
/// immutable reference of another type of the same size.
///
/// This macro behaves like an invocation of this function:
///
/// ```ignore
/// const fn transmute_ref<'src, 'dst, Src, Dst>(src: &'src Src) -> &'dst Dst
/// where
///     'src: 'dst,
///     Src: IntoBytes + Immutable,
///     Dst: FromBytes + Immutable,
///     size_of::<Src>() == size_of::<Dst>(),
///     align_of::<Src>() >= align_of::<Dst>(),
/// {
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// However, unlike a function, this macro can only be invoked when the types of
/// `Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
/// inferred from the calling context; they cannot be explicitly specified in
/// the macro invocation.
///
/// # Examples
///
/// ```
/// # use zerocopy::transmute_ref;
/// let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
///
/// let two_dimensional: &[[u8; 4]; 2] = transmute_ref!(&one_dimensional);
///
/// assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);
/// ```
///
/// # Use in `const` contexts
///
/// This macro can be invoked in `const` contexts.
///
/// # Alignment increase error message
///
/// Because of limitations on macros, the error message generated when
/// `transmute_ref!` is used to transmute from a type of lower alignment to a
/// type of higher alignment is somewhat confusing. For example, the following
/// code:
///
/// ```compile_fail
/// const INCREASE_ALIGNMENT: &u16 = zerocopy::transmute_ref!(&[0u8; 2]);
/// ```
///
/// ...generates the following error:
///
/// ```text
/// error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
///  --> src/lib.rs:1524:34
///   |
/// 5 | const INCREASE_ALIGNMENT: &u16 = zerocopy::transmute_ref!(&[0u8; 2]);
///   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///   |
///   = note: source type: `AlignOf<[u8; 2]>` (8 bits)
///   = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
///   = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `transmute_ref` (in Nightly builds, run with -Z macro-backtrace for more info)
/// ```
///
/// This is saying that `max(align_of::<T>(), align_of::<U>()) !=
/// align_of::<T>()`, which is equivalent to `align_of::<T>() <
/// align_of::<U>()`.
#[macro_export]
macro_rules! transmute_ref {
    ($e:expr) => {{
        // NOTE: This must be a macro (rather than a function with trait bounds)
        // because there's no way, in a generic context, to enforce that two
        // types have the same size or alignment.

        // Ensure that the source type is a reference or a mutable reference
        // (note that mutable references are implicitly reborrowed here).
        let e: &_ = $e;

        #[allow(unused, clippy::diverging_sub_expression)]
        if false {
            // This branch, though never taken, ensures that the type of `e` is
            // `&T` where `T: 't + Sized + IntoBytes + Immutable`, that the type of
            // this macro expression is `&U` where `U: 'u + Sized + FromBytes +
            // Immutable`, and that `'t` outlives `'u`.

            struct AssertSrcIsSized<'a, T: ::core::marker::Sized>(&'a T);
            struct AssertSrcIsIntoBytes<'a, T: ?::core::marker::Sized + $crate::IntoBytes>(&'a T);
            struct AssertSrcIsImmutable<'a, T: ?::core::marker::Sized + $crate::Immutable>(&'a T);
            struct AssertDstIsSized<'a, T: ::core::marker::Sized>(&'a T);
            struct AssertDstIsFromBytes<'a, U: ?::core::marker::Sized + $crate::FromBytes>(&'a U);
            struct AssertDstIsImmutable<'a, T: ?::core::marker::Sized + $crate::Immutable>(&'a T);

            let _ = AssertSrcIsSized(e);
            let _ = AssertSrcIsIntoBytes(e);
            let _ = AssertSrcIsImmutable(e);

            if true {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsSized(loop {});
                u.0
            } else if true {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsFromBytes(loop {});
                u.0
            } else {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsImmutable(loop {});
                u.0
            }
        } else if false {
            // This branch, though never taken, ensures that `size_of::<T>() ==
            // size_of::<U>()` and that that `align_of::<T>() >=
            // align_of::<U>()`.

            // `t` is inferred to have type `T` because it's assigned to `e` (of
            // type `&T`) as `&t`.
            let mut t = loop {};
            e = &t;

            // `u` is inferred to have type `U` because it's used as `&u` as the
            // value returned from this branch.
            let u;

            $crate::assert_size_eq!(t, u);
            $crate::assert_align_gt_eq!(t, u);

            &u
        } else {
            // SAFETY: For source type `Src` and destination type `Dst`:
            // - We know that `Src: IntoBytes + Immutable` and `Dst: FromBytes +
            //   Immutable` thanks to the uses of `AssertSrcIsIntoBytes`,
            //   `AssertSrcIsImmutable`, `AssertDstIsFromBytes`, and
            //   `AssertDstIsImmutable` above.
            // - We know that `size_of::<Src>() == size_of::<Dst>()` thanks to
            //   the use of `assert_size_eq!` above.
            // - We know that `align_of::<Src>() >= align_of::<Dst>()` thanks to
            //   the use of `assert_align_gt_eq!` above.
            let u = unsafe { $crate::macro_util::transmute_ref(e) };
            $crate::macro_util::must_use(u)
        }
    }}
}

/// Safely transmutes a mutable reference of one type to a mutable reference of
/// another type of the same size.
///
/// This macro behaves like an invocation of this function:
///
/// ```ignore
/// const fn transmute_mut<'src, 'dst, Src, Dst>(src: &'src mut Src) -> &'dst mut Dst
/// where
///     'src: 'dst,
///     Src: FromBytes + IntoBytes + Immutable,
///     Dst: FromBytes + IntoBytes + Immutable,
///     size_of::<Src>() == size_of::<Dst>(),
///     align_of::<Src>() >= align_of::<Dst>(),
/// {
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// However, unlike a function, this macro can only be invoked when the types of
/// `Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
/// inferred from the calling context; they cannot be explicitly specified in
/// the macro invocation.
///
/// # Examples
///
/// ```
/// # use zerocopy::transmute_mut;
/// let mut one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
///
/// let two_dimensional: &mut [[u8; 4]; 2] = transmute_mut!(&mut one_dimensional);
///
/// assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);
///
/// two_dimensional.reverse();
///
/// assert_eq!(one_dimensional, [4, 5, 6, 7, 0, 1, 2, 3]);
/// ```
///
/// # Use in `const` contexts
///
/// This macro can be invoked in `const` contexts.
///
/// # Alignment increase error message
///
/// Because of limitations on macros, the error message generated when
/// `transmute_mut!` is used to transmute from a type of lower alignment to a
/// type of higher alignment is somewhat confusing. For example, the following
/// code:
///
/// ```compile_fail
/// const INCREASE_ALIGNMENT: &mut u16 = zerocopy::transmute_mut!(&mut [0u8; 2]);
/// ```
///
/// ...generates the following error:
///
/// ```text
/// error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
///  --> src/lib.rs:1524:34
///   |
/// 5 | const INCREASE_ALIGNMENT: &mut u16 = zerocopy::transmute_mut!(&mut [0u8; 2]);
///   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///   |
///   = note: source type: `AlignOf<[u8; 2]>` (8 bits)
///   = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
///   = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `transmute_mut` (in Nightly builds, run with -Z macro-backtrace for more info)
/// ```
///
/// This is saying that `max(align_of::<T>(), align_of::<U>()) !=
/// align_of::<T>()`, which is equivalent to `align_of::<T>() <
/// align_of::<U>()`.
#[macro_export]
macro_rules! transmute_mut {
    ($e:expr) => {{
        // NOTE: This must be a macro (rather than a function with trait bounds)
        // because there's no way, in a generic context, to enforce that two
        // types have the same size or alignment.

        // Ensure that the source type is a mutable reference.
        let e: &mut _ = $e;

        #[allow(unused, clippy::diverging_sub_expression)]
        if false {
            // This branch, though never taken, ensures that the type of `e` is
            // `&mut T` where `T: 't + Sized + FromBytes + IntoBytes + Immutable`
            // and that the type of this macro expression is `&mut U` where `U:
            // 'u + Sized + FromBytes + IntoBytes + Immutable`.

            // We use immutable references here rather than mutable so that, if
            // this macro is used in a const context (in which, as of this
            // writing, mutable references are banned), the error message
            // appears to originate in the user's code rather than in the
            // internals of this macro.
            struct AssertSrcIsSized<'a, T: ::core::marker::Sized>(&'a T);
            struct AssertSrcIsFromBytes<'a, T: ?::core::marker::Sized + $crate::FromBytes>(&'a T);
            struct AssertSrcIsIntoBytes<'a, T: ?::core::marker::Sized + $crate::IntoBytes>(&'a T);
            struct AssertDstIsSized<'a, T: ::core::marker::Sized>(&'a T);
            struct AssertDstIsFromBytes<'a, T: ?::core::marker::Sized + $crate::FromBytes>(&'a T);
            struct AssertDstIsIntoBytes<'a, T: ?::core::marker::Sized + $crate::IntoBytes>(&'a T);

            if true {
                let _ = AssertSrcIsSized(&*e);
            } else if true {
                let _ = AssertSrcIsFromBytes(&*e);
            } else {
                let _ = AssertSrcIsIntoBytes(&*e);
            }

            if true {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsSized(loop {});
                &mut *u.0
            } else if true {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsFromBytes(loop {});
                &mut *u.0
            } else {
                #[allow(unused, unreachable_code)]
                let u = AssertDstIsIntoBytes(loop {});
                &mut *u.0
            }
        } else if false {
            // This branch, though never taken, ensures that `size_of::<T>() ==
            // size_of::<U>()` and that that `align_of::<T>() >=
            // align_of::<U>()`.

            // `t` is inferred to have type `T` because it's assigned to `e` (of
            // type `&mut T`) as `&mut t`.
            let mut t = loop {};
            e = &mut t;

            // `u` is inferred to have type `U` because it's used as `&mut u` as
            // the value returned from this branch.
            let u;

            $crate::assert_size_eq!(t, u);
            $crate::assert_align_gt_eq!(t, u);

            &mut u
        } else {
            // SAFETY: For source type `Src` and destination type `Dst`:
            // - We know that `size_of::<Src>() == size_of::<Dst>()` thanks to
            //   the use of `assert_size_eq!` above.
            // - We know that `align_of::<Src>() >= align_of::<Dst>()` thanks to
            //   the use of `assert_align_gt_eq!` above.
            let u = unsafe { $crate::macro_util::transmute_mut(e) };
            $crate::macro_util::must_use(u)
        }
    }}
}

/// Includes a file and safely transmutes it to a value of an arbitrary type.
///
/// The file will be included as a byte array, `[u8; N]`, which will be
/// transmuted to another type, `T`. `T` is inferred from the calling context,
/// and must implement [`FromBytes`].
///
/// The file is located relative to the current file (similarly to how modules
/// are found). The provided path is interpreted in a platform-specific way at
/// compile time. So, for instance, an invocation with a Windows path containing
/// backslashes `\` would not compile correctly on Unix.
///
/// `include_value!` is ignorant of byte order. For byte order-aware types, see
/// the [`byteorder`] module.
///
/// # Examples
///
/// Assume there are two files in the same directory with the following
/// contents:
///
/// File `data` (no trailing newline):
///
/// ```text
/// abcd
/// ```
///
/// File `main.rs`:
///
/// ```rust
/// use zerocopy::include_value;
/// # macro_rules! include_value {
/// # ($file:expr) => { zerocopy::include_value!(concat!("../testdata/include_value/", $file)) };
/// # }
///
/// fn main() {
///     let as_u32: u32 = include_value!("data");
///     assert_eq!(as_u32, u32::from_ne_bytes([b'a', b'b', b'c', b'd']));
///     let as_i32: i32 = include_value!("data");
///     assert_eq!(as_i32, i32::from_ne_bytes([b'a', b'b', b'c', b'd']));
/// }
/// ```
///
/// # Use in `const` contexts
///
/// This macro can be invoked in `const` contexts.
#[doc(alias("include_bytes", "include_data", "include_type"))]
#[macro_export]
macro_rules! include_value {
    ($file:expr $(,)?) => {
        $crate::transmute!(*::core::include_bytes!($file))
    };
}

/// A mutable or immutable reference to a byte slice.
///
/// `ByteSlice` abstracts over the mutability of a byte slice reference, and is
/// implemented for various special reference types such as [`Ref<[u8]>`] and
/// [`RefMut<[u8]>`].
///
/// [`Ref<[u8]>`]: core::cell::Ref
/// [`RefMut<[u8]>`]: core::cell::RefMut
///
/// # Safety
///
/// Implementations of `ByteSlice` must promise that their implementations of
/// [`Deref`] and [`DerefMut`] are "stable". In particular, given `B: ByteSlice`
/// and `b: B`, two calls, each to either `b.deref()` or `b.deref_mut()`, must
/// return a byte slice with the same address and length. This must hold even if
/// the two calls are separated by an arbitrary sequence of calls to methods on
/// `ByteSlice`, [`ByteSliceMut`], [`IntoByteSlice`], or [`IntoByteSliceMut`],
/// or on their super-traits. This does *not* need to hold if the two calls are
/// separated by any method calls, field accesses, or field modifications *other
/// than* those from these traits.
///
/// Note that this also implies that, given `b: B`, the address and length
/// cannot be modified via objects other than `b`, either on the same thread or
/// on another thread.
pub unsafe trait ByteSlice: Deref<Target = [u8]> + Sized {}

/// A mutable reference to a byte slice.
///
/// `ByteSliceMut` abstracts over various ways of storing a mutable reference to
/// a byte slice, and is implemented for various special reference types such as
/// `RefMut<[u8]>`.
///
/// `ByteSliceMut` is a shorthand for [`ByteSlice`] and [`DerefMut`].
pub trait ByteSliceMut: ByteSlice + DerefMut {}
impl<B: ByteSlice + DerefMut> ByteSliceMut for B {}

/// A [`ByteSlice`] which can be copied without violating dereference stability.
///
/// # Safety
///
/// If `B: CopyableByteSlice`, then the dereference stability properties
/// required by `ByteSlice` (see that trait's safety documentation) do not only
/// hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also hold
/// regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by copying
/// `b`.
pub unsafe trait CopyableByteSlice: ByteSlice + Copy + CloneableByteSlice {}

/// A [`ByteSlice`] which can be cloned without violating dereference stability.
///
/// # Safety
///
/// If `B: CloneableByteSlice`, then the dereference stability properties
/// required by `ByteSlice` (see that trait's safety documentation) do not only
/// hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also hold
/// regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
/// `b.clone()`, `b.clone().clone()`, etc.
pub unsafe trait CloneableByteSlice: ByteSlice + Clone {}

/// A [`ByteSlice`] that can be split in two.
///
/// # Safety
///
/// Unsafe code may depend for its soundness on the assumption that `split_at`
/// and `split_at_unchecked` are implemented correctly. In particular, given `B:
/// SplitByteSlice` and `b: B`, if `b.deref()` returns a byte slice with address
/// `addr` and length `len`, then if `split <= len`, both of these
/// invocations:
/// - `b.split_at(split)`
/// - `b.split_at_unchecked(split)`
///
/// ...will return `(first, second)` such that:
/// - `first`'s address is `addr` and its length is `split`
/// - `second`'s address is `addr + split` and its length is `len - split`
pub unsafe trait SplitByteSlice: ByteSlice {
    /// Splits the slice at the midpoint.
    ///
    /// `x.split_at(mid)` returns `x[..mid]` and `x[mid..]`.
    ///
    /// # Panics
    ///
    /// `x.split_at(mid)` panics if `mid > x.deref().len()`.
    #[must_use]
    #[inline]
    fn split_at(self, mid: usize) -> (Self, Self) {
        if let Ok(splits) = try_split_at(self, mid) {
            splits
        } else {
            panic!("mid > len")
        }
    }

    /// Splits the slice at the midpoint, possibly omitting bounds checks.
    ///
    /// `x.split_at_unchecked(mid)` returns `x[..mid]` and `x[mid..]`.
    ///
    /// # Safety
    ///
    /// `mid` must not be greater than `x.deref().len()`.
    #[must_use]
    unsafe fn split_at_unchecked(self, mid: usize) -> (Self, Self);
}

/// Attempts to split the slice at the midpoint.
///
/// `x.try_split_at(mid)` returns `Ok((x[..mid], x[mid..]))` if `mid <=
/// x.deref().len()` and otherwise returns `Err(x)`.
///
/// # Safety
///
/// Unsafe code may rely on this function correctly implementing the above
/// functionality.
#[inline]
fn try_split_at<S>(slice: S, mid: usize) -> Result<(S, S), S>
where
    S: SplitByteSlice,
{
    if mid <= slice.deref().len() {
        // SAFETY: Above, we ensure that `mid <= self.deref().len()`. By
        // invariant on `ByteSlice`, a supertrait of `SplitByteSlice`,
        // `.deref()` is guranteed to be "stable"; i.e., it will always
        // dereference to a byte slice of the same address and length. Thus, we
        // can be sure that the above precondition remains satisfied through the
        // call to `split_at_unchecked`.
        unsafe { Ok(slice.split_at_unchecked(mid)) }
    } else {
        Err(slice)
    }
}

/// A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`].
pub trait SplitByteSliceMut: SplitByteSlice + ByteSliceMut {}
impl<B: SplitByteSlice + ByteSliceMut> SplitByteSliceMut for B {}

/// A [`ByteSlice`] that conveys no ownership, and so can be converted into a
/// byte slice.
///
/// Some `ByteSlice` types (notably, the standard library's [`Ref`] type) convey
/// ownership, and so they cannot soundly be moved by-value into a byte slice
/// type (`&[u8]`). Some methods in this crate's API (such as [`Ref::into_ref`])
/// are only compatible with `ByteSlice` types without these ownership
/// semantics.
///
/// # Safety
///
/// Invoking `self.into()` produces a `&[u8]` with identical address and length
/// as the slice produced by `self.deref()`. Note that this implies that the
/// slice produced by `self.into()` is "stable" in the same sense as defined by
/// [`ByteSlice`]'s safety invariant.
///
/// [`Ref`]: core::cell::Ref
pub trait IntoByteSlice<'a>: ByteSlice + Into<&'a [u8]> {}

/// A [`ByteSliceMut`] that conveys no ownership, and so can be converted into a
/// mutable byte slice.
///
/// Some `ByteSliceMut` types (notably, the standard library's [`RefMut`] type)
/// convey ownership, and so they cannot soundly be moved by-value into a byte
/// slice type (`&mut [u8]`). Some methods in this crate's API (such as
/// [`Ref::into_mut`]) are only compatible with `ByteSliceMut` types without
/// these ownership semantics.
///
/// [`RefMut`]: core::cell::RefMut
pub trait IntoByteSliceMut<'a>: ByteSliceMut + Into<&'a mut [u8]> {}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> ByteSlice for &'a [u8] {}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> CopyableByteSlice for &'a [u8] {}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> CloneableByteSlice for &'a [u8] {}

// SAFETY: This delegates to `polyfills:split_at_unchecked`, which is documented
// to correctly split `self` into two slices at the given `mid` point.
unsafe impl<'a> SplitByteSlice for &'a [u8] {
    #[inline]
    unsafe fn split_at_unchecked(self, mid: usize) -> (Self, Self) {
        // SAFETY: By contract on caller, `mid` is not greater than
        // `bytes.len()`.
        unsafe { (<[u8]>::get_unchecked(self, ..mid), <[u8]>::get_unchecked(self, mid..)) }
    }
}

impl<'a> IntoByteSlice<'a> for &'a [u8] {}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> ByteSlice for &'a mut [u8] {}

// SAFETY: This delegates to `polyfills:split_at_mut_unchecked`, which is
// documented to correctly split `self` into two slices at the given `mid`
// point.
unsafe impl<'a> SplitByteSlice for &'a mut [u8] {
    #[inline]
    unsafe fn split_at_unchecked(self, mid: usize) -> (Self, Self) {
        use core::slice::from_raw_parts_mut;

        // `l_ptr` is non-null, because `self` is non-null, by invariant on
        // `&mut [u8]`.
        let l_ptr = self.as_mut_ptr();

        // SAFETY: By contract on caller, `mid` is not greater than
        // `self.len()`.
        let r_ptr = unsafe { l_ptr.add(mid) };

        let l_len = mid;

        // SAFETY: By contract on caller, `mid` is not greater than
        // `self.len()`.
        //
        // TODO(#67): Remove this allow. See NumExt for more details.
        #[allow(unstable_name_collisions, clippy::incompatible_msrv)]
        let r_len = unsafe { self.len().unchecked_sub(mid) };

        // SAFETY: These invocations of `from_raw_parts_mut` satisfy its
        // documented safety preconditions [1]:
        // - The data `l_ptr` and `r_ptr` are valid for both reads and writes of
        //   `l_len` and `r_len` bytes, respectively, and they are trivially
        //   aligned. In particular:
        //   - The entire memory range of each slice is contained within a
        //     single allocated object, since `l_ptr` and `r_ptr` are both
        //     derived from within the address range of `self`.
        //   - Both `l_ptr` and `r_ptr` are non-null and trivially aligned.
        //     `self` is non-null by invariant on `&mut [u8]`, and the
        //     operations that derive `l_ptr` and `r_ptr` from `self` do not
        //     nullify either pointer.
        // - The data `l_ptr` and `r_ptr` point to `l_len` and `r_len`,
        //   respectively, consecutive properly initialized values of type `u8`.
        //   This is true for `self` by invariant on `&mut [u8]`, and remains
        //   true for these two sub-slices of `self`.
        // - The memory referenced by the returned slice cannot be accessed
        //   through any other pointer (not derived from the return value) for
        //   the duration of lifetime `'a``, because:
        //   - `split_at_unchecked` consumes `self` (which is not `Copy`),
        //   - `split_at_unchecked` does not exfiltrate any references to this
        //     memory, besides those references returned below,
        //   - the returned slices are non-overlapping.
        // - The individual sizes of the sub-slices of `self` are no larger than
        //   `isize::MAX`, because their combined sizes are no larger than
        //   `isize::MAX`, by invariant on `self`.
        //
        // [1] https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html#safety
        unsafe { (from_raw_parts_mut(l_ptr, l_len), from_raw_parts_mut(r_ptr, r_len)) }
    }
}

impl<'a> IntoByteSliceMut<'a> for &'a mut [u8] {}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> ByteSlice for cell::Ref<'a, [u8]> {}

// SAFETY: This delegates to stdlib implementation of `Ref::map_split`, which is
// assumed to be correct, and `SplitByteSlice::split_at_unchecked`, which is
// documented to correctly split `self` into two slices at the given `mid`
// point.
unsafe impl<'a> SplitByteSlice for cell::Ref<'a, [u8]> {
    #[inline]
    unsafe fn split_at_unchecked(self, mid: usize) -> (Self, Self) {
        cell::Ref::map_split(self, |slice|
            // SAFETY: By precondition on caller, `mid` is not greater than
            // `slice.len()`.
            unsafe {
                SplitByteSlice::split_at_unchecked(slice, mid)
            })
    }
}

// TODO(#429): Add a "SAFETY" comment and remove this `allow`.
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<'a> ByteSlice for RefMut<'a, [u8]> {}

// SAFETY: This delegates to stdlib implementation of `RefMut::map_split`, which
// is assumed to be correct, and `SplitByteSlice::split_at_unchecked`, which is
// documented to correctly split `self` into two slices at the given `mid`
// point.
unsafe impl<'a> SplitByteSlice for RefMut<'a, [u8]> {
    #[inline]
    unsafe fn split_at_unchecked(self, mid: usize) -> (Self, Self) {
        RefMut::map_split(self, |slice|
            // SAFETY: By precondition on caller, `mid` is not greater than
            // `slice.len()`
            unsafe {
                SplitByteSlice::split_at_unchecked(slice, mid)
            })
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
mod alloc_support {
    use super::*;

    /// Extends a `Vec<T>` by pushing `additional` new items onto the end of the
    /// vector. The new items are initialized with zeros.
    ///
    /// # Panics
    ///
    /// Panics if `Vec::reserve(additional)` fails to reserve enough memory.
    #[inline(always)]
    pub fn extend_vec_zeroed<T: FromZeros>(v: &mut Vec<T>, additional: usize) {
        insert_vec_zeroed(v, v.len(), additional);
    }

    /// Inserts `additional` new items into `Vec<T>` at `position`.
    /// The new items are initialized with zeros.
    ///
    /// # Panics
    ///
    /// * Panics if `position > v.len()`.
    /// * Panics if `Vec::reserve(additional)` fails to reserve enough memory.
    #[inline]
    pub fn insert_vec_zeroed<T: FromZeros>(v: &mut Vec<T>, position: usize, additional: usize) {
        assert!(position <= v.len());
        v.reserve(additional);
        // SAFETY: The `reserve` call guarantees that these cannot overflow:
        // * `ptr.add(position)`
        // * `position + additional`
        // * `v.len() + additional`
        //
        // `v.len() - position` cannot overflow because we asserted that
        // `position <= v.len()`.
        unsafe {
            // This is a potentially overlapping copy.
            let ptr = v.as_mut_ptr();
            #[allow(clippy::arithmetic_side_effects)]
            ptr.add(position).copy_to(ptr.add(position + additional), v.len() - position);
            ptr.add(position).write_bytes(0, additional);
            #[allow(clippy::arithmetic_side_effects)]
            v.set_len(v.len() + additional);
        }
    }

    #[cfg(test)]
    mod tests {
        use core::convert::TryFrom as _;

        use super::*;

        #[test]
        fn test_extend_vec_zeroed() {
            // Test extending when there is an existing allocation.
            let mut v = vec![100u64, 200, 300];
            extend_vec_zeroed(&mut v, 3);
            assert_eq!(v.len(), 6);
            assert_eq!(&*v, &[100, 200, 300, 0, 0, 0]);
            drop(v);

            // Test extending when there is no existing allocation.
            let mut v: Vec<u64> = Vec::new();
            extend_vec_zeroed(&mut v, 3);
            assert_eq!(v.len(), 3);
            assert_eq!(&*v, &[0, 0, 0]);
            drop(v);
        }

        #[test]
        fn test_extend_vec_zeroed_zst() {
            // Test extending when there is an existing (fake) allocation.
            let mut v = vec![(), (), ()];
            extend_vec_zeroed(&mut v, 3);
            assert_eq!(v.len(), 6);
            assert_eq!(&*v, &[(), (), (), (), (), ()]);
            drop(v);

            // Test extending when there is no existing (fake) allocation.
            let mut v: Vec<()> = Vec::new();
            extend_vec_zeroed(&mut v, 3);
            assert_eq!(&*v, &[(), (), ()]);
            drop(v);
        }

        #[test]
        fn test_insert_vec_zeroed() {
            // Insert at start (no existing allocation).
            let mut v: Vec<u64> = Vec::new();
            insert_vec_zeroed(&mut v, 0, 2);
            assert_eq!(v.len(), 2);
            assert_eq!(&*v, &[0, 0]);
            drop(v);

            // Insert at start.
            let mut v = vec![100u64, 200, 300];
            insert_vec_zeroed(&mut v, 0, 2);
            assert_eq!(v.len(), 5);
            assert_eq!(&*v, &[0, 0, 100, 200, 300]);
            drop(v);

            // Insert at middle.
            let mut v = vec![100u64, 200, 300];
            insert_vec_zeroed(&mut v, 1, 1);
            assert_eq!(v.len(), 4);
            assert_eq!(&*v, &[100, 0, 200, 300]);
            drop(v);

            // Insert at end.
            let mut v = vec![100u64, 200, 300];
            insert_vec_zeroed(&mut v, 3, 1);
            assert_eq!(v.len(), 4);
            assert_eq!(&*v, &[100, 200, 300, 0]);
            drop(v);
        }

        #[test]
        fn test_insert_vec_zeroed_zst() {
            // Insert at start (no existing fake allocation).
            let mut v: Vec<()> = Vec::new();
            insert_vec_zeroed(&mut v, 0, 2);
            assert_eq!(v.len(), 2);
            assert_eq!(&*v, &[(), ()]);
            drop(v);

            // Insert at start.
            let mut v = vec![(), (), ()];
            insert_vec_zeroed(&mut v, 0, 2);
            assert_eq!(v.len(), 5);
            assert_eq!(&*v, &[(), (), (), (), ()]);
            drop(v);

            // Insert at middle.
            let mut v = vec![(), (), ()];
            insert_vec_zeroed(&mut v, 1, 1);
            assert_eq!(v.len(), 4);
            assert_eq!(&*v, &[(), (), (), ()]);
            drop(v);

            // Insert at end.
            let mut v = vec![(), (), ()];
            insert_vec_zeroed(&mut v, 3, 1);
            assert_eq!(v.len(), 4);
            assert_eq!(&*v, &[(), (), (), ()]);
            drop(v);
        }

        #[test]
        fn test_new_box_zeroed() {
            assert_eq!(*u64::new_box_zeroed(), 0);
        }

        #[test]
        fn test_new_box_zeroed_array() {
            drop(<[u32; 0x1000]>::new_box_zeroed());
        }

        #[test]
        fn test_new_box_zeroed_zst() {
            // This test exists in order to exercise unsafe code, especially
            // when running under Miri.
            #[allow(clippy::unit_cmp)]
            {
                assert_eq!(*<()>::new_box_zeroed(), ());
            }
        }

        #[test]
        fn test_new_box_slice_zeroed() {
            let mut s: Box<[u64]> = u64::new_box_slice_zeroed(3);
            assert_eq!(s.len(), 3);
            assert_eq!(&*s, &[0, 0, 0]);
            s[1] = 3;
            assert_eq!(&*s, &[0, 3, 0]);
        }

        #[test]
        fn test_new_box_slice_zeroed_empty() {
            let s: Box<[u64]> = u64::new_box_slice_zeroed(0);
            assert_eq!(s.len(), 0);
        }

        #[test]
        fn test_new_box_slice_zeroed_zst() {
            let mut s: Box<[()]> = <()>::new_box_slice_zeroed(3);
            assert_eq!(s.len(), 3);
            assert!(s.get(10).is_none());
            // This test exists in order to exercise unsafe code, especially
            // when running under Miri.
            #[allow(clippy::unit_cmp)]
            {
                assert_eq!(s[1], ());
            }
            s[2] = ();
        }

        #[test]
        fn test_new_box_slice_zeroed_zst_empty() {
            let s: Box<[()]> = <()>::new_box_slice_zeroed(0);
            assert_eq!(s.len(), 0);
        }

        #[test]
        #[should_panic(expected = "mem::size_of::<Self>() * len overflows `usize`")]
        fn test_new_box_slice_zeroed_panics_mul_overflow() {
            let _ = u16::new_box_slice_zeroed(usize::MAX);
        }

        #[test]
        #[should_panic(expected = "assertion failed: size <= max_alloc")]
        fn test_new_box_slice_zeroed_panics_isize_overflow() {
            let max = usize::try_from(isize::MAX).unwrap();
            let _ = u16::new_box_slice_zeroed((max / mem::size_of::<u16>()) + 1);
        }
    }
}

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use alloc_support::*;

#[cfg(test)]
#[allow(clippy::assertions_on_result_states, clippy::unreadable_literal)]
mod tests {
    use static_assertions::assert_impl_all;

    use super::*;
    use crate::util::testutil::*;

    // An unsized type.
    //
    // This is used to test the custom derives of our traits. The `[u8]` type
    // gets a hand-rolled impl, so it doesn't exercise our custom derives.
    #[derive(Debug, Eq, PartialEq, FromBytes, IntoBytes, Unaligned, Immutable)]
    #[repr(transparent)]
    struct Unsized([u8]);

    impl Unsized {
        fn from_mut_slice(slc: &mut [u8]) -> &mut Unsized {
            // SAFETY: This *probably* sound - since the layouts of `[u8]` and
            // `Unsized` are the same, so are the layouts of `&mut [u8]` and
            // `&mut Unsized`. [1] Even if it turns out that this isn't actually
            // guaranteed by the language spec, we can just change this since
            // it's in test code.
            //
            // [1] https://github.com/rust-lang/unsafe-code-guidelines/issues/375
            unsafe { mem::transmute(slc) }
        }
    }

    #[test]
    fn test_known_layout() {
        // Test that `$ty` and `ManuallyDrop<$ty>` have the expected layout.
        // Test that `PhantomData<$ty>` has the same layout as `()` regardless
        // of `$ty`.
        macro_rules! test {
            ($ty:ty, $expect:expr) => {
                let expect = $expect;
                assert_eq!(<$ty as KnownLayout>::LAYOUT, expect);
                assert_eq!(<ManuallyDrop<$ty> as KnownLayout>::LAYOUT, expect);
                assert_eq!(<PhantomData<$ty> as KnownLayout>::LAYOUT, <() as KnownLayout>::LAYOUT);
            };
        }

        let layout = |offset, align, _trailing_slice_elem_size| DstLayout {
            align: NonZeroUsize::new(align).unwrap(),
            size_info: match _trailing_slice_elem_size {
                None => SizeInfo::Sized { size: offset },
                Some(elem_size) => SizeInfo::SliceDst(TrailingSliceLayout { offset, elem_size }),
            },
        };

        test!((), layout(0, 1, None));
        test!(u8, layout(1, 1, None));
        // Use `align_of` because `u64` alignment may be smaller than 8 on some
        // platforms.
        test!(u64, layout(8, mem::align_of::<u64>(), None));
        test!(AU64, layout(8, 8, None));

        test!(Option<&'static ()>, usize::LAYOUT);

        test!([()], layout(0, 1, Some(0)));
        test!([u8], layout(0, 1, Some(1)));
        test!(str, layout(0, 1, Some(1)));
    }

    #[cfg(feature = "derive")]
    #[test]
    fn test_known_layout_derive() {
        // In this and other files (`late_compile_pass.rs`,
        // `mid_compile_pass.rs`, and `struct.rs`), we test success and failure
        // modes of `derive(KnownLayout)` for the following combination of
        // properties:
        //
        // +------------+--------------------------------------+-----------+
        // |            |      trailing field properties       |           |
        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |------------+----------+----------------+----------+-----------|
        // |          N |        N |              N |        N |      KL00 |
        // |          N |        N |              N |        Y |      KL01 |
        // |          N |        N |              Y |        N |      KL02 |
        // |          N |        N |              Y |        Y |      KL03 |
        // |          N |        Y |              N |        N |      KL04 |
        // |          N |        Y |              N |        Y |      KL05 |
        // |          N |        Y |              Y |        N |      KL06 |
        // |          N |        Y |              Y |        Y |      KL07 |
        // |          Y |        N |              N |        N |      KL08 |
        // |          Y |        N |              N |        Y |      KL09 |
        // |          Y |        N |              Y |        N |      KL10 |
        // |          Y |        N |              Y |        Y |      KL11 |
        // |          Y |        Y |              N |        N |      KL12 |
        // |          Y |        Y |              N |        Y |      KL13 |
        // |          Y |        Y |              Y |        N |      KL14 |
        // |          Y |        Y |              Y |        Y |      KL15 |
        // +------------+----------+----------------+----------+-----------+

        struct NotKnownLayout<T = ()> {
            _t: T,
        }

        #[derive(KnownLayout)]
        #[repr(C)]
        struct AlignSize<const ALIGN: usize, const SIZE: usize>
        where
            elain::Align<ALIGN>: elain::Alignment,
        {
            _align: elain::Align<ALIGN>,
            size: [u8; SIZE],
        }

        type AU16 = AlignSize<2, 2>;
        type AU32 = AlignSize<4, 4>;

        fn _assert_kl<T: ?Sized + KnownLayout>(_: &T) {}

        let sized_layout = |align, size| DstLayout {
            align: NonZeroUsize::new(align).unwrap(),
            size_info: SizeInfo::Sized { size },
        };

        let unsized_layout = |align, elem_size, offset| DstLayout {
            align: NonZeroUsize::new(align).unwrap(),
            size_info: SizeInfo::SliceDst(TrailingSliceLayout { offset, elem_size }),
        };

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          N |        N |              N |        Y |      KL01 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        struct KL01(NotKnownLayout<AU32>, NotKnownLayout<AU16>);

        let expected = DstLayout::for_type::<KL01>();

        assert_eq!(<KL01 as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL01 as KnownLayout>::LAYOUT, sized_layout(4, 8));

        // ...with `align(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(align(64))]
        struct KL01Align(NotKnownLayout<AU32>, NotKnownLayout<AU16>);

        let expected = DstLayout::for_type::<KL01Align>();

        assert_eq!(<KL01Align as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL01Align as KnownLayout>::LAYOUT, sized_layout(64, 64));

        // ...with `packed`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(packed)]
        struct KL01Packed(NotKnownLayout<AU32>, NotKnownLayout<AU16>);

        let expected = DstLayout::for_type::<KL01Packed>();

        assert_eq!(<KL01Packed as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL01Packed as KnownLayout>::LAYOUT, sized_layout(1, 6));

        // ...with `packed(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(packed(2))]
        struct KL01PackedN(NotKnownLayout<AU32>, NotKnownLayout<AU16>);

        assert_impl_all!(KL01PackedN: KnownLayout);

        let expected = DstLayout::for_type::<KL01PackedN>();

        assert_eq!(<KL01PackedN as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL01PackedN as KnownLayout>::LAYOUT, sized_layout(2, 6));

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          N |        N |              Y |        Y |      KL03 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        struct KL03(NotKnownLayout, u8);

        let expected = DstLayout::for_type::<KL03>();

        assert_eq!(<KL03 as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL03 as KnownLayout>::LAYOUT, sized_layout(1, 1));

        // ... with `align(N)`
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(align(64))]
        struct KL03Align(NotKnownLayout<AU32>, u8);

        let expected = DstLayout::for_type::<KL03Align>();

        assert_eq!(<KL03Align as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL03Align as KnownLayout>::LAYOUT, sized_layout(64, 64));

        // ... with `packed`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(packed)]
        struct KL03Packed(NotKnownLayout<AU32>, u8);

        let expected = DstLayout::for_type::<KL03Packed>();

        assert_eq!(<KL03Packed as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL03Packed as KnownLayout>::LAYOUT, sized_layout(1, 5));

        // ... with `packed(N)`
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(packed(2))]
        struct KL03PackedN(NotKnownLayout<AU32>, u8);

        assert_impl_all!(KL03PackedN: KnownLayout);

        let expected = DstLayout::for_type::<KL03PackedN>();

        assert_eq!(<KL03PackedN as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL03PackedN as KnownLayout>::LAYOUT, sized_layout(2, 6));

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          N |        Y |              N |        Y |      KL05 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        struct KL05<T>(u8, T);

        fn _test_kl05<T>(t: T) -> impl KnownLayout {
            KL05(0u8, t)
        }

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          N |        Y |              Y |        Y |      KL07 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        struct KL07<T: KnownLayout>(u8, T);

        fn _test_kl07<T: KnownLayout>(t: T) -> impl KnownLayout {
            let _ = KL07(0u8, t);
        }

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          Y |        N |              Y |        N |      KL10 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C)]
        struct KL10(NotKnownLayout<AU32>, [u8]);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU32>>(), None)
            .extend(<[u8] as KnownLayout>::LAYOUT, None)
            .pad_to_align();

        assert_eq!(<KL10 as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL10 as KnownLayout>::LAYOUT, unsized_layout(4, 1, 4));

        // ...with `align(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, align(64))]
        struct KL10Align(NotKnownLayout<AU32>, [u8]);

        let repr_align = NonZeroUsize::new(64);

        let expected = DstLayout::new_zst(repr_align)
            .extend(DstLayout::for_type::<NotKnownLayout<AU32>>(), None)
            .extend(<[u8] as KnownLayout>::LAYOUT, None)
            .pad_to_align();

        assert_eq!(<KL10Align as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL10Align as KnownLayout>::LAYOUT, unsized_layout(64, 1, 4));

        // ...with `packed`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, packed)]
        struct KL10Packed(NotKnownLayout<AU32>, [u8]);

        let repr_packed = NonZeroUsize::new(1);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU32>>(), repr_packed)
            .extend(<[u8] as KnownLayout>::LAYOUT, repr_packed)
            .pad_to_align();

        assert_eq!(<KL10Packed as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL10Packed as KnownLayout>::LAYOUT, unsized_layout(1, 1, 4));

        // ...with `packed(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, packed(2))]
        struct KL10PackedN(NotKnownLayout<AU32>, [u8]);

        let repr_packed = NonZeroUsize::new(2);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU32>>(), repr_packed)
            .extend(<[u8] as KnownLayout>::LAYOUT, repr_packed)
            .pad_to_align();

        assert_eq!(<KL10PackedN as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL10PackedN as KnownLayout>::LAYOUT, unsized_layout(2, 1, 4));

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          Y |        N |              Y |        Y |      KL11 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C)]
        struct KL11(NotKnownLayout<AU64>, u8);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU64>>(), None)
            .extend(<u8 as KnownLayout>::LAYOUT, None)
            .pad_to_align();

        assert_eq!(<KL11 as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL11 as KnownLayout>::LAYOUT, sized_layout(8, 16));

        // ...with `align(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, align(64))]
        struct KL11Align(NotKnownLayout<AU64>, u8);

        let repr_align = NonZeroUsize::new(64);

        let expected = DstLayout::new_zst(repr_align)
            .extend(DstLayout::for_type::<NotKnownLayout<AU64>>(), None)
            .extend(<u8 as KnownLayout>::LAYOUT, None)
            .pad_to_align();

        assert_eq!(<KL11Align as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL11Align as KnownLayout>::LAYOUT, sized_layout(64, 64));

        // ...with `packed`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, packed)]
        struct KL11Packed(NotKnownLayout<AU64>, u8);

        let repr_packed = NonZeroUsize::new(1);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU64>>(), repr_packed)
            .extend(<u8 as KnownLayout>::LAYOUT, repr_packed)
            .pad_to_align();

        assert_eq!(<KL11Packed as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL11Packed as KnownLayout>::LAYOUT, sized_layout(1, 9));

        // ...with `packed(N)`:
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C, packed(2))]
        struct KL11PackedN(NotKnownLayout<AU64>, u8);

        let repr_packed = NonZeroUsize::new(2);

        let expected = DstLayout::new_zst(None)
            .extend(DstLayout::for_type::<NotKnownLayout<AU64>>(), repr_packed)
            .extend(<u8 as KnownLayout>::LAYOUT, repr_packed)
            .pad_to_align();

        assert_eq!(<KL11PackedN as KnownLayout>::LAYOUT, expected);
        assert_eq!(<KL11PackedN as KnownLayout>::LAYOUT, sized_layout(2, 10));

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          Y |        Y |              Y |        N |      KL14 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C)]
        struct KL14<T: ?Sized + KnownLayout>(u8, T);

        fn _test_kl14<T: ?Sized + KnownLayout>(kl: &KL14<T>) {
            _assert_kl(kl)
        }

        // | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
        // |          Y |        Y |              Y |        Y |      KL15 |
        #[allow(dead_code)]
        #[derive(KnownLayout)]
        #[repr(C)]
        struct KL15<T: KnownLayout>(u8, T);

        fn _test_kl15<T: KnownLayout>(t: T) -> impl KnownLayout {
            let _ = KL15(0u8, t);
        }

        // Test a variety of combinations of field types:
        //  - ()
        //  - u8
        //  - AU16
        //  - [()]
        //  - [u8]
        //  - [AU16]

        #[allow(clippy::upper_case_acronyms, dead_code)]
        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLTU<T, U: ?Sized>(T, U);

        assert_eq!(<KLTU<(), ()> as KnownLayout>::LAYOUT, sized_layout(1, 0));

        assert_eq!(<KLTU<(), u8> as KnownLayout>::LAYOUT, sized_layout(1, 1));

        assert_eq!(<KLTU<(), AU16> as KnownLayout>::LAYOUT, sized_layout(2, 2));

        assert_eq!(<KLTU<(), [()]> as KnownLayout>::LAYOUT, unsized_layout(1, 0, 0));

        assert_eq!(<KLTU<(), [u8]> as KnownLayout>::LAYOUT, unsized_layout(1, 1, 0));

        assert_eq!(<KLTU<(), [AU16]> as KnownLayout>::LAYOUT, unsized_layout(2, 2, 0));

        assert_eq!(<KLTU<u8, ()> as KnownLayout>::LAYOUT, sized_layout(1, 1));

        assert_eq!(<KLTU<u8, u8> as KnownLayout>::LAYOUT, sized_layout(1, 2));

        assert_eq!(<KLTU<u8, AU16> as KnownLayout>::LAYOUT, sized_layout(2, 4));

        assert_eq!(<KLTU<u8, [()]> as KnownLayout>::LAYOUT, unsized_layout(1, 0, 1));

        assert_eq!(<KLTU<u8, [u8]> as KnownLayout>::LAYOUT, unsized_layout(1, 1, 1));

        assert_eq!(<KLTU<u8, [AU16]> as KnownLayout>::LAYOUT, unsized_layout(2, 2, 2));

        assert_eq!(<KLTU<AU16, ()> as KnownLayout>::LAYOUT, sized_layout(2, 2));

        assert_eq!(<KLTU<AU16, u8> as KnownLayout>::LAYOUT, sized_layout(2, 4));

        assert_eq!(<KLTU<AU16, AU16> as KnownLayout>::LAYOUT, sized_layout(2, 4));

        assert_eq!(<KLTU<AU16, [()]> as KnownLayout>::LAYOUT, unsized_layout(2, 0, 2));

        assert_eq!(<KLTU<AU16, [u8]> as KnownLayout>::LAYOUT, unsized_layout(2, 1, 2));

        assert_eq!(<KLTU<AU16, [AU16]> as KnownLayout>::LAYOUT, unsized_layout(2, 2, 2));

        // Test a variety of field counts.

        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLF0;

        assert_eq!(<KLF0 as KnownLayout>::LAYOUT, sized_layout(1, 0));

        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLF1([u8]);

        assert_eq!(<KLF1 as KnownLayout>::LAYOUT, unsized_layout(1, 1, 0));

        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLF2(NotKnownLayout<u8>, [u8]);

        assert_eq!(<KLF2 as KnownLayout>::LAYOUT, unsized_layout(1, 1, 1));

        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLF3(NotKnownLayout<u8>, NotKnownLayout<AU16>, [u8]);

        assert_eq!(<KLF3 as KnownLayout>::LAYOUT, unsized_layout(2, 1, 4));

        #[derive(KnownLayout)]
        #[repr(C)]
        struct KLF4(NotKnownLayout<u8>, NotKnownLayout<AU16>, NotKnownLayout<AU32>, [u8]);

        assert_eq!(<KLF4 as KnownLayout>::LAYOUT, unsized_layout(4, 1, 8));
    }

    #[test]
    fn test_object_safety() {
        fn _takes_no_cell(_: &dyn Immutable) {}
        fn _takes_unaligned(_: &dyn Unaligned) {}
    }

    #[test]
    fn test_from_zeros_only() {
        // Test types that implement `FromZeros` but not `FromBytes`.

        assert!(!bool::new_zeroed());
        assert_eq!(char::new_zeroed(), '\0');

        #[cfg(feature = "alloc")]
        {
            assert_eq!(bool::new_box_zeroed(), Box::new(false));
            assert_eq!(char::new_box_zeroed(), Box::new('\0'));

            assert_eq!(bool::new_box_slice_zeroed(3).as_ref(), [false, false, false]);
            assert_eq!(char::new_box_slice_zeroed(3).as_ref(), ['\0', '\0', '\0']);

            assert_eq!(bool::new_vec_zeroed(3).as_ref(), [false, false, false]);
            assert_eq!(char::new_vec_zeroed(3).as_ref(), ['\0', '\0', '\0']);
        }

        let mut string = "hello".to_string();
        let s: &mut str = string.as_mut();
        assert_eq!(s, "hello");
        s.zero();
        assert_eq!(s, "\0\0\0\0\0");
    }

    #[test]
    fn test_read_write() {
        const VAL: u64 = 0x12345678;
        #[cfg(target_endian = "big")]
        const VAL_BYTES: [u8; 8] = VAL.to_be_bytes();
        #[cfg(target_endian = "little")]
        const VAL_BYTES: [u8; 8] = VAL.to_le_bytes();

        // Test `FromBytes::{read_from, read_from_prefix, read_from_suffix}`.

        assert_eq!(u64::read_from(&VAL_BYTES[..]), Ok(VAL));
        // The first 8 bytes are from `VAL_BYTES` and the second 8 bytes are all
        // zeros.
        let bytes_with_prefix: [u8; 16] = transmute!([VAL_BYTES, [0; 8]]);
        assert_eq!(u64::read_from_prefix(&bytes_with_prefix[..]), Ok(VAL));
        assert_eq!(u64::read_from_suffix(&bytes_with_prefix[..]), Ok(0));
        // The first 8 bytes are all zeros and the second 8 bytes are from
        // `VAL_BYTES`
        let bytes_with_suffix: [u8; 16] = transmute!([[0; 8], VAL_BYTES]);
        assert_eq!(u64::read_from_prefix(&bytes_with_suffix[..]), Ok(0));
        assert_eq!(u64::read_from_suffix(&bytes_with_suffix[..]), Ok(VAL));

        // Test `IntoBytes::{write_to, write_to_prefix, write_to_suffix}`.

        let mut bytes = [0u8; 8];
        assert_eq!(VAL.write_to(&mut bytes[..]), Ok(()));
        assert_eq!(bytes, VAL_BYTES);
        let mut bytes = [0u8; 16];
        assert_eq!(VAL.write_to_prefix(&mut bytes[..]), Ok(()));
        let want: [u8; 16] = transmute!([VAL_BYTES, [0; 8]]);
        assert_eq!(bytes, want);
        let mut bytes = [0u8; 16];
        assert_eq!(VAL.write_to_suffix(&mut bytes[..]), Ok(()));
        let want: [u8; 16] = transmute!([[0; 8], VAL_BYTES]);
        assert_eq!(bytes, want);
    }

    #[test]
    fn test_try_from_bytes_try_read_from() {
        assert_eq!(<bool as TryFromBytes>::try_read_from(&[0]), Ok(false));
        assert_eq!(<bool as TryFromBytes>::try_read_from(&[1]), Ok(true));

        // If we don't pass enough bytes, it fails.
        assert!(matches!(<u8 as TryFromBytes>::try_read_from(&[]), Err(TryReadError::Size(_))));

        // If we pass too many bytes, it fails.
        assert!(matches!(<u8 as TryFromBytes>::try_read_from(&[0, 0]), Err(TryReadError::Size(_))));

        // If we pass an invalid value, it fails.
        assert!(matches!(
            <bool as TryFromBytes>::try_read_from(&[2]),
            Err(TryReadError::Validity(_))
        ));

        // Reading from a misaligned buffer should still succeed. Since `AU64`'s
        // alignment is 8, and since we read from two adjacent addresses one
        // byte apart, it is guaranteed that at least one of them (though
        // possibly both) will be misaligned.
        let bytes: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(<AU64 as TryFromBytes>::try_read_from(&bytes[..8]), Ok(AU64(0)));
        assert_eq!(<AU64 as TryFromBytes>::try_read_from(&bytes[1..9]), Ok(AU64(0)));
    }

    #[test]
    fn test_transmute() {
        // Test that memory is transmuted as expected.
        let array_of_u8s = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let array_of_arrays = [[0, 1], [2, 3], [4, 5], [6, 7]];
        let x: [[u8; 2]; 4] = transmute!(array_of_u8s);
        assert_eq!(x, array_of_arrays);
        let x: [u8; 8] = transmute!(array_of_arrays);
        assert_eq!(x, array_of_u8s);

        // Test that the source expression's value is forgotten rather than
        // dropped.
        #[derive(IntoBytes)]
        #[repr(transparent)]
        struct PanicOnDrop(());
        impl Drop for PanicOnDrop {
            fn drop(&mut self) {
                panic!("PanicOnDrop::drop");
            }
        }
        #[allow(clippy::let_unit_value)]
        let _: () = transmute!(PanicOnDrop(()));

        // Test that `transmute!` is legal in a const context.
        const ARRAY_OF_U8S: [u8; 8] = [0u8, 1, 2, 3, 4, 5, 6, 7];
        const ARRAY_OF_ARRAYS: [[u8; 2]; 4] = [[0, 1], [2, 3], [4, 5], [6, 7]];
        const X: [[u8; 2]; 4] = transmute!(ARRAY_OF_U8S);
        assert_eq!(X, ARRAY_OF_ARRAYS);

        // Test that `transmute!` works with `!Immutable` types.
        let x: usize = transmute!(UnsafeCell::new(1usize));
        assert_eq!(x, 1);
        let x: UnsafeCell<usize> = transmute!(1usize);
        assert_eq!(x.into_inner(), 1);
        let x: UnsafeCell<isize> = transmute!(UnsafeCell::new(1usize));
        assert_eq!(x.into_inner(), 1);
    }

    #[test]
    fn test_transmute_ref() {
        // Test that memory is transmuted as expected.
        let array_of_u8s = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let array_of_arrays = [[0, 1], [2, 3], [4, 5], [6, 7]];
        let x: &[[u8; 2]; 4] = transmute_ref!(&array_of_u8s);
        assert_eq!(*x, array_of_arrays);
        let x: &[u8; 8] = transmute_ref!(&array_of_arrays);
        assert_eq!(*x, array_of_u8s);

        // Test that `transmute_ref!` is legal in a const context.
        const ARRAY_OF_U8S: [u8; 8] = [0u8, 1, 2, 3, 4, 5, 6, 7];
        const ARRAY_OF_ARRAYS: [[u8; 2]; 4] = [[0, 1], [2, 3], [4, 5], [6, 7]];
        #[allow(clippy::redundant_static_lifetimes)]
        const X: &'static [[u8; 2]; 4] = transmute_ref!(&ARRAY_OF_U8S);
        assert_eq!(*X, ARRAY_OF_ARRAYS);

        // Test that it's legal to transmute a reference while shrinking the
        // lifetime (note that `X` has the lifetime `'static`).
        let x: &[u8; 8] = transmute_ref!(X);
        assert_eq!(*x, ARRAY_OF_U8S);

        // Test that `transmute_ref!` supports decreasing alignment.
        let u = AU64(0);
        let array = [0, 0, 0, 0, 0, 0, 0, 0];
        let x: &[u8; 8] = transmute_ref!(&u);
        assert_eq!(*x, array);

        // Test that a mutable reference can be turned into an immutable one.
        let mut x = 0u8;
        #[allow(clippy::useless_transmute)]
        let y: &u8 = transmute_ref!(&mut x);
        assert_eq!(*y, 0);
    }

    #[test]
    fn test_transmute_mut() {
        // Test that memory is transmuted as expected.
        let mut array_of_u8s = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let mut array_of_arrays = [[0, 1], [2, 3], [4, 5], [6, 7]];
        let x: &mut [[u8; 2]; 4] = transmute_mut!(&mut array_of_u8s);
        assert_eq!(*x, array_of_arrays);
        let x: &mut [u8; 8] = transmute_mut!(&mut array_of_arrays);
        assert_eq!(*x, array_of_u8s);

        {
            // Test that it's legal to transmute a reference while shrinking the
            // lifetime.
            let x: &mut [u8; 8] = transmute_mut!(&mut array_of_arrays);
            assert_eq!(*x, array_of_u8s);
        }
        // Test that `transmute_mut!` supports decreasing alignment.
        let mut u = AU64(0);
        let array = [0, 0, 0, 0, 0, 0, 0, 0];
        let x: &[u8; 8] = transmute_mut!(&mut u);
        assert_eq!(*x, array);

        // Test that a mutable reference can be turned into an immutable one.
        let mut x = 0u8;
        #[allow(clippy::useless_transmute)]
        let y: &u8 = transmute_mut!(&mut x);
        assert_eq!(*y, 0);
    }

    #[test]
    fn test_macros_evaluate_args_once() {
        let mut ctr = 0;
        let _: usize = transmute!({
            ctr += 1;
            0usize
        });
        assert_eq!(ctr, 1);

        let mut ctr = 0;
        let _: &usize = transmute_ref!({
            ctr += 1;
            &0usize
        });
        assert_eq!(ctr, 1);
    }

    #[test]
    fn test_include_value() {
        const AS_U32: u32 = include_value!("../testdata/include_value/data");
        assert_eq!(AS_U32, u32::from_ne_bytes([b'a', b'b', b'c', b'd']));
        const AS_I32: i32 = include_value!("../testdata/include_value/data");
        assert_eq!(AS_I32, i32::from_ne_bytes([b'a', b'b', b'c', b'd']));
    }

    #[test]
    fn test_ref_from_mut_from_error() {
        // Test `FromBytes::{ref_from, mut_from}{,_prefix,Suffix}` error cases.

        // Fail because the buffer is too large.
        let mut buf = Align::<[u8; 16], AU64>::default();
        // `buf.t` should be aligned to 8, so only the length check should fail.
        assert!(AU64::ref_from(&buf.t[..]).is_err());
        assert!(AU64::mut_from(&mut buf.t[..]).is_err());
        assert!(<[u8; 8]>::ref_from(&buf.t[..]).is_err());
        assert!(<[u8; 8]>::mut_from(&mut buf.t[..]).is_err());

        // Fail because the buffer is too small.
        let mut buf = Align::<[u8; 4], AU64>::default();
        assert!(AU64::ref_from(&buf.t[..]).is_err());
        assert!(AU64::mut_from(&mut buf.t[..]).is_err());
        assert!(<[u8; 8]>::ref_from(&buf.t[..]).is_err());
        assert!(<[u8; 8]>::mut_from(&mut buf.t[..]).is_err());
        assert!(AU64::ref_from_prefix(&buf.t[..]).is_err());
        assert!(AU64::mut_from_prefix(&mut buf.t[..]).is_err());
        assert!(AU64::ref_from_suffix(&buf.t[..]).is_err());
        assert!(AU64::mut_from_suffix(&mut buf.t[..]).is_err());
        assert!(<[u8; 8]>::ref_from_prefix(&buf.t[..]).is_err());
        assert!(<[u8; 8]>::mut_from_prefix(&mut buf.t[..]).is_err());
        assert!(<[u8; 8]>::ref_from_suffix(&buf.t[..]).is_err());
        assert!(<[u8; 8]>::mut_from_suffix(&mut buf.t[..]).is_err());

        // Fail because the alignment is insufficient.
        let mut buf = Align::<[u8; 13], AU64>::default();
        assert!(AU64::ref_from(&buf.t[1..]).is_err());
        assert!(AU64::mut_from(&mut buf.t[1..]).is_err());
        assert!(AU64::ref_from(&buf.t[1..]).is_err());
        assert!(AU64::mut_from(&mut buf.t[1..]).is_err());
        assert!(AU64::ref_from_prefix(&buf.t[1..]).is_err());
        assert!(AU64::mut_from_prefix(&mut buf.t[1..]).is_err());
        assert!(AU64::ref_from_suffix(&buf.t[..]).is_err());
        assert!(AU64::mut_from_suffix(&mut buf.t[..]).is_err());
    }

    #[test]
    fn test_to_methods() {
        /// Run a series of tests by calling `IntoBytes` methods on `t`.
        ///
        /// `bytes` is the expected byte sequence returned from `t.as_bytes()`
        /// before `t` has been modified. `post_mutation` is the expected
        /// sequence returned from `t.as_bytes()` after `t.as_mut_bytes()[0]`
        /// has had its bits flipped (by applying `^= 0xFF`).
        ///
        /// `N` is the size of `t` in bytes.
        fn test<T: FromBytes + IntoBytes + Immutable + Debug + Eq + ?Sized, const N: usize>(
            t: &mut T,
            bytes: &[u8],
            post_mutation: &T,
        ) {
            // Test that we can access the underlying bytes, and that we get the
            // right bytes and the right number of bytes.
            assert_eq!(t.as_bytes(), bytes);

            // Test that changes to the underlying byte slices are reflected in
            // the original object.
            t.as_mut_bytes()[0] ^= 0xFF;
            assert_eq!(t, post_mutation);
            t.as_mut_bytes()[0] ^= 0xFF;

            // `write_to` rejects slices that are too small or too large.
            assert!(t.write_to(&mut vec![0; N - 1][..]).is_err());
            assert!(t.write_to(&mut vec![0; N + 1][..]).is_err());

            // `write_to` works as expected.
            let mut bytes = [0; N];
            assert_eq!(t.write_to(&mut bytes[..]), Ok(()));
            assert_eq!(bytes, t.as_bytes());

            // `write_to_prefix` rejects slices that are too small.
            assert!(t.write_to_prefix(&mut vec![0; N - 1][..]).is_err());

            // `write_to_prefix` works with exact-sized slices.
            let mut bytes = [0; N];
            assert_eq!(t.write_to_prefix(&mut bytes[..]), Ok(()));
            assert_eq!(bytes, t.as_bytes());

            // `write_to_prefix` works with too-large slices, and any bytes past
            // the prefix aren't modified.
            let mut too_many_bytes = vec![0; N + 1];
            too_many_bytes[N] = 123;
            assert_eq!(t.write_to_prefix(&mut too_many_bytes[..]), Ok(()));
            assert_eq!(&too_many_bytes[..N], t.as_bytes());
            assert_eq!(too_many_bytes[N], 123);

            // `write_to_suffix` rejects slices that are too small.
            assert!(t.write_to_suffix(&mut vec![0; N - 1][..]).is_err());

            // `write_to_suffix` works with exact-sized slices.
            let mut bytes = [0; N];
            assert_eq!(t.write_to_suffix(&mut bytes[..]), Ok(()));
            assert_eq!(bytes, t.as_bytes());

            // `write_to_suffix` works with too-large slices, and any bytes
            // before the suffix aren't modified.
            let mut too_many_bytes = vec![0; N + 1];
            too_many_bytes[0] = 123;
            assert_eq!(t.write_to_suffix(&mut too_many_bytes[..]), Ok(()));
            assert_eq!(&too_many_bytes[1..], t.as_bytes());
            assert_eq!(too_many_bytes[0], 123);
        }

        #[derive(Debug, Eq, PartialEq, FromBytes, IntoBytes, Immutable)]
        #[repr(C)]
        struct Foo {
            a: u32,
            b: Wrapping<u32>,
            c: Option<NonZeroU32>,
        }

        let expected_bytes: Vec<u8> = if cfg!(target_endian = "little") {
            vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]
        } else {
            vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0]
        };
        let post_mutation_expected_a =
            if cfg!(target_endian = "little") { 0x00_00_00_FE } else { 0xFF_00_00_01 };
        test::<_, 12>(
            &mut Foo { a: 1, b: Wrapping(2), c: None },
            expected_bytes.as_bytes(),
            &Foo { a: post_mutation_expected_a, b: Wrapping(2), c: None },
        );
        test::<_, 3>(
            Unsized::from_mut_slice(&mut [1, 2, 3]),
            &[1, 2, 3],
            Unsized::from_mut_slice(&mut [0xFE, 2, 3]),
        );
    }

    #[test]
    fn test_array() {
        #[derive(FromBytes, IntoBytes, Immutable)]
        #[repr(C)]
        struct Foo {
            a: [u16; 33],
        }

        let foo = Foo { a: [0xFFFF; 33] };
        let expected = [0xFFu8; 66];
        assert_eq!(foo.as_bytes(), &expected[..]);
    }

    #[test]
    fn test_new_zeroed() {
        assert!(!bool::new_zeroed());
        assert_eq!(u64::new_zeroed(), 0);
        // This test exists in order to exercise unsafe code, especially when
        // running under Miri.
        #[allow(clippy::unit_cmp)]
        {
            assert_eq!(<()>::new_zeroed(), ());
        }
    }

    #[test]
    fn test_transparent_packed_generic_struct() {
        #[derive(IntoBytes, FromBytes, Unaligned)]
        #[repr(transparent)]
        struct Foo<T> {
            _t: T,
            _phantom: PhantomData<()>,
        }

        assert_impl_all!(Foo<u32>: FromZeros, FromBytes, IntoBytes);
        assert_impl_all!(Foo<u8>: Unaligned);

        #[derive(IntoBytes, FromBytes, Unaligned)]
        #[repr(packed)]
        struct Bar<T, U> {
            _t: T,
            _u: U,
        }

        assert_impl_all!(Bar<u8, AU64>: FromZeros, FromBytes, IntoBytes, Unaligned);
    }

    #[test]
    fn test_impls() {
        // A type that can supply test cases for testing
        // `TryFromBytes::is_bit_valid`. All types passed to `assert_impls!`
        // must implement this trait; that macro uses it to generate runtime
        // tests for `TryFromBytes` impls.
        //
        // All `T: FromBytes` types are provided with a blanket impl. Other
        // types must implement `TryFromBytesTestable` directly (ie using
        // `impl_try_from_bytes_testable!`).
        trait TryFromBytesTestable {
            fn with_passing_test_cases<F: Fn(Box<Self>)>(f: F);
            fn with_failing_test_cases<F: Fn(&mut [u8])>(f: F);
        }

        impl<T: FromBytes> TryFromBytesTestable for T {
            fn with_passing_test_cases<F: Fn(Box<Self>)>(f: F) {
                // Test with a zeroed value.
                f(Self::new_box_zeroed());

                let ffs = {
                    let mut t = Self::new_zeroed();
                    let ptr: *mut T = &mut t;
                    // SAFETY: `T: FromBytes`
                    unsafe { ptr::write_bytes(ptr.cast::<u8>(), 0xFF, mem::size_of::<T>()) };
                    t
                };

                // Test with a value initialized with 0xFF.
                f(Box::new(ffs));
            }

            fn with_failing_test_cases<F: Fn(&mut [u8])>(_f: F) {}
        }

        macro_rules! impl_try_from_bytes_testable_for_null_pointer_optimization {
            ($($tys:ty),*) => {
                $(
                    impl TryFromBytesTestable for Option<$tys> {
                        fn with_passing_test_cases<F: Fn(Box<Self>)>(f: F) {
                            // Test with a zeroed value.
                            f(Box::new(None));
                        }

                        fn with_failing_test_cases<F: Fn(&mut [u8])>(f: F) {
                            for pos in 0..mem::size_of::<Self>() {
                                let mut bytes = [0u8; mem::size_of::<Self>()];
                                bytes[pos] = 0x01;
                                f(&mut bytes[..]);
                            }
                        }
                    }
                )*
            };
        }

        // Implements `TryFromBytesTestable`.
        macro_rules! impl_try_from_bytes_testable {
            // Base case for recursion (when the list of types has run out).
            (=> @success $($success_case:expr),* $(, @failure $($failure_case:expr),*)?) => {};
            // Implements for type(s) with no type parameters.
            ($ty:ty $(,$tys:ty)* => @success $($success_case:expr),* $(, @failure $($failure_case:expr),*)?) => {
                impl TryFromBytesTestable for $ty {
                    impl_try_from_bytes_testable!(
                        @methods     @success $($success_case),*
                                 $(, @failure $($failure_case),*)?
                    );
                }
                impl_try_from_bytes_testable!($($tys),* => @success $($success_case),* $(, @failure $($failure_case),*)?);
            };
            // Implements for multiple types with no type parameters.
            ($($($ty:ty),* => @success $($success_case:expr), * $(, @failure $($failure_case:expr),*)?;)*) => {
                $(
                    impl_try_from_bytes_testable!($($ty),* => @success $($success_case),* $(, @failure $($failure_case),*)*);
                )*
            };
            // Implements only the methods; caller must invoke this from inside
            // an impl block.
            (@methods @success $($success_case:expr),* $(, @failure $($failure_case:expr),*)?) => {
                fn with_passing_test_cases<F: Fn(Box<Self>)>(_f: F) {
                    $(
                        _f(Box::<Self>::from($success_case));//.borrow());
                    )*
                }

                fn with_failing_test_cases<F: Fn(&mut [u8])>(_f: F) {
                    $($(
                        // `unused_qualifications` is spuriously triggered on
                        // `Option::<Self>::None`.
                        #[allow(unused_qualifications)]
                        let mut case = $failure_case;//.as_mut_bytes();
                        _f(case.as_mut_bytes());
                    )*)?
                }
            };
        }

        impl_try_from_bytes_testable_for_null_pointer_optimization!(
            Box<UnsafeCell<NotZerocopy>>,
            &'static UnsafeCell<NotZerocopy>,
            &'static mut UnsafeCell<NotZerocopy>,
            NonNull<UnsafeCell<NotZerocopy>>,
            fn(),
            FnManyArgs,
            extern "C" fn(),
            ECFnManyArgs
        );

        macro_rules! bx {
            ($e:expr) => {
                Box::new($e)
            };
        }

        // Note that these impls are only for types which are not `FromBytes`.
        // `FromBytes` types are covered by a preceding blanket impl.
        impl_try_from_bytes_testable!(
            bool => @success true, false,
                    @failure 2u8, 3u8, 0xFFu8;
            char => @success '\u{0}', '\u{D7FF}', '\u{E000}', '\u{10FFFF}',
                    @failure 0xD800u32, 0xDFFFu32, 0x110000u32;
            str  => @success "", "hello", "❤️🧡💛💚💙💜",
                    @failure [0, 159, 146, 150];
            [u8] => @success vec![].into_boxed_slice(), vec![0, 1, 2].into_boxed_slice();
            NonZeroU8, NonZeroI8, NonZeroU16, NonZeroI16, NonZeroU32,
            NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128,
            NonZeroUsize, NonZeroIsize
                => @success Self::new(1).unwrap(),
                   // Doing this instead of `0` ensures that we always satisfy
                   // the size and alignment requirements of `Self` (whereas `0`
                   // may be any integer type with a different size or alignment
                   // than some `NonZeroXxx` types).
                   @failure Option::<Self>::None;
            [bool; 0] => @success [];
            [bool; 1]
                => @success [true], [false],
                   @failure [2u8], [3u8], [0xFFu8];
            [bool]
                => @success vec![true, false].into_boxed_slice(), vec![false, true].into_boxed_slice(),
                    @failure [2u8], [3u8], [0xFFu8], [0u8, 1u8, 2u8];
            Unalign<bool>
                => @success Unalign::new(false), Unalign::new(true),
                   @failure 2u8, 0xFFu8;
            ManuallyDrop<bool>
                => @success ManuallyDrop::new(false), ManuallyDrop::new(true),
                   @failure 2u8, 0xFFu8;
            ManuallyDrop<[u8]>
                => @success bx!(ManuallyDrop::new([])), bx!(ManuallyDrop::new([0u8])), bx!(ManuallyDrop::new([0u8, 1u8]));
            ManuallyDrop<[bool]>
                => @success bx!(ManuallyDrop::new([])), bx!(ManuallyDrop::new([false])), bx!(ManuallyDrop::new([false, true])),
                   @failure [2u8], [3u8], [0xFFu8], [0u8, 1u8, 2u8];
            ManuallyDrop<[UnsafeCell<u8>]>
                => @success bx!(ManuallyDrop::new([UnsafeCell::new(0)])), bx!(ManuallyDrop::new([UnsafeCell::new(0), UnsafeCell::new(1)]));
            ManuallyDrop<[UnsafeCell<bool>]>
                => @success bx!(ManuallyDrop::new([UnsafeCell::new(false)])), bx!(ManuallyDrop::new([UnsafeCell::new(false), UnsafeCell::new(true)])),
                @failure [2u8], [3u8], [0xFFu8], [0u8, 1u8, 2u8];
            Wrapping<bool>
                => @success Wrapping(false), Wrapping(true),
                    @failure 2u8, 0xFFu8;
            *const NotZerocopy
                => @success ptr::null::<NotZerocopy>(),
                   @failure [0x01; mem::size_of::<*const NotZerocopy>()];
            *mut NotZerocopy
                => @success ptr::null_mut::<NotZerocopy>(),
                   @failure [0x01; mem::size_of::<*mut NotZerocopy>()];
        );

        // Use the trick described in [1] to allow us to call methods
        // conditional on certain trait bounds.
        //
        // In all of these cases, methods return `Option<R>`, where `R` is the
        // return type of the method we're conditionally calling. The "real"
        // implementations (the ones defined in traits using `&self`) return
        // `Some`, and the default implementations (the ones defined as inherent
        // methods using `&mut self`) return `None`.
        //
        // [1] https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
        mod autoref_trick {
            use super::*;

            pub(super) struct AutorefWrapper<T: ?Sized>(pub(super) PhantomData<T>);

            pub(super) trait TestIsBitValidShared<T: ?Sized> {
                #[allow(clippy::needless_lifetimes)]
                fn test_is_bit_valid_shared<
                    'ptr,
                    A: invariant::Aliasing + invariant::AtLeast<invariant::Shared>,
                >(
                    &self,
                    candidate: Maybe<'ptr, T, A>,
                ) -> Option<bool>;
            }

            impl<T: TryFromBytes + Immutable + ?Sized> TestIsBitValidShared<T> for AutorefWrapper<T> {
                #[allow(clippy::needless_lifetimes)]
                fn test_is_bit_valid_shared<
                    'ptr,
                    A: invariant::Aliasing + invariant::AtLeast<invariant::Shared>,
                >(
                    &self,
                    candidate: Maybe<'ptr, T, A>,
                ) -> Option<bool> {
                    Some(T::is_bit_valid(candidate))
                }
            }

            pub(super) trait TestTryFromRef<T: ?Sized> {
                #[allow(clippy::needless_lifetimes)]
                fn test_try_from_ref<'bytes>(
                    &self,
                    bytes: &'bytes [u8],
                ) -> Option<Option<&'bytes T>>;

                #[allow(clippy::needless_lifetimes)]
                fn test_try_from_mut<'bytes>(
                    &self,
                    bytes: &'bytes mut [u8],
                ) -> Option<Option<&'bytes mut T>>;
            }

            impl<T: TryFromBytes + Immutable + KnownLayout + ?Sized> TestTryFromRef<T> for AutorefWrapper<T> {
                #[allow(clippy::needless_lifetimes)]
                fn test_try_from_ref<'bytes>(
                    &self,
                    bytes: &'bytes [u8],
                ) -> Option<Option<&'bytes T>> {
                    Some(T::try_ref_from(bytes).ok())
                }

                #[allow(clippy::needless_lifetimes)]
                fn test_try_from_mut<'bytes>(
                    &self,
                    bytes: &'bytes mut [u8],
                ) -> Option<Option<&'bytes mut T>> {
                    Some(T::try_mut_from(bytes).ok())
                }
            }

            pub(super) trait TestTryReadFrom<T> {
                fn test_try_read_from(&self, bytes: &[u8]) -> Option<Option<T>>;
            }

            impl<T: TryFromBytes> TestTryReadFrom<T> for AutorefWrapper<T> {
                fn test_try_read_from(&self, bytes: &[u8]) -> Option<Option<T>> {
                    Some(T::try_read_from(bytes).ok())
                }
            }

            pub(super) trait TestAsBytes<T: ?Sized> {
                #[allow(clippy::needless_lifetimes)]
                fn test_as_bytes<'slf, 't>(&'slf self, t: &'t T) -> Option<&'t [u8]>;
            }

            impl<T: IntoBytes + Immutable + ?Sized> TestAsBytes<T> for AutorefWrapper<T> {
                #[allow(clippy::needless_lifetimes)]
                fn test_as_bytes<'slf, 't>(&'slf self, t: &'t T) -> Option<&'t [u8]> {
                    Some(t.as_bytes())
                }
            }
        }

        use autoref_trick::*;

        // Asserts that `$ty` is one of a list of types which are allowed to not
        // provide a "real" implementation for `$fn_name`. Since the
        // `autoref_trick` machinery fails silently, this allows us to ensure
        // that the "default" impls are only being used for types which we
        // expect.
        //
        // Note that, since this is a runtime test, it is possible to have an
        // allowlist which is too restrictive if the function in question is
        // never called for a particular type. For example, if `as_bytes` is not
        // supported for a particular type, and so `test_as_bytes` returns
        // `None`, methods such as `test_try_from_ref` may never be called for
        // that type. As a result, it's possible that, for example, adding
        // `as_bytes` support for a type would cause other allowlist assertions
        // to fail. This means that allowlist assertion failures should not
        // automatically be taken as a sign of a bug.
        macro_rules! assert_on_allowlist {
            ($fn_name:ident($ty:ty) $(: $($tys:ty),*)?) => {{
                use core::any::TypeId;

                let allowlist: &[TypeId] = &[ $($(TypeId::of::<$tys>()),*)? ];
                let allowlist_names: &[&str] = &[ $($(stringify!($tys)),*)? ];

                let id = TypeId::of::<$ty>();
                assert!(allowlist.contains(&id), "{} is not on allowlist for {}: {:?}", stringify!($ty), stringify!($fn_name), allowlist_names);
            }};
        }

        // Asserts that `$ty` implements any `$trait` and doesn't implement any
        // `!$trait`. Note that all `$trait`s must come before any `!$trait`s.
        //
        // For `T: TryFromBytes`, uses `TryFromBytesTestable` to test success
        // and failure cases.
        macro_rules! assert_impls {
            ($ty:ty: TryFromBytes) => {
                // "Default" implementations that match the "real"
                // implementations defined in the `autoref_trick` module above.
                #[allow(unused, non_local_definitions)]
                impl AutorefWrapper<$ty> {
                    #[allow(clippy::needless_lifetimes)]
                    fn test_is_bit_valid_shared<'ptr, A: invariant::Aliasing + invariant::AtLeast<invariant::Shared>>(
                        &mut self,
                        candidate: Maybe<'ptr, $ty, A>,
                    ) -> Option<bool> {
                        assert_on_allowlist!(
                            test_is_bit_valid_shared($ty):
                            ManuallyDrop<UnsafeCell<()>>,
                            ManuallyDrop<[UnsafeCell<u8>]>,
                            ManuallyDrop<[UnsafeCell<bool>]>,
                            MaybeUninit<NotZerocopy>,
                            MaybeUninit<UnsafeCell<()>>,
                            Wrapping<UnsafeCell<()>>
                        );

                        None
                    }

                    #[allow(clippy::needless_lifetimes)]
                    fn test_try_from_ref<'bytes>(&mut self, _bytes: &'bytes [u8]) -> Option<Option<&'bytes $ty>> {
                        assert_on_allowlist!(
                            test_try_from_ref($ty):
                            ManuallyDrop<[UnsafeCell<bool>]>
                        );

                        None
                    }

                    #[allow(clippy::needless_lifetimes)]
                    fn test_try_from_mut<'bytes>(&mut self, _bytes: &'bytes mut [u8]) -> Option<Option<&'bytes mut $ty>> {
                        assert_on_allowlist!(
                            test_try_from_mut($ty):
                            ManuallyDrop<[UnsafeCell<bool>]>
                        );

                        None
                    }

                    fn test_try_read_from(&mut self, _bytes: &[u8]) -> Option<Option<&$ty>> {
                        assert_on_allowlist!(
                            test_try_read_from($ty):
                            str,
                            ManuallyDrop<[u8]>,
                            ManuallyDrop<[bool]>,
                            ManuallyDrop<[UnsafeCell<bool>]>,
                            [u8],
                            [bool]
                        );

                        None
                    }

                    fn test_as_bytes(&mut self, _t: &$ty) -> Option<&[u8]> {
                        assert_on_allowlist!(
                            test_as_bytes($ty):
                            Option<&'static UnsafeCell<NotZerocopy>>,
                            Option<&'static mut UnsafeCell<NotZerocopy>>,
                            Option<NonNull<UnsafeCell<NotZerocopy>>>,
                            Option<Box<UnsafeCell<NotZerocopy>>>,
                            Option<fn()>,
                            Option<FnManyArgs>,
                            Option<extern "C" fn()>,
                            Option<ECFnManyArgs>,
                            MaybeUninit<u8>,
                            MaybeUninit<NotZerocopy>,
                            MaybeUninit<UnsafeCell<()>>,
                            ManuallyDrop<UnsafeCell<()>>,
                            ManuallyDrop<[UnsafeCell<u8>]>,
                            ManuallyDrop<[UnsafeCell<bool>]>,
                            Wrapping<UnsafeCell<()>>,
                            *const NotZerocopy,
                            *mut NotZerocopy
                        );

                        None
                    }
                }

                <$ty as TryFromBytesTestable>::with_passing_test_cases(|mut val| {
                    // TODO(#494): These tests only get exercised for types
                    // which are `IntoBytes`. Once we implement #494, we should
                    // be able to support non-`IntoBytes` types by zeroing
                    // padding.

                    // We define `w` and `ww` since, in the case of the inherent
                    // methods, Rust thinks they're both borrowed mutably at the
                    // same time (given how we use them below). If we just
                    // defined a single `w` and used it for multiple operations,
                    // this would conflict.
                    //
                    // We `#[allow(unused_mut]` for the cases where the "real"
                    // impls are used, which take `&self`.
                    #[allow(unused_mut)]
                    let (mut w, mut ww) = (AutorefWrapper::<$ty>(PhantomData), AutorefWrapper::<$ty>(PhantomData));

                    let c = Ptr::from_ref(&*val);
                    let c = c.forget_aligned();
                    // SAFETY: TODO(#899): This is unsound. `$ty` is not
                    // necessarily `IntoBytes`, but that's the corner we've
                    // backed ourselves into by using `Ptr::from_ref`.
                    let c = unsafe { c.assume_initialized() };
                    let res = w.test_is_bit_valid_shared(c);
                    if let Some(res) = res {
                        assert!(res, "{}::is_bit_valid({:?}) (shared `Ptr`): got false, expected true", stringify!($ty), val);
                    }

                    let c = Ptr::from_mut(&mut *val);
                    let c = c.forget_aligned();
                    // SAFETY: TODO(#899): This is unsound. `$ty` is not
                    // necessarily `IntoBytes`, but that's the corner we've
                    // backed ourselves into by using `Ptr::from_ref`.
                    let c = unsafe { c.assume_initialized() };
                    let res = <$ty as TryFromBytes>::is_bit_valid(c);
                    assert!(res, "{}::is_bit_valid({:?}) (exclusive `Ptr`): got false, expected true", stringify!($ty), val);

                    // `bytes` is `Some(val.as_bytes())` if `$ty: IntoBytes +
                    // Immutable` and `None` otherwise.
                    let bytes = w.test_as_bytes(&*val);

                    // The inner closure returns
                    // `Some($ty::try_ref_from(bytes))` if `$ty: Immutable` and
                    // `None` otherwise.
                    let res = bytes.and_then(|bytes| ww.test_try_from_ref(bytes));
                    if let Some(res) = res {
                        assert!(res.is_some(), "{}::try_ref_from({:?}): got `None`, expected `Some`", stringify!($ty), val);
                    }

                    if let Some(bytes) = bytes {
                        // We need to get a mutable byte slice, and so we clone
                        // into a `Vec`. However, we also need these bytes to
                        // satisfy `$ty`'s alignment requirement, which isn't
                        // guaranteed for `Vec<u8>`. In order to get around
                        // this, we create a `Vec` which is twice as long as we
                        // need. There is guaranteed to be an aligned byte range
                        // of size `size_of_val(val)` within that range.
                        let val = &*val;
                        let size = mem::size_of_val(val);
                        let align = mem::align_of_val(val);

                        let mut vec = bytes.to_vec();
                        vec.extend(bytes);
                        let slc = vec.as_slice();
                        let offset = slc.as_ptr().align_offset(align);
                        let bytes_mut = &mut vec.as_mut_slice()[offset..offset+size];
                        bytes_mut.copy_from_slice(bytes);

                        let res = ww.test_try_from_mut(bytes_mut);
                        if let Some(res) = res {
                            assert!(res.is_some(), "{}::try_mut_from({:?}): got `None`, expected `Some`", stringify!($ty), val);
                        }
                    }

                    let res = bytes.and_then(|bytes| ww.test_try_read_from(bytes));
                    if let Some(res) = res {
                        assert!(res.is_some(), "{}::try_read_from({:?}): got `None`, expected `Some`", stringify!($ty), val);
                    }
                });
                #[allow(clippy::as_conversions)]
                <$ty as TryFromBytesTestable>::with_failing_test_cases(|c| {
                    #[allow(unused_mut)] // For cases where the "real" impls are used, which take `&self`.
                    let mut w = AutorefWrapper::<$ty>(PhantomData);

                    // This is `Some($ty::try_ref_from(c))` if `$ty: Immutable` and
                    // `None` otherwise.
                    let res = w.test_try_from_ref(c);
                    if let Some(res) = res {
                        assert!(res.is_none(), "{}::try_ref_from({:?}): got Some, expected None", stringify!($ty), c);
                    }

                    let res = w.test_try_from_mut(c);
                    if let Some(res) = res {
                        assert!(res.is_none(), "{}::try_mut_from({:?}): got Some, expected None", stringify!($ty), c);
                    }

                    let res = w.test_try_read_from(c);
                    if let Some(res) = res {
                        assert!(res.is_none(), "{}::try_read_from({:?}): got Some, expected None", stringify!($ty), c);
                    }
                });

                #[allow(dead_code)]
                const _: () = { static_assertions::assert_impl_all!($ty: TryFromBytes); };
            };
            ($ty:ty: $trait:ident) => {
                #[allow(dead_code)]
                const _: () = { static_assertions::assert_impl_all!($ty: $trait); };
            };
            ($ty:ty: !$trait:ident) => {
                #[allow(dead_code)]
                const _: () = { static_assertions::assert_not_impl_any!($ty: $trait); };
            };
            ($ty:ty: $($trait:ident),* $(,)? $(!$negative_trait:ident),*) => {
                $(
                    assert_impls!($ty: $trait);
                )*

                $(
                    assert_impls!($ty: !$negative_trait);
                )*
            };
        }

        // NOTE: The negative impl assertions here are not necessarily
        // prescriptive. They merely serve as change detectors to make sure
        // we're aware of what trait impls are getting added with a given
        // change. Of course, some impls would be invalid (e.g., `bool:
        // FromBytes`), and so this change detection is very important.

        assert_impls!(
            (): KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned
        );
        assert_impls!(
            u8: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned
        );
        assert_impls!(
            i8: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned
        );
        assert_impls!(
            u16: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            i16: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            u32: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            i32: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            u64: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            i64: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            u128: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            i128: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            usize: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            isize: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            f32: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );
        assert_impls!(
            f64: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            !Unaligned
        );

        assert_impls!(
            bool: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            IntoBytes,
            Unaligned,
            !FromBytes
        );
        assert_impls!(
            char: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            str: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            IntoBytes,
            Unaligned,
            !FromBytes
        );

        assert_impls!(
            NonZeroU8: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            Unaligned,
            !FromZeros,
            !FromBytes
        );
        assert_impls!(
            NonZeroI8: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            Unaligned,
            !FromZeros,
            !FromBytes
        );
        assert_impls!(
            NonZeroU16: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroI16: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroU32: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroI32: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroU64: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroI64: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroU128: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroI128: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroUsize: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );
        assert_impls!(
            NonZeroIsize: KnownLayout,
            Immutable,
            TryFromBytes,
            IntoBytes,
            !FromBytes,
            !Unaligned
        );

        assert_impls!(Option<NonZeroU8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        assert_impls!(Option<NonZeroI8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        assert_impls!(Option<NonZeroU16>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroI16>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroU32>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroI32>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroU64>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroI64>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroU128>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroI128>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroUsize>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);
        assert_impls!(Option<NonZeroIsize>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned);

        // Implements none of the ZC traits.
        struct NotZerocopy;

        #[rustfmt::skip]
        type FnManyArgs = fn(
            NotZerocopy, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8,
        ) -> (NotZerocopy, NotZerocopy);

        // Allowed, because we're not actually using this type for FFI.
        #[allow(improper_ctypes_definitions)]
        #[rustfmt::skip]
        type ECFnManyArgs = extern "C" fn(
            NotZerocopy, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8,
        ) -> (NotZerocopy, NotZerocopy);

        #[cfg(feature = "alloc")]
        assert_impls!(Option<Box<UnsafeCell<NotZerocopy>>>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<Box<[UnsafeCell<NotZerocopy>]>>: KnownLayout, !Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<&'static UnsafeCell<NotZerocopy>>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<&'static [UnsafeCell<NotZerocopy>]>: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<&'static mut UnsafeCell<NotZerocopy>>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<&'static mut [UnsafeCell<NotZerocopy>]>: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<NonNull<UnsafeCell<NotZerocopy>>>: KnownLayout, TryFromBytes, FromZeros, Immutable, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<NonNull<[UnsafeCell<NotZerocopy>]>>: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<fn()>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<FnManyArgs>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<extern "C" fn()>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Option<ECFnManyArgs>: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);

        assert_impls!(PhantomData<NotZerocopy>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        assert_impls!(PhantomData<UnsafeCell<()>>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        assert_impls!(PhantomData<[u8]>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);

        assert_impls!(ManuallyDrop<u8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        // This test is important because it allows us to test our hand-rolled
        // implementation of `<ManuallyDrop<T> as TryFromBytes>::is_bit_valid`.
        assert_impls!(ManuallyDrop<bool>: KnownLayout, Immutable, TryFromBytes, FromZeros, IntoBytes, Unaligned, !FromBytes);
        assert_impls!(ManuallyDrop<[u8]>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        // This test is important because it allows us to test our hand-rolled
        // implementation of `<ManuallyDrop<T> as TryFromBytes>::is_bit_valid`.
        assert_impls!(ManuallyDrop<[bool]>: KnownLayout, Immutable, TryFromBytes, FromZeros, IntoBytes, Unaligned, !FromBytes);
        assert_impls!(ManuallyDrop<NotZerocopy>: !Immutable, !TryFromBytes, !KnownLayout, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(ManuallyDrop<[NotZerocopy]>: KnownLayout, !Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(ManuallyDrop<UnsafeCell<()>>: KnownLayout, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned, !Immutable);
        assert_impls!(ManuallyDrop<[UnsafeCell<u8>]>: KnownLayout, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned, !Immutable);
        assert_impls!(ManuallyDrop<[UnsafeCell<bool>]>: KnownLayout, TryFromBytes, FromZeros, IntoBytes, Unaligned, !Immutable, !FromBytes);

        assert_impls!(MaybeUninit<u8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, Unaligned, !IntoBytes);
        assert_impls!(MaybeUninit<NotZerocopy>: KnownLayout, TryFromBytes, FromZeros, FromBytes, !Immutable, !IntoBytes, !Unaligned);
        assert_impls!(MaybeUninit<UnsafeCell<()>>: KnownLayout, TryFromBytes, FromZeros, FromBytes, Unaligned, !Immutable, !IntoBytes);

        assert_impls!(Wrapping<u8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        // This test is important because it allows us to test our hand-rolled
        // implementation of `<Wrapping<T> as TryFromBytes>::is_bit_valid`.
        assert_impls!(Wrapping<bool>: KnownLayout, Immutable, TryFromBytes, FromZeros, IntoBytes, Unaligned, !FromBytes);
        assert_impls!(Wrapping<NotZerocopy>: KnownLayout, !Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(Wrapping<UnsafeCell<()>>: KnownLayout, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned, !Immutable);

        assert_impls!(Unalign<u8>: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, Unaligned);
        // This test is important because it allows us to test our hand-rolled
        // implementation of `<Unalign<T> as TryFromBytes>::is_bit_valid`.
        assert_impls!(Unalign<bool>: KnownLayout, Immutable, TryFromBytes, FromZeros, IntoBytes, Unaligned, !FromBytes);
        assert_impls!(Unalign<NotZerocopy>: Unaligned, !Immutable, !KnownLayout, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes);

        assert_impls!(
            [u8]: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned
        );
        assert_impls!(
            [bool]: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            IntoBytes,
            Unaligned,
            !FromBytes
        );
        assert_impls!([NotZerocopy]: KnownLayout, !Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(
            [u8; 0]: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned,
        );
        assert_impls!(
            [NotZerocopy; 0]: KnownLayout,
            !Immutable,
            !TryFromBytes,
            !FromZeros,
            !FromBytes,
            !IntoBytes,
            !Unaligned
        );
        assert_impls!(
            [u8; 1]: KnownLayout,
            Immutable,
            TryFromBytes,
            FromZeros,
            FromBytes,
            IntoBytes,
            Unaligned,
        );
        assert_impls!(
            [NotZerocopy; 1]: KnownLayout,
            !Immutable,
            !TryFromBytes,
            !FromZeros,
            !FromBytes,
            !IntoBytes,
            !Unaligned
        );

        assert_impls!(*const NotZerocopy: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(*mut NotZerocopy: KnownLayout, Immutable, TryFromBytes, FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(*const [NotZerocopy]: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(*mut [NotZerocopy]: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(*const dyn Debug: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);
        assert_impls!(*mut dyn Debug: KnownLayout, Immutable, !TryFromBytes, !FromZeros, !FromBytes, !IntoBytes, !Unaligned);

        #[cfg(feature = "simd")]
        {
            #[allow(unused_macros)]
            macro_rules! test_simd_arch_mod {
                ($arch:ident, $($typ:ident),*) => {
                    {
                        use core::arch::$arch::{$($typ),*};
                        use crate::*;
                        $( assert_impls!($typ: KnownLayout, Immutable, TryFromBytes, FromZeros, FromBytes, IntoBytes, !Unaligned); )*
                    }
                };
            }
            #[cfg(target_arch = "x86")]
            test_simd_arch_mod!(x86, __m128, __m128d, __m128i, __m256, __m256d, __m256i);

            #[cfg(all(feature = "simd-nightly", target_arch = "x86"))]
            test_simd_arch_mod!(x86, __m512bh, __m512, __m512d, __m512i);

            #[cfg(target_arch = "x86_64")]
            test_simd_arch_mod!(x86_64, __m128, __m128d, __m128i, __m256, __m256d, __m256i);

            #[cfg(all(feature = "simd-nightly", target_arch = "x86_64"))]
            test_simd_arch_mod!(x86_64, __m512bh, __m512, __m512d, __m512i);

            #[cfg(target_arch = "wasm32")]
            test_simd_arch_mod!(wasm32, v128);

            #[cfg(all(feature = "simd-nightly", target_arch = "powerpc"))]
            test_simd_arch_mod!(
                powerpc,
                vector_bool_long,
                vector_double,
                vector_signed_long,
                vector_unsigned_long
            );

            #[cfg(all(feature = "simd-nightly", target_arch = "powerpc64"))]
            test_simd_arch_mod!(
                powerpc64,
                vector_bool_long,
                vector_double,
                vector_signed_long,
                vector_unsigned_long
            );
            #[cfg(all(target_arch = "aarch64", zerocopy_aarch64_simd))]
            #[rustfmt::skip]
            test_simd_arch_mod!(
                aarch64, float32x2_t, float32x4_t, float64x1_t, float64x2_t, int8x8_t, int8x8x2_t,
                int8x8x3_t, int8x8x4_t, int8x16_t, int8x16x2_t, int8x16x3_t, int8x16x4_t, int16x4_t,
                int16x8_t, int32x2_t, int32x4_t, int64x1_t, int64x2_t, poly8x8_t, poly8x8x2_t, poly8x8x3_t,
                poly8x8x4_t, poly8x16_t, poly8x16x2_t, poly8x16x3_t, poly8x16x4_t, poly16x4_t, poly16x8_t,
                poly64x1_t, poly64x2_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t, uint8x16_t,
                uint8x16x2_t, uint8x16x3_t, uint8x16x4_t, uint16x4_t, uint16x8_t, uint32x2_t, uint32x4_t,
                uint64x1_t, uint64x2_t
            );
            #[cfg(all(feature = "simd-nightly", target_arch = "arm"))]
            #[rustfmt::skip]
            test_simd_arch_mod!(arm, int8x4_t, uint8x4_t);
        }
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    impl kani::Arbitrary for DstLayout {
        fn any() -> Self {
            let align: NonZeroUsize = kani::any();
            let size_info: SizeInfo = kani::any();

            kani::assume(align.is_power_of_two());
            kani::assume(align < DstLayout::THEORETICAL_MAX_ALIGN);

            // For testing purposes, we most care about instantiations of
            // `DstLayout` that can correspond to actual Rust types. We use
            // `Layout` to verify that our `DstLayout` satisfies the validity
            // conditions of Rust layouts.
            kani::assume(
                match size_info {
                    SizeInfo::Sized { size } => Layout::from_size_align(size, align.get()),
                    SizeInfo::SliceDst(TrailingSliceLayout { offset, elem_size: _ }) => {
                        // `SliceDst`` cannot encode an exact size, but we know
                        // it is at least `offset` bytes.
                        Layout::from_size_align(offset, align.get())
                    }
                }
                .is_ok(),
            );

            Self { align: align, size_info: size_info }
        }
    }

    impl kani::Arbitrary for SizeInfo {
        fn any() -> Self {
            let is_sized: bool = kani::any();

            match is_sized {
                true => {
                    let size: usize = kani::any();

                    kani::assume(size <= isize::MAX as _);

                    SizeInfo::Sized { size }
                }
                false => SizeInfo::SliceDst(kani::any()),
            }
        }
    }

    impl kani::Arbitrary for TrailingSliceLayout {
        fn any() -> Self {
            let elem_size: usize = kani::any();
            let offset: usize = kani::any();

            kani::assume(elem_size < isize::MAX as _);
            kani::assume(offset < isize::MAX as _);

            TrailingSliceLayout { elem_size, offset }
        }
    }

    #[kani::proof]
    fn prove_dst_layout_extend() {
        use crate::util::{max, min, padding_needed_for};

        let base: DstLayout = kani::any();
        let field: DstLayout = kani::any();
        let packed: Option<NonZeroUsize> = kani::any();

        if let Some(max_align) = packed {
            kani::assume(max_align.is_power_of_two());
            kani::assume(base.align <= max_align);
        }

        // The base can only be extended if it's sized.
        kani::assume(matches!(base.size_info, SizeInfo::Sized { .. }));
        let base_size = if let SizeInfo::Sized { size } = base.size_info {
            size
        } else {
            unreachable!();
        };

        // Under the above conditions, `DstLayout::extend` will not panic.
        let composite = base.extend(field, packed);

        // The field's alignment is clamped by `max_align` (i.e., the
        // `packed` attribute, if any) [1].
        //
        // [1] Per https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers:
        //
        //   The alignments of each field, for the purpose of positioning
        //   fields, is the smaller of the specified alignment and the
        //   alignment of the field's type.
        let field_align = min(field.align, packed.unwrap_or(DstLayout::THEORETICAL_MAX_ALIGN));

        // The struct's alignment is the maximum of its previous alignment and
        // `field_align`.
        assert_eq!(composite.align, max(base.align, field_align));

        // Compute the minimum amount of inter-field padding needed to
        // satisfy the field's alignment, and offset of the trailing field.
        // [1]
        //
        // [1] Per https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers:
        //
        //   Inter-field padding is guaranteed to be the minimum required in
        //   order to satisfy each field's (possibly altered) alignment.
        let padding = padding_needed_for(base_size, field_align);
        let offset = base_size + padding;

        // For testing purposes, we'll also construct `alloc::Layout`
        // stand-ins for `DstLayout`, and show that `extend` behaves
        // comparably on both types.
        let base_analog = Layout::from_size_align(base_size, base.align.get()).unwrap();

        match field.size_info {
            SizeInfo::Sized { size: field_size } => {
                if let SizeInfo::Sized { size: composite_size } = composite.size_info {
                    // If the trailing field is sized, the resulting layout
                    // will be sized. Its size will be the sum of the
                    // preceeding layout, the size of the new field, and the
                    // size of inter-field padding between the two.
                    assert_eq!(composite_size, offset + field_size);

                    let field_analog =
                        Layout::from_size_align(field_size, field_align.get()).unwrap();

                    if let Ok((actual_composite, actual_offset)) = base_analog.extend(field_analog)
                    {
                        assert_eq!(actual_offset, offset);
                        assert_eq!(actual_composite.size(), composite_size);
                        assert_eq!(actual_composite.align(), composite.align.get());
                    } else {
                        // An error here reflects that composite of `base`
                        // and `field` cannot correspond to a real Rust type
                        // fragment, because such a fragment would violate
                        // the basic invariants of a valid Rust layout. At
                        // the time of writing, `DstLayout` is a little more
                        // permissive than `Layout`, so we don't assert
                        // anything in this branch (e.g., unreachability).
                    }
                } else {
                    panic!("The composite of two sized layouts must be sized.")
                }
            }
            SizeInfo::SliceDst(TrailingSliceLayout {
                offset: field_offset,
                elem_size: field_elem_size,
            }) => {
                if let SizeInfo::SliceDst(TrailingSliceLayout {
                    offset: composite_offset,
                    elem_size: composite_elem_size,
                }) = composite.size_info
                {
                    // The offset of the trailing slice component is the sum
                    // of the offset of the trailing field and the trailing
                    // slice offset within that field.
                    assert_eq!(composite_offset, offset + field_offset);
                    // The elem size is unchanged.
                    assert_eq!(composite_elem_size, field_elem_size);

                    let field_analog =
                        Layout::from_size_align(field_offset, field_align.get()).unwrap();

                    if let Ok((actual_composite, actual_offset)) = base_analog.extend(field_analog)
                    {
                        assert_eq!(actual_offset, offset);
                        assert_eq!(actual_composite.size(), composite_offset);
                        assert_eq!(actual_composite.align(), composite.align.get());
                    } else {
                        // An error here reflects that composite of `base`
                        // and `field` cannot correspond to a real Rust type
                        // fragment, because such a fragment would violate
                        // the basic invariants of a valid Rust layout. At
                        // the time of writing, `DstLayout` is a little more
                        // permissive than `Layout`, so we don't assert
                        // anything in this branch (e.g., unreachability).
                    }
                } else {
                    panic!("The extension of a layout with a DST must result in a DST.")
                }
            }
        }
    }

    #[kani::proof]
    #[kani::should_panic]
    fn prove_dst_layout_extend_dst_panics() {
        let base: DstLayout = kani::any();
        let field: DstLayout = kani::any();
        let packed: Option<NonZeroUsize> = kani::any();

        if let Some(max_align) = packed {
            kani::assume(max_align.is_power_of_two());
            kani::assume(base.align <= max_align);
        }

        kani::assume(matches!(base.size_info, SizeInfo::SliceDst(..)));

        let _ = base.extend(field, packed);
    }

    #[kani::proof]
    fn prove_dst_layout_pad_to_align() {
        use crate::util::padding_needed_for;

        let layout: DstLayout = kani::any();

        let padded: DstLayout = layout.pad_to_align();

        // Calling `pad_to_align` does not alter the `DstLayout`'s alignment.
        assert_eq!(padded.align, layout.align);

        if let SizeInfo::Sized { size: unpadded_size } = layout.size_info {
            if let SizeInfo::Sized { size: padded_size } = padded.size_info {
                // If the layout is sized, it will remain sized after padding is
                // added. Its sum will be its unpadded size and the size of the
                // trailing padding needed to satisfy its alignment
                // requirements.
                let padding = padding_needed_for(unpadded_size, layout.align);
                assert_eq!(padded_size, unpadded_size + padding);

                // Prove that calling `DstLayout::pad_to_align` behaves
                // identically to `Layout::pad_to_align`.
                let layout_analog =
                    Layout::from_size_align(unpadded_size, layout.align.get()).unwrap();
                let padded_analog = layout_analog.pad_to_align();
                assert_eq!(padded_analog.align(), layout.align.get());
                assert_eq!(padded_analog.size(), padded_size);
            } else {
                panic!("The padding of a sized layout must result in a sized layout.")
            }
        } else {
            // If the layout is a DST, padding cannot be statically added.
            assert_eq!(padded.size_info, layout.size_info);
        }
    }
}

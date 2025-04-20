// ---------------------------------------------------------------------------
// Imports
// ---------------------------------------------------------------------------

use vstd::prelude::*;          // Verus prelude: spec‐level macros, traits, etc.
use std::mem::ManuallyDrop;    // A std wrapper that suppresses automatic Drop
use std::ops::Deref;           // Provides the Deref trait (used below)

verus!{

// ---------------------------------------------------------------------------
// 1. A “view” abstraction for specs
// ---------------------------------------------------------------------------

/// A small helper trait used only in specifications.  
/// `view()` maps a *concrete* value into an *abstract* logical value (`Self::V`)
/// that we use when writing post‑conditions and invariants.
///
/// • `spec fn` means the function exists **only** in ghost/spec code and is
///   erased after verification.
/// • The trait itself never appears in regular compiled code.
pub trait ViewLocalCrate {
    type V;                    // Associated “view” type

    spec fn view(&self) -> Self::V;
}

// ---------------------------------------------------------------------------
// 2. Spec implementation for `ManuallyDrop<T>`
// ---------------------------------------------------------------------------

/// For any `ManuallyDrop<T>`, its logical view is a `Box<T>`
/// (we treat the raw, non‑dropping wrapper as owning a boxed value).
impl<T: ?Sized> ViewLocalCrate for ManuallyDrop<T> {
    type V = Box<T>;

    // No body: Verus only needs the *signature* for reasoning.
    spec fn view(&self) -> Self::V;
}

// ---------------------------------------------------------------------------
// 3. External spec for `ManuallyDrop::new`
// ---------------------------------------------------------------------------

/// We tell Verus *what* `ManuallyDrop::new` guarantees without re‑verifying its
/// standard‑library body.  `assume_specification` asserts the condition as an
/// axiom the verifier may rely on.
///
/// Post‑condition:
///   The freshly‑created wrapper’s `view()` equals `Box::new(v)`.
pub assume_specification<T>[ ManuallyDrop::<T>::new ](v: T) -> (a: ManuallyDrop<T>)
    ensures
        a.view() == Box::new(v)
;

// ---------------------------------------------------------------------------
// 4. External spec for `ManuallyDrop::deref`
// ---------------------------------------------------------------------------

/// Similar idea for `deref`: we give Verus the relationship between the
/// returned reference and the wrapper’s abstract view.
///
/// Post‑condition:
///   Calling `deref` yields `&T` such that wrapping that reference in `Box`
///   equals the wrapper’s current view.
pub assume_specification<T: ?Sized>[ ManuallyDrop::<T>::deref ](s: &ManuallyDrop<T>) -> (a: &T)
    ensures
        s.view() == Box::new(a)
;

} // end verus! block

// ---------------------------------------------------------------------------
// 5. Regular Rust entry point (not used for verification here).
// ---------------------------------------------------------------------------

fn main() {
    // All verification happens in the `verus! { ... }` macro above.
}

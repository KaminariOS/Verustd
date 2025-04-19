use vstd::prelude::*;
use std::cmp::Ordering;

verus!{

// ---------------------------------------------------------------------------
// 1.  External type specification for `Ordering`
// ---------------------------------------------------------------------------
// `Ordering` is defined in the standard library.  We wrap it in a phantom
// struct so Verus can reason about it at the *spec* level.
#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);

// We *assume* a spec function that tells us when an Ordering is `Greater`.
// The body is omitted; we only state the logical behaviour via `ensures`.
// Now the verifier can treat `Ordering::is_gt(v)` like a pure Boolean test.
pub assume_specification[ Ordering::is_gt](
    v: Ordering,
) -> (result: bool)
    ensures result == (v == Ordering::Greater),
;

// ---------------------------------------------------------------------------
// 2.  External trait specification for a subset of `Ord`
// ---------------------------------------------------------------------------
// We declare an *external* trait, so we can talk about `cmp` semantically
// without importing the real implementation (which may involve `unsafe` code).
#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd {
    // An associated type that must already implement the *real* `Ord`.
    type ExternalTraitSpecificationFor: core::cmp::Ord;

    // Logical signature for `cmp`; Verus assumes the usual `Ord` contract.
    fn cmp(&self, other: &Self) -> Ordering;
}

// A wrapper function that simply calls `cmp`.  This shows that once the
// external spec is in place, we can use it in our verified code.
fn compare<T: Ord>(a: T, b: &T) -> Ordering {
    a.cmp(b)
}

// ---------------------------------------------------------------------------
// 3.  External spec for `usize::saturating_sub`
// ---------------------------------------------------------------------------
// We give *three* cases describing the saturating subtraction semantics.
//   • Case 1: result underflows  -> clamp to `usize::MIN`
//   • Case 2: result overflows   -> clamp to `usize::MAX`
//   • Case 3: otherwise the true difference
pub assume_specification[ usize::saturating_sub](
    v: usize,
    rhs: usize,
) -> (result: usize)
    ensures
        // Underflow
        (v as int - rhs as int < usize::MIN as int) ==> (result == usize::MIN),
        // Overflow
        (v as int - rhs as int > usize::MAX as int) ==> (result == usize::MAX),
        // Exact subtraction when in range
        (usize::MIN as int <= v as int - rhs as int <= usize::MAX as int)
            ==> (result == v - rhs),
;

// Quick unit test: 1 − 2 saturates to 0.
fn test_sub() {
    let v = 1usize.saturating_sub(2);
    assert(v == 0);
}

// ---------------------------------------------------------------------------
// 4.  A recursive spec function: floor(log2(n))
// ---------------------------------------------------------------------------
// Demonstrates how Verus tracks termination with `decreases n`.
spec fn floor_log2(n: nat) -> nat
    decreases n
{
    if n < 2 { 0 } else { 1 + floor_log2(n / 2) }
}

} // end verus!

fn main() {}

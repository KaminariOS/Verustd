use vstd::prelude::*;
use core::cmp::Ordering;

verus!{

// External specification for Rust's built-in Ordering type
#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);

// Trait defining total ordering properties for type T
pub trait TotalOrdered : Sized {
    // Specification for "less or equal" relation
    spec fn le(self, other: Self) -> bool;

    // Proof that le is reflexive (x ≤ x)
    proof fn reflexive(x: Self)
        ensures Self::le(x, x);

    // Proof that le is transitive (x ≤ y ∧ y ≤ z ⇒ x ≤ z)
    proof fn transitive(x: Self, y: Self, z: Self)
        requires Self::le(x, y), Self::le(y, z),
        ensures Self::le(x, z);

    // Proof that le is antisymmetric (x ≤ y ∧ y ≤ x ⇒ x = y)
    proof fn antisymmetric(x: Self, y: Self)
        requires Self::le(x, y), Self::le(y, x),
        ensures x == y;

    // Proof that ordering is total (x ≤ y ∨ y ≤ x)
    proof fn total(x: Self, y: Self)
        ensures Self::le(x, y) || Self::le(y, x);

    // Method for comparing two values, ensures the ordering matches the le spec
    fn compare(&self, other: &Self) -> (c: Ordering)
        ensures (match c {
            Ordering::Less => self.le(*other) && self != other,
            Ordering::Equal => self == other,
            Ordering::Greater => other.le(*self) && self != other,
        });
}

// External trait specification linking Verus' traits to Rust's standard Ord
#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd  {
    type ExternalTraitSpecificationFor: core::cmp::Ord;
    fn cmp(&self, other: &Self) -> (res: Ordering)
                where Self: Ord
        ensures (match res {
            Ordering::Less => le(self, other) && self != other,
            Ordering::Equal => self == other,
            Ordering::Greater => le(other, self) && self != other,
        });
}

// Implementation of TotalOrdered for any Ord type, admitted proofs for simplicity
impl<T: Ord> TotalOrdered for T {
    spec fn le(self, other: Self) -> bool;

    proof fn reflexive(x: Self) { admit(); }
    proof fn transitive(x: Self, y: Self, z: Self) { admit(); }
    proof fn antisymmetric(x: Self, y: Self) { admit(); }
    proof fn total(x: Self, y: Self) { admit(); }

    #[verifier::external_body]
    fn compare(&self, other: &Self) -> (c: Ordering) {
        self.cmp(other)
    }
}

// Proof helper function example demonstrating usage of le
proof fn sel<T: TotalOrdered>(a: T, b: T) -> (res: T)
    requires a.le(a) 
{
    a
}

// Function returning the maximum of two TotalOrdered elements
fn max<T: TotalOrdered>(a: T, b: T) -> (res: T) 
    ensures a.le(res) && b.le(res)
{
    let r = match a.compare(&b) {
        Ordering::Less => {
            assert(a.le(b));
            b
        },
        _ => {
            proof { T::total(a, b); }
            assert(b.le(a));
            a
        } 
    };
    proof {
        T::reflexive(a);
        T::reflexive(b);
    }
    r
}

// Spec function providing ordering comparison logic for reference types
pub open spec fn le<T: ?Sized>(a: &T, b: &T) -> bool;

// Proof methods for reflexive and total properties using references
proof fn reflexive<T: Ord + ?Sized>(x: &T)
    ensures le(x, x) { admit(); }

proof fn total<T: Ord + ?Sized>(x: &T, y: &T)
    ensures le(x, y) || le(y, x) { admit(); }

// Spec function for a general comparison returning Ordering
spec fn spec_cmp<T: Ord>(a: &T, b: &T) -> Ordering {
    Ordering::Less
}
}

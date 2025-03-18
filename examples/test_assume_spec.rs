use vstd::prelude::*;
use std::cmp::Ordering;
verus!{

#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);


pub assume_specification[ Ordering::is_gt](
    v: Ordering,
) -> (result: bool)
    ensures
        result == (v == Ordering::Greater) ,
;

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd  {
    type ExternalTraitSpecificationFor: core::cmp::Ord;
    fn cmp(&self, other: &Self) -> Ordering;
}
    fn compare<T: Ord>(a: T, b: &T) -> Ordering {
        a.cmp(b)
    }

pub assume_specification[ usize::saturating_sub](
    v: usize,
    rhs: usize,
) -> (result: usize)
    ensures
        // Case 1: Underflow (result would be < i8::MIN)
        (v - rhs < usize::MIN) ==> (result == usize::MIN),
        // Case 2: Overflow (result would be > i8::MAX)
        (v - rhs > usize::MAX) ==> (result == usize::MAX),
        // Case 3: No overflow/underflow
        (usize::MIN <= v - rhs <= usize::MAX) ==> (result == v - rhs),
;

    fn test_sub() {
        let v = 1usize.saturating_sub(2);
        assert(v == 0);
    }

    spec fn floor_log2(n: nat) -> nat 
    decreases n
    {
        if n < 2 {
            0
        } else {
            1 + floor_log2(n / 2)
        }
    }
}



fn main(){}

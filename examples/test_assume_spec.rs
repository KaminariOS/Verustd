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

}

fn main(){}

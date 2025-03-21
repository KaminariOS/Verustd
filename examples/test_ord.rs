use vstd::prelude::*;
use core::cmp::Ordering;

verus!{

#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);

pub trait TotalOrdered : Sized {
    spec fn le(self, other: Self) -> bool;

    proof fn reflexive(x: Self)
        ensures Self::le(x, x);

    proof fn transitive(x: Self, y: Self, z: Self)
        requires Self::le(x, y), Self::le(y, z),
        ensures Self::le(x, z);

    proof fn antisymmetric(x: Self, y: Self)
        requires Self::le(x, y), Self::le(y, x),
        ensures x == y;

    proof fn total(x: Self, y: Self)
        ensures Self::le(x, y) || Self::le(y, x);

    fn compare(&self, other: &Self) -> (c: Ordering)
        ensures (match c {
            Ordering::Less => self.le(*other) && self != other,
            Ordering::Equal => self == other,
            Ordering::Greater => other.le(*self) && self != other,
        });
}

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd  {
    type ExternalTraitSpecificationFor: core::cmp::Ord;
    fn cmp(&self, other: &Self) -> (res: Ordering)
    // ensures le(self, other)
;
}

// pub closed spec fn le<T: Ord + ?Sized>(a: &T, b: &T) -> bool;



impl<T: Ord> TotalOrdered for T {
    spec fn le(self, other: Self) -> bool;
    proof fn reflexive(x: Self) {admit()}
    proof fn transitive(x: Self, y: Self, z: Self) {admit()}
    proof fn antisymmetric(x: Self, y: Self) {admit()}
    proof fn total(x: Self, y: Self) {admit()}

    #[verifier::external_body]
    fn compare(&self, other: &Self) -> (c: Ordering) 
    {
        self.cmp(other)
    }
}


proof fn sel<T: TotalOrdered>(a: T, b: T) -> (res: T)
requires a.le(a) 
{
 a
}

fn max<T: TotalOrdered>(a: T, b: T) -> (res: T) 
    ensures a.le(res) && b.le(res)
{
    let r = match a.compare(&b) 
{
    Ordering::Less => {
        assert(a.le(b));
        b
    },
    _ => 
        {
        proof {
            T::total(a, b);
        }
        assert(b.le(a));
        a} 

    };
    proof {
        T::reflexive(a);
        T::reflexive(b);
    }
    // assert(a.le(r)); 
    r
}

spec fn le<T: Ord>(a: &T, b: &T) -> bool;
spec fn spec_cmp<T: Ord>(a: &T, b: &T) -> Ordering 
        {Ordering::Less}

// #[verifier::when_used_as_spec(spec_cmp)]
// pub assume_specification<T: Ord>[ T::cmp ](
//     sel: &T,
//     other: &T  
// ) -> (result: Ordering)
//         ensures (match result {
//                     Ordering::Less => sel.le(other) && sel != other,
//                     Ordering::Equal => sel == other,
//                     Ordering::Greater => other.le(sel) && sel != other,
//                 })
// ;


// pub open spec fn iter_into_iter_spec<I: Iterator>(i: I) -> I {
//     i
// }
//
// #[verifier::when_used_as_spec(iter_into_iter_spec)]
// pub assume_specification<I: Iterator>[ I::into_iter ](i: I) -> (r: I)
//     ensures
//         r == i,
// ;
}

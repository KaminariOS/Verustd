use vstd::prelude::*;
use std::mem::ManuallyDrop;
use std::ops::Deref;
verus!{

    pub trait ViewLocalCrate {
        type V;

        spec fn view(&self) -> Self::V;
    }
    impl<T: ?Sized> ViewLocalCrate for ManuallyDrop<T> {
        type V = Box<T>;
        spec fn view(&self) -> Self::V;
    }
    
    pub assume_specification<T>[ ManuallyDrop::<T>::new ](v: T) -> (a: ManuallyDrop<T>)
        ensures
            a.view() == Box::new(v)
    ;

    pub assume_specification<T: ?Sized>[ ManuallyDrop::<T>::deref ](s: &ManuallyDrop<T>) -> (a: &T)
        ensures
            s.view() == Box::new(a)
    ;
}

fn main() {
    // println!("{}", overflow());
}

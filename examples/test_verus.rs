use builtin_macros::*;
use vstd::prelude::*;
use vstd::simple_pptr::PPtr;

use std::collections::HashMap;
use vstd::relations::total_ordering;
use vstd::std_specs::hash::HashMapAdditionalSpecFns;
verus! {

    spec fn uninterp_fn(x: u64) -> bool;

    fn macro_test() {
         let mut index = 0;
         let len = 11;
         let v = vec![1];
         assert(v@[0] == 1);
    }

    fn hash_set_test() {
        let mut contacts = HashMap::new();

        broadcast use vstd::std_specs::hash::group_hash_axioms;
        contacts.insert(1, 5);
        assert(contacts[1] == 5);
        contacts.insert(1, 6);
        assert(contacts[1] == 6);
        // assert(contacts@["Daniel"] == "798-1364");  
        // println!("{contacts:?}");
        
    }
    pub struct BinaryHeap<
        T
    > {
        data: Vec<T>,
    }

    impl<T: Ord> Default for BinaryHeap<T> {
    /// Creates an empty `BinaryHeap<T>`.
    fn default() -> BinaryHeap<T> {
        BinaryHeap::new()
    }
}    
    impl<T: Ord> BinaryHeap<T> {
    pub const fn new() -> BinaryHeap<T> {
        BinaryHeap { data: vec![] }
    }
    }


    fn test_heap() {
        // fn compare(a: usize, b: usize) -> bool {
        //     a > b
        // }
        // assert(total_ordering(compare));
    }

    fn pop_test(t: Vec<u64>)
        requires
            t.len() > 0,
            forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]),
    {
        let mut t = t;
        let x = t.pop().unwrap();
        assert(uninterp_fn(x));
        assert(forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]));
    }

    // fn ng() -> (b: bool) 
    // ensures b
    //  {
    //     // Introduce unsoundness
    //     assume(false == true);
    //     // assume(true);
    //     // 1
    //     false
    // }
    fn main() {
        let (ptr, Tracked(perm)) = PPtr::new(
            5
                );
    }

}

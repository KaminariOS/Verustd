use vstd::prelude::*;
verus!{
    struct TestT(pub String);
    impl Clone for TestT {
    fn clone(&self) -> (out: Self) 
        ensures self.0 == out.0
        {
        Self(
               self.0.clone() 
        )
    }
}
impl PartialEq for TestT {
    fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
    }
}

impl Eq for TestT {}

// Then, manually mark Eq (only safe if equality is reflexive)
    fn test_clone(a: TestT) -> (b: TestT) 
        ensures a == b,
    {
        a.clone()
    }


    // This fails
    // fn test_clone1<T: Clone + Eq>(a: T) -> (b: T) 
    //     ensures a == b,
    // {
    //     a.clone()
    // }
}

fn main() {}

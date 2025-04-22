use vstd::prelude::*;
verus!{
    // Simple wrapper struct around a Rust `String`.
    struct TestT(pub String);

    // Implement `Clone` trait for `TestT`.
    // Allows creating a duplicate (`clone`) of an instance.
    impl Clone for TestT {

        // Clone method returns a new instance (`out`) identical to `self`.
        fn clone(&self) -> (out: Self)
            ensures self.0 == out.0 // Logical guarantee: cloned instance equals the original.
        {
            Self(self.0.clone()) // Cloning underlying `String`
        }
    }

    // Implement `PartialEq` for logical equality comparisons between `TestT` instances.
    impl PartialEq for TestT {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 // Compares the underlying strings for equality
        }
    }

    // Implement `Eq` marker trait, indicating `PartialEq` implementation is reflexive and total.
    impl Eq for TestT {}

// Then, manually mark Eq (only safe if equality is reflexive)
    // Function demonstrating verified clone operation.
    //
    // Returns a clone of input `a`, verifying through the postcondition (`ensures`)
    // that the returned object is equal to the original.
    fn test_clone(a: TestT) -> (b: TestT)
        ensures a == b,
    {
        a.clone() // Verifiably safe clone operation as ensured by specification
    }

    // This fails
    // fn test_clone1<T: Clone + Eq>(a: T) -> (b: T) 
    //     ensures a == b,
    // {
    //     a.clone()
    // }
}

fn main() {}

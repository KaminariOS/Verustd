use vstd::prelude::*;

verus!{

    // Demonstrates iterating over a range and maintaining an invariant
    fn vec(v: Vec<usize>, n: usize) {
        for i in 0..n
            invariant i <= n
        {
            require(i, n);
        }
    }

    // Helper function with a precondition requiring i < n
    fn require(i: usize, n: usize) 
        requires i < n
    {
        // Intentionally left empty for verification purposes
    }

    // Struct representing a wrapper around an index
    struct Hole(usize);

    impl Hole {
        // Constructor ensuring the index matches the provided parameter
        fn new(index: usize) -> (res: Self)
            ensures res.get_index_spec() == index
        {
            Hole(index)
        }

        // Getter ensuring returned value matches internal index
        fn get_index(&self) -> (res: usize)
            ensures res == self.get_index_spec() 
        {
            self.0
        }

        // Specification method for retrieving the index
        spec fn get_index_spec(&self) -> usize {
            self.0
        }
    }

    // Struct wrapping a vector of usize values
    struct Wrapper { data: Vec<usize> }

    impl Wrapper {
        // Closed specification method returning vector length
        pub closed spec fn spec_len(&self) -> usize {
            self.data.len()
        }

        // Getter method ensuring it matches the specified length
        fn len(&self) -> (res: usize) 
            ensures res == self.spec_len()
        {
            self.data.len()
        }

        // Method demonstrating a conditional index operation
        fn visit(&self) {
            if self.len() > 2 {
                let i = self.len() / 2;
                let h = Hole(i);
                assert(h.get_index_spec() < self.spec_len());
            }
        }

        // Method demonstrating a loop invariant over the vector length
        fn looping(&self) {
            for i in iter: 0..self.len()
                invariant iter.end == self.spec_len()
            {
                // Loop body intentionally empty
            }
        }

        // Method ensuring an index is within bounds after mutation
        fn end(&mut self, end: usize) 
            requires end <= old(self).spec_len()
        {
            assert(end <= self.spec_len());
        }

        // Method safely rebuilding the tail of the vector from a given start index
        fn rebuild_tail(&mut self, start: usize)
            requires start <= old(self).spec_len()
        {
            if start == self.len() {
                return;
            }

            let tail_len = self.len() - start;
            // Implementation details omitted
        }
    }

    // Function iterating safely over a slice, maintaining the length invariant
    fn array_loop(v: &[u8]) {
        let mut index = 0;
        let len = v.len();
        while index < len 
            invariant v.len() == len
        {
            let first = v[index];
            index += 1;
        }
    }
}

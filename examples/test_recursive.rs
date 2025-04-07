use vstd::prelude::*;
verus!{
spec fn reverse(xs: Seq<int>) -> Seq<int>
    decreases xs,
{
    if xs.len() == 0 {
        xs
    } else {
        reverse(xs.drop_first()).push(xs.first())
    }
}

proof fn reverse_id(x: int) ensures reverse(seq![x]) =~= seq![x] {

        reveal_with_fuel(reverse, 2);
}

// A richer function signature that includes stack depth(space complexity) and running time(time
        // complexity
// Problem: is it usable on general functions? This is purely static, unlike PointsTo 

// Similarly we can have linear ghost types for function calls and loop iteration: gas. We can
        // approximate prove the function terminates within a gas bound
tracked struct Gas(nat);
impl Gas {
    #[verifier::external_body]
    proof fn new(num: nat) -> (tracked res: Self) 
        ensures res.0 == num
    {
        unimplemented!()
    }

    #[verifier::external_body]
    proof fn consume_loop(tracked &mut self)
        requires old(self).0 > 0,
            ensures old(self).0 == self.0 + 1
    {
        // self.0 -= 1;
    }

    #[verifier::external_body]
    proof fn consume_func(tracked &mut self)
        requires old(self).0 > 0,
            ensures old(self).0 == self.0 + 1
    {
        // self.0 -= 1;
    }
}

fn sum(i: usize, Tracked(gas): Tracked<&mut Gas>) 
    requires 
        i < 1000,
        old(gas).0 > i + 1,
        ensures old(gas).0 == gas.0 + i + 1
        {
            proof {
                gas.consume_func();
            }
            let mut j = 0;
            assert(gas.0 + 1 + j == old(gas).0);
            while j < i 
            invariant gas.0 + 1 + j == old(gas).0,
            old(gas).0 > i + 1,
            j <= i
            decreases i - j
            {
                
                proof {
                    gas.consume_loop();
                }
                j += 1;
            }
            assert(gas.0 + 1 + j == old(gas).0);
            // assert( i <= j);
            assert(gas.0 + 1 + i == old(gas).0);
}

// This serves as a token for stack length; we may grow or shrink it by calling external functions.
tracked struct RecursiveCount(nat);
impl RecursiveCount {
    #[verifier::external_body]
    proof fn new(num: nat) -> (tracked res: Self) 
        ensures res.0 == num
    {
        unimplemented!()
    }
    #[verifier::external_body]
    proof fn push(tracked &mut self)
        requires old(self).0 > 0,
            ensures old(self).0 == self.0 + 1
    {
        // self.0 -= 1;
    }

    #[verifier::external_body]
    proof fn pop(tracked &mut self) 
            ensures old(self).0 + 1 == self.0
    {
        // self.0 += 1;
    }

    spec fn new_counter(s: Self) -> Self {
        s
    }
} 

    fn fib(n: u32, Tracked(counter): Tracked<&mut RecursiveCount>) -> (res: u32) 
        // Make the number of stack frames need explicit in the signature
        requires  old(counter).0 > n
        ensures old(counter).0 == counter.0
        // we need to force all functions have a counter. For recursive functions, the developer
// can finish the proof by connecting the recursive count with the rank function
        // decreases n 
    {
            // Ideally we have a const fn that return the size of the stack frame
                    proof {
                        counter.push();
                    }
        let res = match n {
            0 => 0,
            1 => 1,
            _ => {
                    // Or just use a new counter with a count - 1?
                    let res = fib(n - 1, Tracked(counter));
                    res
                },
        };
                    proof {
                        counter.pop();
                    }
        res
    }

    fn call_fib() {
        let tracked mut rc = RecursiveCount::new(12);
        proof {
            rc.push();
        }
        fib(10, Tracked(&mut rc));
        fib(10, Tracked(&mut rc));
        proof {
            rc.pop();
        }
    }

// fn recursive(x: usize)
//     decreases x {
//         recursive_1(x);
//     }
//
// fn recursive_1(x: usize)
//     decreases x {
//
//         recursive(x);
//     }
}

use builtin::*;
use builtin_macros::*;
use state_machines_macros::tokenized_state_machine;
use std::sync::Arc;
use vstd::atomic_ghost::*;
use vstd::modes::*;
use vstd::prelude::*;
use vstd::thread::*;
use vstd::{pervasive::*, *};

verus! {

// Define a tokenized state machine named X with specified fields and invariants
tokenized_state_machine!(
    X {
        fields {
            #[sharding(variable)]
            pub counter: int,  // Counter field representing the current count

            #[sharding(variable)]
            pub inc_a: bool,   // Flag indicating if thread A has incremented

            #[sharding(variable)]
            pub inc_b: bool,   // Flag indicating if thread B has incremented
        }

        // Main invariant ensuring the counter matches increments from threads
        #[invariant]
        pub fn main_inv(&self) -> bool {
            self.counter == (if self.inc_a { 1 as int } else { 0 }) + (if self.inc_b { 1 as int } else { 0 })
        }

        // Initialization transition setting default values
        init!{
            initialize() {
                init counter = 0;
                init inc_a = false;
                init inc_b = false;
            }
        }

        // Transition for thread A incrementing the counter
        transition!{
            tr_inc_a() {
                require(!pre.inc_a);
                update counter = pre.counter + 1;
                update inc_a = true;
            }
        }

        // Transition for thread B incrementing the counter
        transition!{
            tr_inc_b() {
                require(!pre.inc_b);
                update counter = pre.counter + 1;
                update inc_b = true;
            }
        }

        // Property ensuring increment does not overflow
        property!{
            increment_will_not_overflow_u32() {
                assert 0 <= pre.counter < 0xffff_ffff;
            }
        }

        // Property asserting the counter is 2 after both threads increment
        property!{
            finalize() {
                require(pre.inc_a);
                require(pre.inc_b);
                assert pre.counter == 2;
            }
        }

        // Proofs preserving invariants for transitions
        #[inductive(tr_inc_a)]
        fn tr_inc_a_preserves(pre: Self, post: Self) {}

        #[inductive(tr_inc_b)]
        fn tr_inc_b_preserves(pre: Self, post: Self) {}

        #[inductive(initialize)]
        fn initialize_inv(post: Self) {}
    }
);

// Structure with invariants wrapping an atomic counter tied to the state machine
struct_with_invariants!{
    pub struct Global {
        pub atomic: AtomicU32<_, X::counter, _>,
        pub instance: Tracked<X::Instance>,
    }

    spec fn wf(&self) -> bool {
        invariant on atomic with (instance) is (v: u32, g: X::counter) {
            g.instance_id() == instance@.id()
            && g.value() == v as int
        }
    }
}

fn main() {
    // Initialize protocol and atomic counter
    let tracked (
        Tracked(instance),
        Tracked(counter_token),
        Tracked(inc_a_token),
        Tracked(inc_b_token),
    ) = X::Instance::initialize();

    let tr_instance: Tracked<X::Instance> = Tracked(instance.clone());
    let atomic = AtomicU32::new(Ghost(tr_instance), 0, Tracked(counter_token));
    let global = Global { atomic, instance: Tracked(instance.clone()) };
    let global_arc = Arc::new(global);

    // Thread 1 increments counter
    let global_arc1 = global_arc.clone();
    let join_handle1 = spawn((move || -> (new_token: Tracked<X::inc_a>) ensures new_token@.instance_id() == instance.id() && new_token@.value() == true, {
        let tracked mut token = inc_a_token;
        let globals = &*global_arc1;
        let _ = atomic_with_ghost!(&globals.atomic => fetch_add(1);
            ghost c => {
                globals.instance.borrow().increment_will_not_overflow_u32(&c);
                globals.instance.borrow().tr_inc_a(&mut c, &mut token);
            }
        );
        Tracked(token)
    }));

    // Thread 2 increments counter
    let global_arc2 = global_arc.clone();
    let join_handle2 = spawn((move || -> (new_token: Tracked<X::inc_b>) ensures new_token@.instance_id() == instance.id() && new_token@.value() == true, {
        let tracked mut token = inc_b_token;
        let globals = &*global_arc2;
        let _ = atomic_with_ghost!(&globals.atomic => fetch_add(1);
            ghost c => {
                globals.instance.borrow().increment_will_not_overflow_u32(&mut c);
                globals.instance.borrow().tr_inc_b(&mut c, &mut token);
            }
        );
        Tracked(token)
    }));

    // Join threads and retrieve tokens
    let tracked inc_a_token;
    match join_handle1.join() {
        Result::Ok(token) => { proof { inc_a_token = token.get(); } },
        _ => { return; },
    };
    let tracked inc_b_token;
    match join_handle2.join() {
        Result::Ok(token) => { proof { inc_b_token = token.get(); } },
        _ => { return; },
    };

    // Final verification of the atomic counter's final state
    let global = &*global_arc;
    let x = atomic_with_ghost!(&global.atomic => load(); ghost c => {
        instance.finalize(&c, &inc_a_token, &inc_b_token);
    });

    assert(x == 2);
}

} // verus!

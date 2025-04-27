use std::marker::PhantomData;

use vstd::prelude::*;
use vstd::simple_pptr::*;

verus!{

// ─────────────────────────────────────────────────────────────
// 1. Basic mutation on a Vec with a safety assertion
// ─────────────────────────────────────────────────────────────

/// Sets v[0] = 1 when the vector is non‑empty and proves the write
fn set(v: &mut Vec<usize>) {
    if v.len() > 0 {
        v[0] = 1;
        assert(v[0] == 1); // post‑condition: first element updated
    }
}

// ─────────────────────────────────────────────────────────────
// 2. Verified swap for Vec<T>
//    We expose a spec for v.swap that rearranges two indices but preserves
//    length and all other elements.
// ─────────────────────────────────────────────────────────────

#[verifier::external_body]
fn swap<T>(v: &mut Vec<T>, i: usize, j: usize)
    requires i < old(v).len(), j < old(v).len()
    ensures v@ =~= v@.update(i as int, old(v)@[j as int])
                     .update(j as int, old(v)@[i as int]),
            v.len() == old(v).len()
{
    v.swap(i, j); // delegate to std::vec::Vec::swap
}

/// Small client of `swap` proving one index really changed as expected
fn swap_test<T>(v: &mut Vec<T>, i: usize, j: usize)
    requires i < old(v).len(), j < old(v).len()
{
    swap(v, i, j);
    assert(v@[i as int] == old(v)@[j as int]);
}

// ─────────────────────────────────────────────────────────────
// 3. Ghost example: passing a pure Seq view of Vec to another function
// ─────────────────────────────────────────────────────────────

#[verifier::external_body]
fn test_ghost(v: Ghost<Seq<usize>>)
    ensures v@.len() == 0
{ /* body irrelevant to proof */ }

fn pass_ghost(v: Vec<usize>) {
    test_ghost(Ghost(v@)); // supply a purely‑spec view
    assert(v.len() == 0);  // Vec moved; length is 0 after move
}

// ─────────────────────────────────────────────────────────────
// 4. Illustration of tracked PointsTo permissions with raw pointer ops
// ─────────────────────────────────────────────────────────────

fn test_tracked_points(Tracked(perm): Tracked<&mut PointsTo<u64>>)
    requires old(perm).is_init()
{ /* body omitted */ }

#[verifier::external_body]
fn increment(counter: PPtr<u64>, Tracked(perm): Tracked<&mut PointsTo<u64>>)
    requires counter == old(perm).pptr(),
             old(perm).is_init() && old(perm).value() < 100,
    ensures  perm.pptr() == old(perm).pptr(),
             perm.opt_value() == MemContents::Init((old(perm).value() + 1) as u64)
{ /* implemented in Rust, trusted by spec */ }

// ─────────────────────────────────────────────────────────────
// 5. Mutating through raw pointers inside a helper "Hole" struct
// ─────────────────────────────────────────────────────────────

struct Hole { ptr: *mut usize, pos: usize }
impl Hole {
    /// Create a Hole pointing to the underlying buffer of the Vec
    #[verifier::external_body]
    fn new(v: &mut Vec<usize>) -> (s: Self)
        ensures v@ =~= old(v)@,   // Vec content unchanged
                s.pos() == 0
    {
        Self { ptr: v.as_mut_ptr(), pos: 0 }
    }

    /// Safe wrapper for reading via raw pointer (spec trusted)
    #[verifier::external_body]
    fn get(&self, i: usize) -> &usize {
        unsafe { &*self.ptr.add(i) }
    }

    /// Safe wrapper for writing via raw pointer (spec trusted)
    #[verifier::external_body]
    fn set(&mut self, i: usize, v: usize) {
        unsafe { *self.ptr.add(i) = v; }
    }

    // Pure spec for exposing pos with an offset (example of when_used_as_spec)
    spec fn spec_pos(&self) -> usize { (self.pos + 1) as usize }

    #[verifier::when_used_as_spec(spec_pos)]
    fn pos(&self) -> (res: usize) { self.pos }
}

/// Example using Hole to read & write without invalidating the Vec spec view
fn vec_ptr(v: &mut Vec<usize>) {
    let mut h = Hole::new(v);
    assert(h.pos() == 0);
    let _old = h.get(0);
    h.set(0, 1);
    assert(old(v)@ =~= v@); // content equality cannot be proven here; left as demo
}

// ─────────────────────────────────────────────────────────────
// 6. PhantomData‑based lifetime tracking toy example
// ─────────────────────────────────────────────────────────────

struct Hole1<'a, T: 'a> { mark: PhantomData<&'a T> }
impl<'a, T> Hole1<'a, T> {
    fn new(_v: &mut Vec<T>) -> Self { Self { mark: PhantomData } }
    fn set(&mut self, _v: &mut Vec<T>) { /* no‑op */ }
}

fn test(v: &mut Vec<usize>) {
    let mut h = Hole1::new(v);
    h.set(v);
}

} // verus! block

fn main(){}

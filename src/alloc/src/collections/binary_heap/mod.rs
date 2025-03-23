#![allow(missing_docs)]
#![allow(unused_imports)]
// #![stable(feature = "rust1", since = "1.0.0")]

// use core::alloc::Allocator;
use core::iter::{FusedIterator};
use core::mem::{self, ManuallyDrop, swap};
use core::num::NonZero;
use core::ops::{Deref, DerefMut};
use core::{fmt, ptr};
use core::cmp::Ordering;
use std::marker::PhantomData;

// use crate::alloc::Global;
// use crate::collections::TryReserveError;
// use crate::slice;
// use crate::vec::{self, AsVecIntoIter, Vec};
// use alloc::alloc::Global;
// use alloc::vec::{self, Vec};

use vstd::prelude::*;

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

pub open spec fn le<T: ?Sized>(a: &T, b: &T) -> bool;
proof fn reflexive<T: Ord + ?Sized>(x: &T)
ensures le(x, x) 
{
    admit()
}

proof fn total<T: Ord + ?Sized>(x: &T, y: &T)
ensures le(x, y) || le(y, x) {
    admit()
}

proof fn transitive<T: Ord + ?Sized>(x: &T, y: &T, z: &T)
requires le(x, y), le(y, z),
ensures le(x, z) {
    admit()
}

proof fn antisymmetric<T: Ord + ?Sized>(x: &T, y: &T)
requires le(x, y), le(y, x),
ensures x == y {
    admit()
}

#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd  {
    type ExternalTraitSpecificationFor: core::cmp::Ord;
    fn cmp(&self, other: &Self) -> Ordering;
}



pub assume_specification[ Ordering::is_gt ](
    v: Ordering,
) -> (result: bool)
    ensures
        result == (v == Ordering::Greater) ,
;

pub assume_specification[ Ordering::is_lt ](
    v: Ordering,
) -> (result: bool)
    ensures
        result == (v == Ordering::Less) ,
;

pub assume_specification[ usize::saturating_sub](
    v: usize,
    rhs: usize,
) -> (result: usize)
    ensures
        // Case 1: Underflow (result would be < i8::MIN)
        (v - rhs < usize::MIN) ==> (result == usize::MIN),
        // Case 2: Overflow (result would be > i8::MAX)
        (v - rhs > usize::MAX) ==> (result == usize::MAX),
        // Case 3: No overflow/underflow
        (usize::MIN <= v - rhs <= usize::MAX) ==> (result == v - rhs),
;

// pub assume_specification<T, A: Allocator>[ Vec::<T, A>::len ](
//     v: &Vec<T, A>,
// ) -> (result: usize)
//     ensures
//         (size_of::<T>() == 0 ==> result == usize::MAX) && (size_of::<T>() != 0 ==> result * size_of::<T>() <= isize::MAX) 
// ;
// #[verifier::external_body]
// fn vec_len<T>(v: &Vec<T>) -> (len: usize) 
//     ensures
//         (size_of::<T>() == 0 ==> len == usize::MAX) && (size_of::<T>() != 0 ==> len * size_of::<T>() <= isize::MAX) 
// {
//     v.len()
// }
//
//     spec fn floor_log2(n: nat) -> nat 
//     decreases n
//     {
//         if n < 2 {
//             0
//         } else {
//             1 + floor_log2(n / 2)
//         }
//     }

// TODO: hava a formal spec for leading_zeros() and prove the correctness of this function 
// #[inline(always)]
// #[verifier::external_body]
// fn log2_fast(x: usize) -> (res: usize) 
//     ensures res == floor_log2(x as nat) && res <= usize::BITS
//     {
// (usize::BITS - x.leading_zeros() - 1) as usize
// }

/// A priority queue implemented with a binary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the [`Ord`]
/// trait, changes while it is in the heap. This is normally only possible
/// through interior mutability, global state, I/O, or unsafe code. The
/// behavior resulting from such a logic error is not specified, but will
/// be encapsulated to the `BinaryHeap` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// As long as no elements change their relative order while being in the heap
/// as described above, the API of `BinaryHeap` guarantees that the heap
/// invariant remains intact i.e. its methods all behave as documented. For
/// example if a method is documented as iterating in sorted order, that's
/// guaranteed to work as long as elements in the heap have not changed order,
/// even in the presence of closures getting unwinded out of, iterators getting
/// leaked, and similar foolishness.
///
/// # Examples
///
/// ```
/// use std::collections::BinaryHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{x}");
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// A `BinaryHeap` with a known list of items can be initialized from an array:
///
/// ```
/// use std::collections::BinaryHeap;
///
/// let heap = BinaryHeap::from([1, 5, 2]);
/// ```
///
/// ## Min-heap
///
/// Either [`core::cmp::Reverse`] or a custom [`Ord`] implementation can be used to
/// make `BinaryHeap` a min-heap. This makes `heap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use std::collections::BinaryHeap;
/// use std::cmp::Reverse;
///
/// let mut heap = BinaryHeap::new();
///
/// // Wrap values in `Reverse`
/// heap.push(Reverse(1));
/// heap.push(Reverse(5));
/// heap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(Reverse(1)));
/// assert_eq!(heap.pop(), Some(Reverse(2)));
/// assert_eq!(heap.pop(), Some(Reverse(5)));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | [push]  | [pop]         | [peek]/[peek\_mut] |
/// |---------|---------------|--------------------|
/// | *O*(1)~ | *O*(log(*n*)) | *O*(1)             |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [`core::cmp::Reverse`]: core::cmp::Reverse
/// [`Cell`]: core::cell::Cell
/// [`RefCell`]: core::cell::RefCell
/// [push]: BinaryHeap::push
/// [pop]: BinaryHeap::pop
/// [peek]: BinaryHeap::peek
/// [peek\_mut]: BinaryHeap::peek_mut
// #[stable(feature = "rust1", since = "1.0.0")]
// #[cfg_attr(not(test), rustc_diagnostic_item = "BinaryHeap")]
pub struct BinaryHeap<
    T,
    // #[unstable(feature = "allocator_api", issue = "32838")] 
> {
    data: Vec<T>,
    pub elems: Tracked<Map<nat, T>>,
}

impl<T: Ord> View for BinaryHeap<T> {
    type V = Seq<T>;
    closed spec fn view(&self) -> Self::V {
        self.data@
    }
}

impl<T: Ord> BinaryHeap<T> {
    pub closed spec fn spec_len(&self) -> usize {
            self.data.len()
    }

    pub proof fn spec_len_limit(&self) 
        ensures self.spec_len() < isize::MAX
    {
        // An implicit invariant from the implementation of raw_vec: for non ZST, the capacity(in bytes) of the buffer is no greater than isize::MAX; for ZST, it is always usize::MAX 
        // (size_of::<T>() == 0 ==> len == usize::MAX) && (size_of::<T>() != 0 ==> len * size_of::<T>() <= isize::MAX) 
        // It is possible that T is ZST, for now let's ignore it
        admit()
    }
    spec fn parent(&self, child_idx: nat) -> T {
        self@[Self::parent_index(child_idx)]
    }
    
    spec fn left_child_index(child_idx: nat) -> int {
        (child_idx * 2 + 1) as int
    }

    spec fn right_child_index(child_idx: nat) -> int {
        Self::left_child_index(child_idx)
    }

    spec fn parent_index(child_idx: nat) -> int {
        if child_idx == 0 {
            0
        } else {
            (child_idx - 1) / 2
        }
    }

    pub closed spec fn well_formed(&self) -> bool {
            // true
        // &&& (forall|i: nat| 0 <= i < self.spec_len() ==> #[trigger] self.elems@.dom().contains(i) && self@[i as int] == self.elems@.index(i))
        // Every child is not greater than its parent
        self.well_formed_to(self.spec_len())
    }

    pub closed spec fn well_formed_to(&self, end: usize) -> bool {
            // true
        // &&& (forall|i: nat| 0 <= i < self.spec_len() ==> #[trigger] self.elems@.dom().contains(i) && self@[i as int] == self.elems@.index(i))
        // Every child is not greater than its parent
        &&& (forall|i: nat| 0 <= i < end ==> #[trigger] le(&self@[i as int], &self.parent(i)))
    }

    pub const fn new() -> (res: BinaryHeap<T>) 
    ensures res.well_formed()
    {
        BinaryHeap { data: vec![], elems: Tracked(Map::tracked_empty())}
    }

    pub fn with_capacity(capacity: usize) -> (s: BinaryHeap<T>) 
    ensures s.well_formed()
    {
        BinaryHeap { data: Vec::with_capacity(capacity), elems: Tracked(Map::tracked_empty()) }
    }

    pub fn len(&self) -> (len: usize) 

    ensures
        // An implicit invariant from the implementation of raw_vec: for non ZST, the capacity(in bytes) of the buffer is no greater than isize::MAX; for ZST, it is always usize::MAX 
        // (size_of::<T>() == 0 ==> len == usize::MAX) && (size_of::<T>() != 0 ==> len * size_of::<T>() <= isize::MAX) 
        len <= isize::MAX,
        len == self.spec_len(),
        // len == old(self).spec_len()
        {
        proof {
            self.spec_len_limit();
        }
        self.data.len()
    }

    pub fn is_empty(&self) -> (res: bool) 
    ensures res == (self.spec_len() == 0)
    {
        self.len() == 0
    }

    pub fn push(&mut self, item: T)
    requires old(self).well_formed()
    ensures self.well_formed()
    {
        let old_len = self.len();
        self.data.push(item);
        // SAFETY: Since we pushed a new item it means that
        //  old_len = self.len() - 1 < self.len()
        unsafe { self.sift_up(0, old_len) };
    }

    pub fn pop(&mut self) -> (res: Option<T>)
    requires old(self).well_formed()
    ensures old(self).spec_len() == 0 ==> res.is_none(),
        // old(self).spec_len() != 0 ==> 
    {
        if let Some(mut item) = self.data.pop() {
                if !self.is_empty() {
                    self.swap_with_i(&mut item, 0);
                    // SAFETY: !self.is_empty() means that self.len() > 0
                    unsafe { self.sift_down_to_bottom(0) };
                }
                Some(item)
            } else {
                None
            }
    }

    #[verifier::external_body]
    fn swap_with_i(&mut self, item: &mut T, i: usize) 
    requires i < old(self).spec_len()
    ensures item == self@[0 as int], self@ == old(self)@.update(0, *old(item))
    {
         swap(&mut self.data[i], item);
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> (res: usize) 
        requires pos < old(self).spec_len(), 
        start == 0 // all calls to this function have start == 0
        ensures self.spec_len() == old(self).spec_len()
        {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.len()
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };

        while hole.pos() > start
        invariant self.spec_len() == old(self).spec_len(),
        hole.pos < self.spec_len()
        {
            let parent = (hole.pos() - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            let order = hole.element().cmp(unsafe { hole.get(parent, &self.data) }); 
            if !order.is_gt() {
                break;
            }

            // SAFETY: Same as above
            unsafe { hole.move_to(parent, &mut self.data) };
        }

        hole.pre_drop(&mut self.data);
        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) 
        requires pos < end, end <= old(self).spec_len()
        ensures old(self).spec_len() == self.spec_len() 
{
        proof {
            self.spec_len_limit();
        }
        // SAFETY: The caller guarantees that pos < end <= self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };

        assert(old(self).spec_len() == self.spec_len());
        // assert(hole.spec_pos() <= end);
        // assert(hole.spec_pos() <= self.spec_len());
        let mut child = 2 * hole.pos() + 1;

            assert(end <= old(self).spec_len());
        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) 
        invariant 
        child == 2 * hole.spec_pos() + 1,
        // child + 1 < end <= self.spec_len(),
        old(self).spec_len() == self.spec_len(),
        end <= old(self).spec_len()
        {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { !hole.get(child, &self.data).cmp(hole.get(child + 1, &self.data)).is_gt() } as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.len() and != hole.pos()
            if !hole.element().cmp(unsafe { hole.get(child, &self.data) }).is_lt() {
                // assert(old(self).spec_len() == self.spec_len());
                return;
            }

            // SAFETY: same as above, for now let's ignore it
            unsafe { hole.move_to(child, &mut self.data) };
            proof {
                self.spec_len_limit();
            }
            child = 2 * hole.pos() + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.len().
        if child == end - 1 && hole.element().cmp(unsafe { hole.get(child, &self.data) }).is_lt() {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child, &mut self.data) };
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down(&mut self, pos: usize) 
    requires pos < old(self).spec_len()
    ensures old(self).spec_len() == self.spec_len()
    {
        let len = self.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.len() <= self.len().
        unsafe { self.sift_down_range(pos, len) };
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) 
    requires pos < old(self).spec_len(), pos == 0,
    old(self).well_formed_to(pos)
    {
        let end = self.len();
        let start = pos;

        proof {
            self.spec_len_limit();
        }
        // SAFETY: The caller guarantees that pos < self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) 
        invariant child == 2 * hole.spec_pos() + 1,
        // child + 1 < end <= self.spec_len(),
        old(self).spec_len() == self.spec_len(),
        end <= old(self).spec_len(),
        hole.pos() < self.spec_len()
{
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { if !hole.get(child, &self.data).cmp(hole.get(child + 1, &self.data)).is_gt() {1} else {0} } as usize;

            // SAFETY: Same as above
            unsafe { hole.move_to(child, &mut self.data) };
            proof {
                self.spec_len_limit();
            }
            child = 2 * hole.pos() + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child, &mut self.data) };
        }
        pos = hole.pos();
        hole.pre_drop(&mut self.data);
        // drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    fn rebuild_tail(&mut self, start: usize)
        requires start <= old(self).spec_len(), old(self).well_formed_to(start)
        ensures self.well_formed()
        {
        if start == self.len() {
            return;
        }
        assert(start <= self.spec_len());
        let tail_len = self.len() - start;


        // `rebuild` takes O(self.len()) operations
        // and about 2 * self.len() comparisons in the worst case
        // while repeating `sift_up` takes O(tail_len * log(start)) operations
        // and about 1 * tail_len * log_2(start) comparisons in the worst case,
        // assuming start >= tail_len. For larger heaps, the crossover point
        // no longer follows this reasoning and was determined empirically.
        let better_to_rebuild = if start < tail_len {
            true
        } else if self.len() <= 2048 {
            // NOTE: Right side may overflow but it has no impact on correctness 
            // 2 * self.len() < tail_len * log2_fast(start)
            true
        } else {
            // 2 * self.len() < tail_len * 11
            true
        };

        if better_to_rebuild {
            self.rebuild();
        } else {
            for i in iter: start..self.len() 
                invariant iter.end == self.spec_len()
            {
                // SAFETY: The index `i` is always less than self.len().
                unsafe { self.sift_up(0, i) };

            }
        }
    }

    fn rebuild(&mut self) 
    requires old(self).spec_len() > 0
    {
        let mut n = self.len() / 2;
        while n > 0 
        invariant n < self.spec_len()
        {
            n -= 1;
            // SAFETY: n starts from self.len() / 2 and goes down to 0.
            //  The only case when !(n < self.len()) is if
            //  self.len() == 0, but it's ruled out by the loop condition.
            unsafe { self.sift_down(n) };
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::BinaryHeap;
    ///
    /// let mut a = BinaryHeap::from([-10, 1, 2, 3, 3]);
    /// let mut b = BinaryHeap::from([-20, 5, 43]);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self)
    requires old(self).well_formed(), old(other).well_formed()
    ensures self.well_formed() // && new self is the combination and new other is empty
    {
        if self.len() < other.len() {
            swap(self, other);
        } else {
            // assert(!old(self).spec_len() < old(other).spec_len());
            // assert(old(self).spec_len() >= old(other).spec_len());
        }

        let start = self.data.len();

        self.data.append(&mut other.data); 
        proof {
            if old(self).spec_len() < old(other).spec_len() {
                assert(self@.subrange(0int, start as int) =~= old(other)@);
                assert((forall|i: nat| 0 <= i < start &&  
                (
                    #[trigger] 
                    old(other)@[i as int] == self@[i as int] && 
                    // old(other).parent(i) == self.parent(i) && 
                    le(&old(other)@[i as int], &old(other).parent(i))
                ) 
                ==> 
                le(&(self)@[i as int], &(self).parent(i))
                ));
                // assert((forall|i: nat| 0 <= i < start ==> #[trigger]  le(&(self)@[i as int], &(self).parent(i)) ));
            } else {
                assert(self@.subrange(0int, start as int) =~= old(self)@);
                assert((forall|i: nat| 0 <= i < start &&  
                (
                    #[trigger] 
                    old(self)@[i as int] == self@[i as int] && 
                    // old(self).parent(i) == self.parent(i) && 
                    le(&old(self)@[i as int], &old(self).parent(i))
                ) 
                ==> 
                le(&(self)@[i as int], &(self).parent(i))
                ));
                // assert((forall|i: nat| 0 <= i < start ==> #[trigger]  le(&(self)@[i as int], &(self).parent(i)) ));
            }
            
            // assert((forall|i: nat| 0 <= i < start ==> #[trigger]  le(&(self)@[i as int], &(self).parent(i)) ));
        }
        // assert(start == old(self).spec_len());
        // assert((!old(self).spec_len() < old(other).spec_len()) == (old(self).spec_len() >= old(other).spec_len()));
    //     assert(( (!old(self).spec_len() < old(other).spec_len()) ==> self@.subrange(0int, start as int) =~= old(self)@) 
    //     && 
    //         ( old(self).spec_len() < old(other).spec_len() ==> self@.subrange(0int, start as int) =~= old(other)@ )
    // );
        // assert((forall|i: nat| 0 <= i < start &&  
        // (
        //     (
        //     (!old(self).spec_len() < old(other).spec_len()) ==> 
        //     #[trigger] 
        //     old(self)@[i as int] == self@[i as int] && 
        //     old(self).parent(i) == self.parent(i) && 
        //     le(&old(self)@[i as int], &old(self).parent(i))
        //     ) 
        // && 
        //     (
        //     old(self).spec_len() < old(other).spec_len() ==> 
        //     #[trigger] 
        //     old(other)@[i as int] == self@[i as int] && 
        //     old(other).parent(i) == self.parent(i) && 
        //     le(&old(other)@[i as int], &old(other).parent(i))
        //     )
        // ) 
        // ==> 
        // le(&(self)@[i as int], &(self).parent(i))
        // ));
        // assert((forall|i: nat| 0 <= i < start ==> #[trigger]  le(&(self)@[i as int], &(self).parent(i)) ));
        // assert(self.well_formed_to(start));

        self.rebuild_tail(start);
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    // data: *mut T,
    len: usize,
    elt: ManuallyDrop<T>,
    pos: usize,
    marker: PhantomData<&'a T>
}

impl<'a, T: 'a> Hole<'a, T> {
    spec fn spec_pos(&self) -> usize {
        self.pos
    } 
    /// Creates a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    #[verifier::external_body]
    unsafe fn new(data: &mut Vec<T>, pos: usize) -> (res: Self) 
        requires pos < old(data).len()
        ensures pos == res.spec_pos(), data == old(data)
        {
        // debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = ManuallyDrop::new(unsafe { ptr::read(data.get_unchecked(pos)) });
        let len = data.len();
        Hole { elt, pos, len, marker: PhantomData}
    }

    #[inline]
    // #[verifier::
    #[verifier::when_used_as_spec(spec_pos)]
    fn pos(&self) -> (res: usize) 
    ensures res == self.spec_pos()
    {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        self.elt.deref()
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    #[verifier::external_body]
    unsafe fn get<'b>(&self, index: usize, v: &'b Vec<T>) -> (res: &'b T) 
    requires index != self.pos,
    index < v.len()
    ensures *res == v@[index as int]
    {
        // debug_assert!(index != self.pos);
        // debug_assert!(index < self.len);
        unsafe { v.get_unchecked(index) }
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    #[verifier::external_body]
    unsafe fn move_to(&mut self, index: usize, v: &mut Vec<T>)
        requires index != old(self).pos, index < old(v).len()
        ensures self.pos == index, 
        v@ =~= old(v)@.update(index as int, old(v)@[old(self).pos as int]).update(old(self).pos as int, old(v)@[index as int]),
        {
        // debug_assert!(index != self.pos);
        // debug_assert!(index < self.len);
        unsafe {
            let ptr = v.as_mut_ptr();
            let index_ptr: *const _ = ptr.add(index);
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        }
        self.pos = index;
    }
    
    #[verifier::external_body]
    fn pre_drop(&mut self, v: &mut Vec<T>) 
    ensures v == old(v)
    {
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, v.get_unchecked_mut(pos), 1);
        }
    }
}


// impl<'a, T: 'a> Drop for Hole<'a, T> {
//     #[inline]
//     #[verifier::external_body]
//     fn drop(&mut self) 
//     opens_invariants none 
//         no_unwind
//         {
//         // fill the hole again
//         unsafe {
//             let pos = self.pos;
//             ptr::copy_nonoverlapping(&self.elt, self.data.add(pos), 1);
//         }
//     }
// }

}


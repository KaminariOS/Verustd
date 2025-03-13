//! A priority queue implemented with a binary heap.
//!
//! Insertion and popping the largest element have *O*(log(*n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a binary heap
//! can be done in-place, and has *O*(*n*) complexity. A binary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an *O*(*n* * log(*n*))
//! in-place heapsort.
//!
//! # Examples
//!
//! This is a larger example that implements [Dijkstra's algorithm][dijkstra]
//! to solve the [shortest path problem][sssp] on a [directed graph][dir_graph].
//! It shows how to use [`BinaryHeap`] with custom types.
//!
//! [dijkstra]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [sssp]: https://en.wikipedia.org/wiki/Shortest_path_problem
//! [dir_graph]: https://en.wikipedia.org/wiki/Directed_graph
//!
//! ```
//! use std::cmp::Ordering;
//! use std::collections::BinaryHeap;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! struct State {
//!     cost: usize,
//!     position: usize,
//! }
//!
//! // The priority queue depends on `Ord`.
//! // Explicitly implement the trait so the queue becomes a min-heap
//! // instead of a max-heap.
//! impl Ord for State {
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         // Notice that we flip the ordering on costs.
//!         // In case of a tie we compare positions - this step is necessary
//!         // to make implementations of `PartialEq` and `Ord` consistent.
//!         other.cost.cmp(&self.cost)
//!             .then_with(|| self.position.cmp(&other.position))
//!     }
//! }
//!
//! // `PartialOrd` needs to be implemented as well.
//! impl PartialOrd for State {
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! // Each node is represented as a `usize`, for a shorter implementation.
//! struct Edge {
//!     node: usize,
//!     cost: usize,
//! }
//!
//! // Dijkstra's shortest path algorithm.
//!
//! // Start at `start` and use `dist` to track the current shortest distance
//! // to each node. This implementation isn't memory-efficient as it may leave duplicate
//! // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
//! // for a simpler implementation.
//! fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//!     // dist[node] = current shortest distance from `start` to `node`
//!     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
//!
//!     let mut heap = BinaryHeap::new();
//!
//!     // We're at `start`, with a zero cost
//!     dist[start] = 0;
//!     heap.push(State { cost: 0, position: start });
//!
//!     // Examine the frontier with lower cost nodes first (min-heap)
//!     while let Some(State { cost, position }) = heap.pop() {
//!         // Alternatively we could have continued to find all shortest paths
//!         if position == goal { return Some(cost); }
//!
//!         // Important as we may have already found a better way
//!         if cost > dist[position] { continue; }
//!
//!         // For each node we can reach, see if we can find a way with
//!         // a lower cost going through this node
//!         for edge in &adj_list[position] {
//!             let next = State { cost: cost + edge.cost, position: edge.node };
//!
//!             // If so, add it to the frontier and continue
//!             if next.cost < dist[next.position] {
//!                 heap.push(next);
//!                 // Relaxation, we have now found a better way
//!                 dist[next.position] = next.cost;
//!             }
//!         }
//!     }
//!
//!     // Goal not reachable
//!     None
//! }
//!
//! fn main() {
//!     // This is the directed graph we're going to use.
//!     // The node numbers correspond to the different states,
//!     // and the edge weights symbolize the cost of moving
//!     // from one node to another.
//!     // Note that the edges are one-way.
//!     //
//!     //                  7
//!     //          +-----------------+
//!     //          |                 |
//!     //          v   1        2    |  2
//!     //          0 -----> 1 -----> 3 ---> 4
//!     //          |        ^        ^      ^
//!     //          |        | 1      |      |
//!     //          |        |        | 3    | 1
//!     //          +------> 2 -------+      |
//!     //           10      |               |
//!     //                   +---------------+
//!     //
//!     // The graph is represented as an adjacency list where each index,
//!     // corresponding to a node value, has a list of outgoing edges.
//!     // Chosen for its efficiency.
//!     let graph = vec![
//!         // Node 0
//!         vec![Edge { node: 2, cost: 10 },
//!              Edge { node: 1, cost: 1 }],
//!         // Node 1
//!         vec![Edge { node: 3, cost: 2 }],
//!         // Node 2
//!         vec![Edge { node: 1, cost: 1 },
//!              Edge { node: 3, cost: 3 },
//!              Edge { node: 4, cost: 1 }],
//!         // Node 3
//!         vec![Edge { node: 0, cost: 7 },
//!              Edge { node: 4, cost: 2 }],
//!         // Node 4
//!         vec![]];
//!
//!     assert_eq!(shortest_path(&graph, 0, 1), Some(1));
//!     assert_eq!(shortest_path(&graph, 0, 3), Some(3));
//!     assert_eq!(shortest_path(&graph, 3, 0), Some(7));
//!     assert_eq!(shortest_path(&graph, 0, 4), Some(5));
//!     assert_eq!(shortest_path(&graph, 4, 0), None);
//! }
//! ```

#![allow(missing_docs)]
// #![stable(feature = "rust1", since = "1.0.0")]

// use core::alloc::Allocator;
use core::iter::{FusedIterator};
use core::mem::{self, ManuallyDrop, swap};
use core::num::NonZero;
use core::ops::{Deref, DerefMut};
use core::{fmt, ptr};
use core::cmp::Ordering;

// use crate::alloc::Global;
// use crate::collections::TryReserveError;
// use crate::slice;
// use crate::vec::{self, AsVecIntoIter, Vec};
// use alloc::alloc::Global;
use alloc::vec::{self, Vec};


use vstd::prelude::*;

verus!{

#[verifier::external_type_specification]
pub struct ExOrdering(Ordering);

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd  {
    type ExternalTraitSpecificationFor: core::cmp::Ord;
    fn cmp(&self, other: &Self) -> Ordering;
}

pub assume_specification[ Ordering::is_gt](
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

pub assume_specification<T>[ core::mem::drop ](
    v: T,
);


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
#[verifier::external_body]
fn vec_len<T>(v: &Vec<T>) -> (len: usize) 
    ensures
        (size_of::<T>() == 0 ==> len == usize::MAX) && (size_of::<T>() != 0 ==> len * size_of::<T>() <= isize::MAX) 
{
    v.len()
}

fn count_leading_zeros(n: usize) -> u32 {
    if n == 0 {
        return usize::BITS; // All bits are zero
    }

    let total_bits = usize::BITS;
    // Iterate from the highest bit down to the lowest (0)
    for bit in (0..total_bits) {
        let bit = total_bits - bit - 1;
        // Check if the current bit is set (shift `n` right by `bit` positions)
        if (n >> bit) & 1 != 0 {
            // Leading zeros = total_bits - position_of_highest_set_bit - 1
            return total_bits - bit - 1;
        }
    }
    // Unreachable for `n != 0` (since at least one bit is set)
    total_bits
}

#[inline(always)]
fn log2_fast(x: usize) -> usize {
(usize::BITS - count_leading_zeros(x) - 1) as usize
}

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

}

impl<T: Ord> BinaryHeap<T> {
    pub const fn new() -> BinaryHeap<T> {
        BinaryHeap { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> BinaryHeap<T> {
        BinaryHeap { data: Vec::with_capacity(capacity) }
    }

    #[verifier::external_body]
    pub fn len(&self) -> (len: usize) 

    ensures
        // (size_of::<T>() == 0 ==> len == usize::MAX) && (size_of::<T>() != 0 ==> len * size_of::<T>() <= isize::MAX) 
        len <= isize::MAX
        {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        // SAFETY: Since we pushed a new item it means that
        //  old_len = self.len() - 1 < self.len()
        unsafe { self.sift_up(0, old_len) };
    }

    #[verifier::external_body]
    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut item) = self.data.pop() {
                if !self.is_empty() {
                    swap(&mut item, &mut self.data[0]);
                    // SAFETY: !self.is_empty() means that self.len() > 0
                    unsafe { self.sift_down_to_bottom(0) };
                }
                Some(item)
            } else {
                None
            }
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
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.len()
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };

        while hole.pos() > start {
            let parent = (hole.pos() - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            let order = hole.element().cmp(unsafe { hole.get(parent) }); 
            if !order.is_gt() {
                break;
            }

            // SAFETY: Same as above
            unsafe { hole.move_to(parent) };
        }

        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        // SAFETY: The caller guarantees that pos < end <= self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { !hole.get(child).cmp(hole.get(child + 1)).is_gt() } as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.len() and != hole.pos()
            if !hole.element().cmp(unsafe { hole.get(child) }).is_lt() {
                return;
            }

            // SAFETY: same as above.
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.len().
        if child == end - 1 && hole.element().cmp(unsafe { hole.get(child) }).is_lt() {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
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
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.len();
        let start = pos;

        // SAFETY: The caller guarantees that pos < self.len().
        let mut hole = unsafe { Hole::new(&mut self.data, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // SAFETY: child < end - 1 < self.len() and
            //  child + 1 < end <= self.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { if !hole.get(child).cmp(hole.get(child + 1)).is_gt() {1} else {0} } as usize;

            // SAFETY: Same as above
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
        pos = hole.pos();
        drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    fn rebuild_tail(&mut self, start: usize) {
        if start == self.len() {
            return;
        }

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
            2 * self.len() < tail_len * log2_fast(start)
        } else {
            2 * self.len() < tail_len * 11
        };

        if better_to_rebuild {
            self.rebuild();
        } else {
            for i in start..self.len() {
                // SAFETY: The index `i` is always less than self.len().
                unsafe { self.sift_up(0, i) };
            }
        }
    }

    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
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
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        let start = self.data.len();

        self.data.append(&mut other.data);

        self.rebuild_tail(start);
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<T> {
    data: *mut T,
    len: usize,
    elt: T,
    pos: usize
}

impl<T> Hole<T> {
    /// Creates a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    #[verifier::external_body]
    unsafe fn new(data: &mut Vec<T>, pos: usize) -> Self {
        // debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = unsafe { ptr::read(data.get_unchecked(pos)) };
        let len = data.len();
        Hole { data: data.as_mut_ptr(), elt: elt, pos, len }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    #[verifier::external_body]
    unsafe fn get(&self, index: usize) -> &T {
        // debug_assert!(index != self.pos);
        // debug_assert!(index < self.len);
        unsafe { &*self.data.add(index) }
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    #[verifier::external_body]
    unsafe fn move_to(&mut self, index: usize) {
        // debug_assert!(index != self.pos);
        // debug_assert!(index < self.len);
        unsafe {
            let ptr = self.data;
            let index_ptr: *const _ = ptr.add(index);
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        }
        self.pos = index;
    }
}

impl<T> Drop for Hole<T> {
    #[inline]
    #[verifier::external_body]
    fn drop(&mut self) 
    opens_invariants none 
        no_unwind
        {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&self.elt, self.data.add(pos), 1);
        }
    }
}
}


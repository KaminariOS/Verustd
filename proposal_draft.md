# Verify Lock-free Data Structures With Verus

## Introduction

Concurrent data structures in practice are typically classified into two broad categories: those that employ blocking (lock-based) synchronization mechanisms and those that utilize non-blocking, usually lock-free, techniques. Blocking data structures are generally easier to implement and reason about but can lead to issues like deadlocks, priority inversion, and poor scalability under high contention. On the contrary, non-blocking data structures ensures system-wide progress and responsiveness by using atomic operations for synchronization, at the cost of increased complexity and subtle correctness challenges. 

The most common type of non-blocking data structures are lock-free: always at least one thread makes progress. 


well-formedness

atomic invariant 

## State of the Art
model checking

heroic effort

## Background
// Need an Introduction of Concurrent separation logic here.
It is worth mentioning that in the special case where only 2 threads needs to synchronize, there exists mechanisms that does not use locksor atomic operations like Peterson's algorithm and kfifo in Linux. 


Rust affine type system ownership

### Verus 

permission-based reasoning

efficient SMT encoding

linear ghost state

### Limitation 

Currently Verus only supports sequential consistent memory model. In practice, most implementations of lock-free data structures strive to maximize performance by using the weakest memory orderings that still guarantee correctness. Reasoning about weak memory ordering is intricate and beyond the scope of this paper.    

### VerusSync


state machine 


## Approaches

### Specification

### 
ABA problem 

Michael and Scott queue

# Verify Lock-free Data Structures With Verus

## Introduction

Concurrent data structures in practice are typically classified into two broad categories: those that employ blocking (lock-based) synchronization mechanisms and those that utilize non-blocking, usually lock-free, techniques. 

high contention

low latency

avoid deadlocks or priority inversion

atomic primitives 

correctness implementation

well-formedness

atomic invariant 

## State of the Art
model checking

heroic effort

## Background

Rust affine type system ownership

Verus 
efficient SMT encoding

Limitation: currently Verus only supports sequential consistent momory model.

VerusSync

linear ghost state

state machine 


## Approaches

ABA problem 

Michael and Scott queue

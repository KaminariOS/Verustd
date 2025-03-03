# Verifying std::collections With Verus

## Introduction

<!-- Deductive program verification based on Floyd-Hoare logic can formally prove the partial correctness of a program, but it requires significant verification expertise. The expertise challenge exacerbates when extending the axiomatic basis of Floyd-Hoare logic to address modern programming paradigms. Separation logic introduces spatial reasoning principles to handle mutable heap-allocated data structures, significantly increasing the complexity of invariant specification. Further extension to concurrent separation logic (O’Hearn, 2007; Brookes, 2004) introduces additional layers of abstraction—such as resource ownership and thread isolation—to reason about shared-memory concurrency, compounding the need for expertise in both program semantics and proof engineering. Just as application programming interfaces (APIs) abstract the internal implementation details of software libraries—enabling developers to utilize their functionality without requiring knowledge of their underlying mechanisms—formal verification tools ought to provide analogous abstractions(VPI: verification programming interface) that allow programmers to leverage rigorous correctness guarantees without necessitating expertise in the underlying verification logic or formal proof techniques.   -->

Verus is an SMT-based deductive verifier for verifying Rust programs.   




push button

well-formedness

atomic invariant 

## State of the Art
model checking

heroic effort

## Background
// Need an Introduction of Concurrent separation logic here.


Rust affine type system ownership mainstream

lower proof-to-code ratio

### Verus 

permission-based reasoning

efficient SMT encoding

linear ghost state

### Limitation 


### VerusSync


state machine 


## Approaches

### Specification

### 


# Verust: Verifying Rust Standard Library

## Running Verification

To run verification, invoke Verus with the crate-type library on the `src/lib.rs` file:

```
$ verus --crate-type=lib src/lib.rs
```
## Cross-crate verification
Currently, Verus works by invoking `rustc` and [Cargo support](https://github.com/verus-lang/verus/pull/1475) is on the way. 

To support cross-crate verification, we need to run commands like these:

For the lib crate
```
rust_verify --crate-type=lib --export vl.vir src/lib.rs
```

For the main crate
```
rust_verify src/main.rs --extern=verified_lib -L target/debug/deps --import verified_lib=../verified_lib/vl.vir
```

## Limitations of Verus
- Mutable reference(`&mut T`) as return value or struct field not supported 
- `&mut data[i]` not supported
- Deref type conversion not supported
- Comparison of non SMT-arith types not supported
- Insufficient external axioms for `std`

## Difficulties for verification of `std`
- Language items
- `std`-only features 
- High level invariants

## How Verus can help `std`
- Ghost code: specifies all safety invariants explicitly and statically checks them 
- Eliminates debug runtime asserts
- Removes redundant safety abstractions 

## Hints for verifying data structures
- Select an abstract model(`impl View`) of the target data structure. For example, `BinaryHeap<T>` can be represented as `Seq<T>` 
- Define a well-formedness specification(usually recursively) in terms of the abstract model. For example, in a `BinaryHeap`, both left and right children are less than or equal to their parent. For all `pub fn x(&mut self, ...)` API, the precondition and postcondition both include well-formedness.  
- Write external specifications for functions, types and traits that are out of the scope of verification.
- Use assumptions to temporarily finish proofs.

## Verus features 
The `examples` directory contains small code snippets we write for testing Verus features. 

## Reference
1. [Verus Doc](https://verus-lang.github.io/verus/guide/)
1. [Vstd doc](https://verus-lang.github.io/verus/verusdoc/vstd/)
1. [Verification Challenges of Rust std](https://model-checking.github.io/verify-rust-std)
1. [Kani overview](https://model-checking.github.io/kani-verifier-blog/2023/08/03/turbocharging-rust-code-verification.html)
1. [The Rust Security Advisory Database](https://rustsec.org/advisories/)
1. [Too many lists](https://rust-unofficial.github.io/too-many-lists/fifth-miri.html)
1. [Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html)
1. [Systems Verification](https://tchajed.github.io/sys-verif-fa24/)

//! # The Rust Core Library
//!
//! The Rust Core Library is the dependency-free[^free] foundation of [The
//! Rust Standard Library](../std/index.html). It is the portable glue
//! between the language and its libraries, defining the intrinsic and
//! primitive building blocks of all Rust code. It links to no
//! upstream libraries, no system libraries, and no libc.
//!
//! [^free]: Strictly speaking, there are some symbols which are needed but
//!          they aren't always necessary.
//!
//! The core library is *minimal*: it isn't even aware of heap allocation,
//! nor does it provide concurrency or I/O. These things require
//! platform integration, and this library is platform-agnostic.
//!
//! # How to use the core library
//!
//! Please note that all of these details are currently not considered stable.
//!
// FIXME: Fill me in with more detail when the interface settles
//! This library is built on the assumption of a few existing symbols:
//!
//! * `memcpy`, `memmove`, `memset`, `memcmp`, `bcmp`, `strlen` - These are core memory routines
//!   which are generated by Rust codegen backends. Additionally, this library can make explicit
//!   calls to `strlen`. Their signatures are the same as found in C, but there are extra
//!   assumptions about their semantics: For `memcpy`, `memmove`, `memset`, `memcmp`, and `bcmp`, if
//!   the `n` parameter is 0, the function is assumed to not be UB, even if the pointers are NULL or
//!   dangling. (Note that making extra assumptions about these functions is common among compilers:
//!   [clang](https://reviews.llvm.org/D86993) and [GCC](https://gcc.gnu.org/onlinedocs/gcc/Standards.html#C-Language) do the same.)
//!   These functions are often provided by the system libc, but can also be provided by the
//!   [compiler-builtins crate](https://crates.io/crates/compiler_builtins).
//!   Note that the library does not guarantee that it will always make these assumptions, so Rust
//!   user code directly calling the C functions should follow the C specification! The advice for
//!   Rust user code is to call the functions provided by this library instead (such as
//!   `ptr::copy`).
//!
//! * Panic handler - This function takes one argument, a `&panic::PanicInfo`. It is up to consumers of this core
//!   library to define this panic function; it is only required to never
//!   return. You should mark your implementation using `#[panic_handler]`.
//!
//! * `rust_eh_personality` - is used by the failure mechanisms of the
//!    compiler. This is often mapped to GCC's personality function, but crates
//!    which do not trigger a panic can be assured that this function is never
//!    called. The `lang` attribute is called `eh_personality`.

// Since core defines many fundamental lang items, all tests live in a
// separate crate, coretests (library/coretests), to avoid bizarre issues.
//
// Here we explicitly #[cfg]-out this whole crate when testing. If we don't do
// this, both the generated test artifact and the linked libtest (which
// transitively includes core) will both define the same set of lang items,
// and this will cause the E0152 "found duplicate lang item" error. See
// discussion in #50466 for details.
//
// This cfg won't affect doc tests.

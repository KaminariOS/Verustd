// use vstd::prelude::*;
// use vstd::raw_ptr::*;
// use vstd::layout::valid_layout;
//
// verus!{
//
// /// Deallocate with the global allocator.
// // #[cfg(feature = "alloc")]
// #[verifier::external_body]
// pub fn reallocate(
//     p: *mut u8,
//     size: usize,
//     align: usize,
//     new_size: usize,
//     Tracked(pt): Tracked<PointsToRaw>,
//     Tracked(dealloc): Tracked<Dealloc>,
//     ) -> (new_pt: (*mut u8, Tracked<PointsToRaw>, Tracked<Dealloc>)) 
//         requires 
//             valid_layout(size, align),
//             size != 0,
//             new_size != 0,
//             dealloc.addr() == p.addr(),
//             dealloc.size() == size,
//             dealloc.align() == align,
//             dealloc.provenance() == pt.provenance(),
//             pt.is_range(dealloc.addr() as int, dealloc.size() as int),
//             p@.provenance == dealloc.provenance(),
//         ensures
//             new_pt.1@.is_range(new_pt.0.addr() as int, new_size as int),
//             new_pt.2@@ == (DeallocData {
//                     addr: new_pt.0.addr(),
//                     size: new_size as nat,
//                     align: align as nat,
//                     provenance: new_pt.1@.provenance(),
//                 }),
//             new_pt.0.addr() as int % align as int == 0,
//             new_pt.0@.metadata == Metadata::Thin,
//             new_pt.0@.provenance == new_pt.1@.provenance(),
//             opens_invariants none
// {
//     // SAFETY: valid_layout is a precondition
//     let layout = unsafe { alloc::alloc::Layout::from_size_align_unchecked(size, align) };
//     // SAFETY: size != 0
//     let p = unsafe { ::alloc::alloc::realloc(p, layout, new_size) };
//     if p == core::ptr::null_mut() {
//         std::process::abort();
//     }
//     if p == core::ptr::null_mut() {
//         std::process::abort();
//     }
//     (p, Tracked::assume_new(), Tracked::assume_new())
// }
//
// }

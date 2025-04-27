use vstd::prelude::*;                // Import Verus standard library preludes
use vstd::invariant::*;              // Import invariant support for maintaining conditions
use vstd::cell::*;                   // Import PCell module for verifiable mutable cells

verus!{

// Define a ghost struct to represent the invariant "EvenCell" (used only at verification time)
ghost struct EvenCell { }

// Implement the invariant predicate for EvenCell to ensure that cell contents are always even
impl InvariantPredicate<CellId, PointsTo<u8>> for EvenCell {
    // Specification function defining the invariant conditions
    open spec fn inv(cell_id: CellId, points_to: PointsTo<u8>) -> bool {
        // Invariant: the cell ID matches and the cell content is initialized and even
        points_to.id() == cell_id
          && (match points_to.mem_contents() {
              MemContents::Uninit => false,           // Must be initialized
              MemContents::Init(x) => x % 2 == 0,      // Must be an even number
          })
    }
}

// Function to increment the cell content by 2 while preserving the even invariant
fn add_2(
    cell: &PCell<u8>,
    Tracked(inv): Tracked<&LocalInvariant<CellId, PointsTo<u8>, EvenCell>>
)
    // Precondition: ensure the invariant matches the cell ID
    requires inv.constant() == cell.id(),
{
    // Open the invariant scope to safely access and mutate cell contents
    open_local_invariant!(inv => points_to => {
        // Confirm cell is initialized and even before modification
        assert(points_to.is_init());
        assert(points_to.value() % 2 == 0);

        // Retrieve current value from the cell
        let x = cell.take(Tracked(&mut points_to));
        assert(x % 2 == 0);  // Verify the retrieved value is even

        // Calculate the new value, handle potential overflow (254 + 2 wraps to 0)
        let x_plus_2 = if x == 254 { 0 } else { x + 2 };

        // Update the cell with the new even value
        cell.put(Tracked(&mut points_to), x_plus_2);

        // Ensure invariant still holds after updating
        assert(points_to.is_init());
        assert(points_to.value() % 2 == 0);
    });
}

// Example usage: main function demonstrating invariant setup and repeated addition
fn main() {
    // Create a new PCell initialized to 4 with tracking for the points_to handle
    let (cell, Tracked(points_to)) = PCell::new(4);

    // Establish a local invariant for the cell with an arbitrary namespace
    let tracked inv = LocalInvariant::new(
        cell.id(),
        points_to,
        1337 /* arbitrary namespace */);

    // Call the add_2 function multiple times, maintaining the invariant each time
    add_2(&cell, Tracked(&inv));
    add_2(&cell, Tracked(&inv));
    add_2(&cell, Tracked(&inv));
}

}

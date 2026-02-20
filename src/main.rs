//! HW02 -- Collatz Conjecture Explorer
//! See README.md for more details

// * Declares that the q1 module exists (found in src/q1/)
mod q1;

// * Imports individual functions from the q1 module.
// * Feel free to import more/less functions in your final `main.rs` to test your implementations!
 use q1::{average_collatz_steps, collatz_max_value, collatz_steps, longest_collatz_in_range};


// TODO: Use your existing functions in main!
fn main() {

    println!("=== Collatz Conjecture Explorer ===\n");
    println!("is_even(6) = {}", q1::is_even(6));
    println!("next_collatz(57) = {}", q1::next_collatz(57))
    // Demonstrate the surprising behavior of n = 27
    // Report the number of steps to reach 1 and the highest value hit.
    // (1 point)

    //todo!()
    
    // Find which number in [1, 100] has the longest sequence and report the
    // number and the number of steps. (1 point)
    //todo!()

    // Calculate the average number of steps to reach 1 across the range [1, 100].
    // (1 point)
    
    //todo!()
}

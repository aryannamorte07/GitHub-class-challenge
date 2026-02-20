//! Put all code for individual functions here!

pub fn is_even(n: i64) -> bool {
    // TODO: Return true if n is even, false otherwise
    if n%2 == 0 { 
        return true;
    }
    else {
        
    } 
        return false;
    }


pub fn next_collatz(n: i64) -> i64 {
    // TODO: Apply the Collatz rule once:
    //   - If n is even, return n / 2
    if n%2 == 0 { 
        return true
    }
    else { 
        return false
    }
    //   - If n is odd, return 3 * n + 1
    
}

pub fn collatz_steps(n: i64) -> u32 {
    // TODO: Count how many steps it takes for the Collatz sequence starting at n to reach 1
    //   - If n is already 1, return 0


    todo!()
    //   - Otherwise, repeatedly apply next_collatz and count the steps

     
}

pub fn collatz_max_value(n: i64) -> i64 {
    // TODO: Find the highest value reached in the Collatz sequence starting at n
    //   - Track the maximum value seen as you iterate toward 1
    //   - The starting value n should be considered as a candidate for the max


}

pub fn longest_collatz_in_range(start: i64, end: i64) -> i64 {
    // TODO: Find which starting number in [start, end] (inclusive) takes the most
    //       Collatz steps to reach 1. Return that starting number.
    //   - If there is a tie, return the first (smallest) number with the most steps

     todo!()
}

pub fn average_collatz_steps(start: i64, end: i64) -> f64 {
    // TODO: Calculate the average (mean) number of Collatz steps for all starting
    //       values from start to end (inclusive)
    //   - Use type casting (as f64) to compute the floating-point average

     todo!()
}

// !-- DO NOT REMOVE THESE LINES. THIS SETS UP THE `cargo test` TEST CASES -------
#[cfg(test)]
mod tests;

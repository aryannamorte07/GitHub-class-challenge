[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/0riQ5d93)
# DS 210 Homework 2: Collatz Conjecture Explorer

## Instructions

**Submitting** We have invited you to a GitHub Classroom assignment - you now have a starter repo that you should work in to complete your assignment. Please submit your assignment to mark it as finished / ready for grading.

**Deadline** Listed in GitHub Classroom. You can submit up to 48 hours late to receive up to 80% credit on the additions since the on-time due date.

**Citing your sources** You must adhere to all course policies, including the policies on collaboration, academic honesty, and AI use.  In particular, if you collaborated with other students or used AI while working on the assignment, you must cite who/what you used as sources in your writeup and explain briefly how you used them. Reminder regarding collagoration -- you must complete this assignment on your own. Do not share your solutions with other students nor copy from other students. It's ok to discuss ideas with other students, but you must write your own code.

### Repository Setup

In the starter repo, you will see `src` which contains one problem directory (`q1`).  This directory has a `mod.rs` file where you will do your work, with the "function signature" or `fn` line of code specifying its inputs and outputs.  It's your job to fill in these functions and add any additional functions you need to the `mod.rs` file to get it to work.

### Tests (and autograding)

The assignment will be autograded using GitHub Actions. The autograder will check your code for correctness and formatting. You can view the autograder results in the Actions tab of your repository.

We highly recommend you check your code locally. To check your code you can run `cargo test` on the whole project or `cargo test is_even` (for example) to run tests on a particular function.  At the bottom of this file there is also a summary table of all tests that will run and their points values.

The autograder tests should be identical to the tests you run locally. If you pass all the tests locally, you will pass the autograder.

### Git Workflow Requirements

**Branching** (autograded — requires at least 6 branches: `main` + 5 feature branches)

1. Create a separate feature branch for each subproblem (5 branches, e.g. `subproblem-1`, `subproblem-2`, etc.)
2. Implement each subproblem entirely on its feature branch
3. Merge each feature branch back to `main` when complete so that all parts of the problem are visible in `main`
4. Do **not** delete your feature branches after merging — the autograder checks that all 6 branches exist

**Commit Requirements:** (autograded — requires at least 10 commits)

Each subproblem should have at least 1 commit on its feature branch plus 1 merge commit into `main`, giving a minimum of 10 commits across all 5 subproblems. In practice you should make more than the minimum to show your development process!

1. Use clear, descriptive commit messages
2. Commits should represent logical units of work (e.g., "Implement is_even function", "Add next_collatz with conditional logic")
3. Do not commit all changes at once - show your development process!

***OBSERVATIONS.md File Requirement:***

1. Create a new file called `OBSERVATIONS.md` in the root of your repository.
2. Include an overview of your solution, documenting any assumptions or design decisions you made (1 point)
3. Comment on the behavior of the Collatz sequences you observed. What surprised you? Which starting numbers produced unexpectedly long sequences or high peak values? (1 point)
4. Add a brief reflection on what you learned while working through the assignment. This could be about Rust syntax, the Collatz conjecture, git and GitHub processes, etc. (1 point)

Note that items 2, 3 and 4 are each worth 1 point for a total of 3 points.

## Background: The Collatz Conjecture

The [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) (also known as the **3n+1 problem**) is one of the most famous unsolved problems in mathematics. The rule is simple:

- Start with any positive integer **n**
- If **n** is even, divide it by 2
- If **n** is odd, multiply by 3 and add 1
- Repeat until you reach 1

The conjecture states that **every** positive integer will eventually reach 1, but nobody has been able to prove it! It has been verified computationally for enormous numbers, yet a proof remains elusive.

Some sequences are surprisingly long — for example, starting at **n = 27**, the sequence takes **111 steps** and climbs as high as **9232** before eventually descending back to 1.

## Problem 1 - Explore the Collatz Conjecture

In this problem, you will build functions to explore Collatz sequences, find patterns, and compute statistics about them.

### Subproblem 1: Building blocks

```rust
pub fn is_even(n: i64) -> bool         // return true if n is even
pub fn next_collatz(n: i64) -> i64      // apply the Collatz rule once
```

### Subproblem 2: Sequence length

Count how many steps it takes for the sequence to reach 1:

```rust
pub fn collatz_steps(n: i64) -> u32
```

### Subproblem 3: Peak value

Find the highest value the sequence reaches before coming back down to 1:

```rust
pub fn collatz_max_value(n: i64) -> i64
```

### Subproblem 4: Search a range

Find which starting number in a range `[start, end]` takes the most steps to reach 1:

```rust
pub fn longest_collatz_in_range(start: i64, end: i64) -> i64
```

### Subproblem 5: Compute a statistic

Calculate the average number of Collatz steps across all starting values in a range:

```rust
pub fn average_collatz_steps(start: i64, end: i64) -> f64
```

## Autograder Test Cases

| Name                                     | Points | Description                                                                   |
|------------------------------------------|--------|-------------------------------------------------------------------------------|
| `is_even_test_cases`                     | 1      | Tests is_even with positive, negative, and zero values.                       |
| `next_collatz_test_cases`                | 1      | Tests next_collatz for both even and odd inputs.                              |
| `collatz_steps_test_cases`               | 1      | Tests collatz_steps with known sequence lengths including n=27.               |
| `collatz_max_value_test_cases`           | 1      | Tests collatz_max_value with known peak values including n=27 reaching 9232.  |
| `longest_collatz_in_range_test_cases`    | 1      | Tests longest_collatz_in_range with known ranges.                             |
| `average_collatz_steps_test_cases`       | 1      | Tests average_collatz_steps with known averages.                              |
| `test_rustfmt_passes`                    | 1      | Checks that code is properly formatted (`cargo fmt -- --check`).             |
| `test_clippy_passes`                     | 1      | Checks that code passes clippy lints with no warnings.                       |
| `COMMIT_COUNT`                           | 1      | Ensures at least 10 commits.                                                 |
| `BRANCH_COUNT`                           | 1      | Ensures at least 6 branches (main + 5 feature branches).                     |

## Manual Grading

- Your OBSERVATIONS.md file.

- Main function output of the demonstration and the results of the calculations.

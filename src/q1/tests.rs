use crate::q1::*;

/// Tests is_even with positive, negative, and zero values.
#[test]
fn is_even_test_cases() {
    assert_eq!(is_even(2), true);
    assert_eq!(is_even(3), false);
    assert_eq!(is_even(0), true);
    assert_eq!(is_even(-4), true);
    assert_eq!(is_even(-3), false);
    assert_eq!(is_even(1), false);
    assert_eq!(is_even(100), true);
    assert_eq!(is_even(99), false);
}

/// Tests next_collatz for both even and odd inputs.
#[test]
fn next_collatz_test_cases() {
    // Even numbers: n / 2
    assert_eq!(next_collatz(2), 1);
    assert_eq!(next_collatz(4), 2);
    assert_eq!(next_collatz(10), 5);
    assert_eq!(next_collatz(16), 8);

    // Odd numbers: 3n + 1
    assert_eq!(next_collatz(1), 4);
    assert_eq!(next_collatz(3), 10);
    assert_eq!(next_collatz(5), 16);
    assert_eq!(next_collatz(7), 22);
    assert_eq!(next_collatz(27), 82);
}

/// Tests collatz_steps with known sequence lengths.
#[test]
fn collatz_steps_test_cases() {
    // n = 1: already at 1, 0 steps
    assert_eq!(collatz_steps(1), 0);

    // n = 2: 2 -> 1 = 1 step
    assert_eq!(collatz_steps(2), 1);

    // n = 4: 4 -> 2 -> 1 = 2 steps
    assert_eq!(collatz_steps(4), 2);

    // n = 3: 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1 = 7 steps
    assert_eq!(collatz_steps(3), 7);

    // n = 5: 5 -> 16 -> 8 -> 4 -> 2 -> 1 = 5 steps
    assert_eq!(collatz_steps(5), 5);

    // n = 27: the famous case — 111 steps!
    assert_eq!(collatz_steps(27), 111);
}

/// Tests collatz_max_value with known peak values.
#[test]
fn collatz_max_value_test_cases() {
    // n = 1: sequence is just [1], max = 1
    assert_eq!(collatz_max_value(1), 1);

    // n = 2: sequence is [2, 1], max = 2
    assert_eq!(collatz_max_value(2), 2);

    // n = 3: sequence reaches 16
    assert_eq!(collatz_max_value(3), 16);

    // n = 7: sequence reaches 52
    assert_eq!(collatz_max_value(7), 52);

    // n = 27: sequence climbs all the way to 9232!
    assert_eq!(collatz_max_value(27), 9232);
}

/// Tests longest_collatz_in_range with known ranges.
#[test]
fn longest_collatz_in_range_test_cases() {
    // Range [1, 1]: only one number
    assert_eq!(longest_collatz_in_range(1, 1), 1);

    // Range [1, 5]: steps are 0, 1, 7, 2, 5 -> longest is 3 (7 steps)
    assert_eq!(longest_collatz_in_range(1, 5), 3);

    // Range [1, 10]: steps are 0,1,7,2,5,8,16,3,19,6 -> longest is 9 (19 steps)
    assert_eq!(longest_collatz_in_range(1, 10), 9);

    // Range [5, 8]: steps are 5,8,16,3 -> longest is 7 (16 steps)
    assert_eq!(longest_collatz_in_range(5, 8), 7);
}

/// Tests average_collatz_steps with known averages.
#[test]
fn average_collatz_steps_test_cases() {
    // Range [1, 1]: just n=1, which has 0 steps
    let avg_1 = average_collatz_steps(1, 1);
    assert!(
        (avg_1 - 0.0).abs() < 0.001,
        "average_collatz_steps(1, 1) should be 0.0, got {avg_1}"
    );

    // Range [1, 5]: steps are 0 + 1 + 7 + 2 + 5 = 15, avg = 3.0
    let avg_5 = average_collatz_steps(1, 5);
    assert!(
        (avg_5 - 3.0).abs() < 0.001,
        "average_collatz_steps(1, 5) should be 3.0, got {avg_5}"
    );

    // Range [1, 10]: steps are 0+1+7+2+5+8+16+3+19+6 = 67, avg = 6.7
    let avg_10 = average_collatz_steps(1, 10);
    assert!(
        (avg_10 - 6.7).abs() < 0.001,
        "average_collatz_steps(1, 10) should be 6.7, got {avg_10}"
    );
}

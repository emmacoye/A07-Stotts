fn main() {
    // Known-answer checks — these will panic if your functions are wrong
    assert_eq!(collatz_length(1), 1, "length of 1 should be 1");
    assert_eq!(collatz_length(6), 9, "length of 6 should be 9");
    assert_eq!(collatz_length(27), 112, "length of 27 should be 112");
    assert_eq!(collatz_length(2), 2);
    assert_eq!(collatz_length(3), 8);
    println!("collatz_length checks passed.");

    let answer = longest_collatz(1_000_000);
    assert_eq!(answer, 837799, "longest below 1M should be 837799");
    println!("longest_collatz(1_000_000) = {}", answer);

    println!("All checks passed.");
}

fn collatz_length(n: u64) -> u64 {
    fn run(n: u64, steps: u64) -> u64 {
        match n {
            1 => steps,
            _ => match n % 2 {
                0 => run(n / 2, steps + 1),
                _ => run(3 * n + 1, steps + 1),
            },
        }
    }

    run(n, 1)
}

fn longest_collatz(limit: u64) -> u64 {
    todo!()
}

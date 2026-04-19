# A07: Collatz Sequence

### Team: Emma Coye, Manasi Chaudhary, Elizabeth Coats

To run: `cargo build && cargo run`
This program implements functions to compure the length of a Collatz sequence and to find the number below a given limit that gives the longest sequence. 

### Notes

We had to use the `tailcall` crate in order to avoid stack overflow. Even though our functions are tail-recursive, we learned that Rust does not have native support for Tail-Call Optimization. This means that the compiler doesn't guarantee that tail-recursive function calls will only take up one stack frame. We found this to be true, so since we were not allowed to use loops, opted for `tailcall` instead. This introduces the risk of stack overflow, which would not exist if we used a loop (since loops reuse a single stack frame as opposed to creating a new one per call).

We also discovered (but could not use) an experimental feature `become` in Rust Nightly, which has not been released as a stable feature yet. This feature directly replaces the old stack frame with the new one during tail recursion.


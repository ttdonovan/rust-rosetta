// Implements an iterable version of http://rosettacode.org/wiki/Hofstadter_Q_sequence
#![feature(associated_types)]

// Define a struct which stores the state for the iterator.
struct HofstadterQ {
    next: uint,
    memoize_vec: Vec<uint>
}

impl HofstadterQ {
  // Define a constructor for the struct.
    fn new() -> HofstadterQ {
        HofstadterQ { next: 1, memoize_vec: vec![1] }
    }
}

// Implement the hofstadter q iteration sequence.
impl Iterator for HofstadterQ {
    type Item = uint;
    // This gets called to fetch the next item of the iterator.
    fn next(&mut self) -> Option<uint> {
        // Cache the current value.
        self.memoize_vec.push(self.next);
        // And then calculate the 'next'.
        // First, make the four recursive calls.
        let current: uint = self.memoize_vec.len();
        let rec_call_1: uint = self.memoize_vec[current - 1];
        let rec_call_2: uint = self.memoize_vec[current - 2];
        let rec_call_3: uint = self.memoize_vec[current - rec_call_1];
        let rec_call_4: uint = self.memoize_vec[current - rec_call_2];
        // Then update self.next and return it.
        self.next = rec_call_3 + rec_call_4;
        Some(self.next)
    }
}

#[cfg(not(test))]
fn main() {
    // Set up the iterable.
    let hof: HofstadterQ = HofstadterQ::new();
    // The number of terms we want from the iterator.
    let upto: uint = 1000;
    // Create the iterator.
    let mut it = hof.take(upto - 2);
    // Print the base values.
    println!("H(1) = 1");
    println!("H(2) = 1");
    // Print the rest of the sequence.
    for i in range(3u, 1+upto) {
        println!("H({}) = {}", i, it.next().unwrap());
    }
}

#[test]
fn test_first_ten() {
    // Set up the iterable.
    let hof: HofstadterQ = HofstadterQ::new();
    // Create the iterator.
    let mut it = hof.take(10);
    // Test that the first ten values are as expected
    // The first two values are hardcoded, so no need to test those.
    let hofstadter_q_expected = vec![2,3,3,4,5,5,6,6];
    for i in range(0u, 8) {
        assert_eq!(hofstadter_q_expected[i], it.next().unwrap());
    }
}

#[test]
fn test_thousandth() {
    // Set up the iterable.
    let hof: HofstadterQ = HofstadterQ::new();
    // The number of terms we want from the iterator.
    let upto: uint = 1000;
    // Create the iterator.
    let mut it = hof.take(upto - 2);
    let expected: uint = 502;
    // Test that the upto-th term is as expected.
    for _ in range(3u, upto) {
        it.next();
    }
    assert_eq!(expected, it.next().unwrap());
}

// http://rosettacode.org/wiki/Sierpinski_triangle

use std::iter::{repeat, range_step};

fn main() {
    let order = 4u;
    let height = 1u << order;
    let mut state: Vec<bool> = repeat(true).take(height + 1).collect();

    // Compute the triangle line-by-line by viewing it as Pascal's triangle (mod 2)
    for i in range(0u, height) {
        for _ in range(0u, height - i - 1) {
            print!(" ");
        }

        for j in range(0u, i + 1) {
            print!(" {}", if state[j] { "*" } else { " " });
        }

        // Compute the next line
        for j in range_step(i as int, 0, -1) {
            state[j as uint] ^= state[(j - 1) as uint];
        }

        print!("\n");
    }
}


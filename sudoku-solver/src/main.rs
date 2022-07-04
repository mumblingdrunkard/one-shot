// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This Source Code Form is “Incompatible With Secondary Licenses”, as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Input was not a string");
    let n: usize = n.trim().parse().unwrap();
    let mut boards = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Input was not a string");
        boards.push(Board::from_str(&s.trim()));
    }

    // want to multithread this? turn this loop into a parallel iterator with rayon
    for mut b in boards {
        b.solve();
        println!("{}\n", b);
    }
}

struct Board {
    data: [u8; 81],
}

impl Board {
    // returns an array of length 10 indicating the valid numbers for a given row and column such
    // that n is valid if array[n] is true.
    fn valid_for_position(&self, r: usize, c: usize) -> [bool; 10] {
        // let all numbers be valid, then invalidate numbers as they are found in the row, column,
        // and box of the given coordinate
        let mut valid = [true; 10];
        valid[0] = false;

        // column
        (0..9)
            .map(|r| self[r][c])
            .for_each(|n| valid[n as usize] = false);

        // row
        (0..9)
            .map(|c| self[r][c])
            .for_each(|n| valid[n as usize] = false);

        // box ( don't worry about the index calculation :) )
        (0..9)
            .map(|k| self[(r / 3) * 3 + k / 3][(c / 3) * 3 + k % 3])
            .for_each(|n| valid[n as usize] = false);

        valid
    }

    // returns a Vec of all the empty slots on the board
    fn empty_slots(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (n == 0).then(|| (i / 9, i % 9)))
            .collect()
    }

    // solves the board and returns true if a valid solution was found
    fn solve(&mut self) -> bool {
        let slots = self.empty_slots();
        self.solve_internal(&slots)
    }

    // recursively solves the board given the slots to be filled
    fn solve_internal(&mut self, slots: &[(usize, usize)]) -> bool {
        if slots.len() == 0 {
            // all slots are filled with no conflicts
            return true;
        }

        // try to insert valid values and solve one level deeper
        let (r, c) = slots[0];
        self.valid_for_position(r, c)
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.then(|| i as u8))
            .any(|n| {
                self[r][c] = n;
                self.solve_internal(&slots[1..]) // solve remaining slots
            })
            || {
                // this block evaluates only if the `||` did not short-circuit; i.e. the previous
                // expression was false
                self[r][c] = 0;
                false
            }
    }

    fn new() -> Self {
        Self { data: [0; 81] }
    }

    fn from_str(s: &str) -> Self {
        if s.len() != 81 {
            panic!("Input string is of incorrect length");
        }

        let mut b = Self::new();
        b.data
            .iter_mut()
            .zip(s.chars())
            .for_each(|(n, c)| *n = c.to_digit(10).unwrap() as u8);
        b
    }
}

impl std::ops::Index<usize> for Board {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[(index * 9)..(index * 9 + 9)]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[(index * 9)..(index * 9 + 9)]
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        for r in 0..9 {
            for c in 0..9 {
                result.push_str(&format!("{} ", self[r][c]));
                if c % 3 == 2 && c != 8 {
                    result.push_str("| ");
                }
            }
            if r != 8 {
                result.push_str("\n");
            }
            if r % 3 == 2 && r != 8 {
                result.push_str("------|-------|------\n");
            }
        }
        write!(f, "{}", result) // write in a single call so we can use the Result here instead of
                                // accumulating a Result through several write! invocations
    }
}

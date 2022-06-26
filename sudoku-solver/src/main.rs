// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This Source Code Form is “Incompatible With Secondary Licenses”, as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard

fn main() {
    let mut board = Board::test_board1();

    let valid = board.solve();

    for r in 0..9 {
        for c in 0..9 {
            print!("{} ", board.data[r * 9 + c]);
            if c % 3 == 2 {
                print!(" ");
            }
        }
        print!("\n");
        if r % 3 == 2 {
            print!("\n");
        }
    }

    println!("{}", valid);
}

struct Board {
    data: [i8; 81],
}

impl Board {
    #[allow(unused)]
    fn test_board1() -> Self {
        Self {
            #[rustfmt::skip]
            data: [0, 0, 0,  7, 9, 0,  0, 5, 0,
                   3, 5, 2,  0, 0, 8,  0, 4, 0,
                   0, 0, 0,  0, 0, 0,  0, 8, 0,

                   0, 1, 0,  0, 7, 0,  0, 0, 4,
                   6, 0, 0,  3, 0, 1,  0, 0, 8,
                   9, 0, 0,  0, 8, 0,  0, 1, 0,

                   0, 2, 0,  0, 0, 0,  0, 0, 0,
                   0, 4, 0,  5, 0, 0,  8, 9, 1,
                   0, 8, 0,  0, 3, 7,  0, 0, 0],
        }
    }

    #[allow(unused)]
    fn test_board2() -> Self {
        Self {
            #[rustfmt::skip]
            data: [0, 2, 0,  0, 0, 0,  0, 0, 0,
                   0, 0, 0,  6, 0, 0,  0, 0, 3,
                   0, 7, 4,  0, 8, 0,  0, 0, 0,

                   0, 0, 0,  0, 0, 3,  0, 0, 2,
                   0, 8, 0,  0, 4, 0,  0, 1, 0,
                   6, 0, 0,  5, 0, 0,  0, 0, 0,

                   0, 0, 0,  0, 1, 0,  7, 8, 0,
                   5, 0, 0,  0, 0, 9,  0, 0, 0,
                   0, 0, 0,  0, 0, 0,  0, 4, 0],
        }
    }

    #[allow(unused)]
    fn test_board_empty() -> Self {
        Self { data: [0; 81] }
    }

    // returns a Vec of all the numbers that are valid for the given position on the board
    fn valid_for_position(&self, r: usize, c: usize) -> Vec<i8> {
        // let all numbers be valid, then invalidate numbers as they are found in the row, column,
        // and box of the given coordinate
        let mut valid = [true; 9];

        // column
        for r in 0..9 {
            let n = self.data[r * 9 + c];
            match n {
                1..=9 => valid[(n - 1) as usize] = false,
                _ => {}
            }
        }

        // row
        for c in 0..9 {
            let n = self.data[r * 9 + c];
            match n {
                1..=9 => valid[(n - 1) as usize] = false,
                _ => {}
            }
        }

        // box
        let (lr, lc) = (r / 3, c / 3);
        for k in 0..9 {
            let (r, c) = (lr * 3 + k / 3, lc * 3 + k % 3);
            let i = r * 9 + c;
            let n = self.data[i];
            match n {
                1..=9 => valid[(n - 1) as usize] = false,
                _ => {}
            }
        }

        // map the array of boolean values to an array of numbers derived from the indices that
        // still contain `true`
        valid
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some((i + 1) as i8) } else { None })
            .collect::<Vec<i8>>()
    }

    // returns a Vec of all the empty slots on the board
    fn empty_slots(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| if n == 0 { Some((i / 9, i % 9)) } else { None })
            .collect()
    }

    // solves the board and returns true if a valid solution was found
    fn solve(&mut self) -> bool {
        let slots = self.empty_slots();
        self.solve_internal(&slots, 0)
    }

    // recursively solves the board given the slots to be filled
    fn solve_internal(&mut self, slots: &[(usize, usize)], depth: usize) -> bool {
        if depth == slots.len() {
            // all slots are filled with no conflicts
            true
        } else {
            // try to insert a valid value and solve one level deeper
            let (r, c) = slots[depth];
            let i = r * 9 + c;
            let valid = self.valid_for_position(r, c);
            if valid.len() == 0 {
                // there are no valid numbers for this position, return false
                false
            } else {
                // there are valid numbers for this position, try all of them
                let viable = valid.iter().any(|&n| {
                    self.data[i] = n;
                    self.solve_internal(slots, depth + 1)
                });
                // if all paths from here were invalid, we set the cell to 0 and backtrack
                if !viable {
                    self.data[i] = 0;
                }
                // return whether the path was viable
                viable
            }
        }
    }
}

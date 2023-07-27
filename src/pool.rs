use rand::Rng;
use std::{cmp::min, fmt, ops, thread};

#[allow(dead_code)]
pub struct Pool {
    /// Alive state of each cell, true is alive.
    state: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.state {
            for cell in row {
                if *cell {
                    write!(f, "O").unwrap()
                } else {
                    write!(f, " ").unwrap()
                }
                //write!(f, "{}", if *cell { "O" } else { " " }).unwrap()
            }
            writeln!(f, "").unwrap()
        }
        Ok(())
    }
}
#[allow(dead_code)]
impl ops::AddAssign<Pool> for Pool {
    fn add_assign(&mut self, other: Pool) {
        let height = min(self.height(), other.height());
        let width = min(self.width(), other.width());

        for i in 0..height {
            for j in 0..width {
                self.state[i as usize][j as usize] = self.get_cell(i, j) || other.get_cell(i, j)
            }
        }
    }
}

#[allow(dead_code)]
impl Pool {
    pub fn new(width: u32, height: u32) -> Self {
        const DEFAULT_STATE: bool = false;
        Self {
            state: vec![vec![DEFAULT_STATE; width as usize]; height as usize],
        }
    }

    pub fn from_array<const WIDTH: usize, const HEIGHT: usize>(
        data: &[[bool; HEIGHT]; WIDTH],
    ) -> Self {
        Self {
            state: Self::convert_2d_array_to_vec(&data),
        }
    }

    fn convert_2d_array_to_vec<const WIDTH: usize, const HEIGHT: usize>(
        arr: &[[bool; HEIGHT]; WIDTH],
    ) -> Vec<Vec<bool>> {
        let mut ret = Vec::with_capacity(WIDTH);
        for row in arr.iter() {
            let mut vec_row = Vec::with_capacity(HEIGHT);
            for cell in row.iter() {
                vec_row.push(cell.to_owned());
            }
            ret.push(vec_row)
        }
        ret
    }

    /// Creates a spaceship oriented towards South East.
    pub fn glider_south_east() -> Self {
        const DATA: [[bool; 3]; 3] = [
            [false, true, false],
            [false, false, true],
            [true, true, true],
        ];

        Self::from_array(&DATA)
    }

    /// Creates an acorn.
    pub fn acorn() -> Self {
        const DATA: [[bool; 7]; 3] = [
            [false, true, false, false, false, false, false],
            [false, false, false, true, false, false, false],
            [true, true, false, false, true, true, true],
        ];

        Self::from_array(&DATA)
    }

    pub fn width(&self) -> u32 {
        match self.state.first() {
            Some(first_row) => first_row.len() as u32,
            None => 0,
        }
    }

    pub fn height(&self) -> u32 {
        self.state.len() as u32
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for row in &mut self.state {
            for cell in row {
                *cell = rng.gen_bool(0.5);
            }
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.state {
            for cell in row {
                *cell = false;
            }
        }
    }

    pub fn get_cell(&self, row: u32, column: u32) -> bool {
        self.state[row as usize][column as usize]
    }

    fn count_alive_neighbors(&self, row: u32, column: u32) -> u8 {
        // TODO : maybe wrap edges ?
        let mut count = 0u8;
        let row_start = if row > 0 { row - 1 } else { row };
        let row_end = if row < self.height() - 1 {
            row + 1
        } else {
            row
        };
        let column_start = if column > 0 { column - 1 } else { column };
        let column_end = if column < self.width() - 1 {
            column + 1
        } else {
            column
        };
        for i in row_start..(row_end + 1) {
            for j in column_start..(column_end + 1) {
                if self.state[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
        // Count only neighbors
        if self.state[row as usize][column as usize] {
            count -= 1;
        }
        count
    }

    /// Returns the new state of a given cell, given its current state and the number of alive neighbors
    fn get_next_state(current_state: bool, neighbors_count: u8) -> bool {
        if current_state {
            neighbors_count == 2 || neighbors_count == 3
        } else {
            neighbors_count == 3
        }
    }

    pub fn step(&mut self) {
        const THREAD_COUNT: u32 = 6;
        // The line indices on which each thread will operate, thread i works from index thread_indices[i] included to thread_indices[i] excluded
        let mut thread_indices = vec![0u32; THREAD_COUNT as usize + 1];
        for (thread, row_index) in thread_indices.iter_mut().enumerate() {
            *row_index = (thread as u32) * self.height() / THREAD_COUNT;
        }

        self.state = thread::scope(|s| {
            let mut handles = Vec::with_capacity(THREAD_COUNT as usize);
            // Spawn worker threads that handle subsets of the state, slices along rows.
            for thread in 0..THREAD_COUNT {
                let start = &thread_indices[thread as usize];
                let stop = &thread_indices[thread as usize + 1];
                handles.push(s.spawn(|| {
                    let start = *start;
                    let stop = *stop;
                    // Create a part of the next state
                    let width_part = self.width() as usize;
                    let height_part = (stop - start) as usize;
                    let mut next_state_part = vec![vec![false; width_part]; height_part];

                    for (i, row) in next_state_part.iter_mut().enumerate() {
                        for (j, cell) in row.iter_mut().enumerate() {
                            let complete_state_row_index = i as u32 + start;
                            *cell = Self::get_next_state(
                                self.state[complete_state_row_index as usize][j],
                                self.count_alive_neighbors(complete_state_row_index, j as u32),
                            );
                        }
                    }
                    next_state_part
                }));
            }

            // Join worker threads, concatenate subsets to complete result.
            let mut next_state_parts = Vec::with_capacity(THREAD_COUNT as usize);
            for handle in handles.into_iter() {
                next_state_parts.push(handle.join().unwrap());
            }
            // Return result out of scope.
            next_state_parts.into_iter().flatten().collect()
        });
    }

    pub fn set_cell(&mut self, row: u32, column: u32, state: bool) {
        self.state[row as usize][column as usize] = state;
    }

    pub fn with_offset(&self, row_offset: u32, column_offset: u32) -> Pool {
        let new_width = self.width() + column_offset;
        let new_height = self.height() + row_offset;
        let mut result = Pool::new(new_width, new_height);
        for i in 0..self.height() {
            for j in 0..self.width() {
                result.state[(row_offset + i) as usize][(column_offset + j) as usize] =
                    self.state[i as usize][j as usize];
            }
        }
        return result;
    }

    // TODO fn rotated(&self, angle)->Pool
    // TODO fn mirrored(&self, horizontal:bool, vertical:bool)->Pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step() {
        let mut pool = Pool::new(3, 3);
        pool.set_cell(0, 1, true);
        pool.set_cell(1, 1, true);
        pool.set_cell(2, 1, true);
        pool.step();
        assert_eq!(
            pool.state,
            vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, false]
            ]
        )
    }

    #[test]
    fn with_offset() {
        let mut pool = Pool::new(2, 1);
        pool.set_cell(0, 0, true);
        let pool_with_offset = pool.with_offset(1, 1);
        assert_eq!(
            pool_with_offset.state,
            vec![vec![false, false, false], vec![false, true, false],]
        )
    }
}

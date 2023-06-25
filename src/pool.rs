use rand::Rng;
use std::fmt;

pub struct Pool<const HEIGHT: usize, const WIDTH: usize> {
    /// Alive state of each cell, true is alive.
    state: [[bool; WIDTH]; HEIGHT],
}
impl<const HEIGHT: usize, const WIDTH: usize> Default for Pool<HEIGHT, WIDTH> {
    fn default() -> Self {
        const DEFAULT_STATE: bool = false;
        Self {
            state: [[DEFAULT_STATE; WIDTH]; HEIGHT],
        }
    }
}
impl<const HEIGHT: usize, const WIDTH: usize> fmt::Display for Pool<HEIGHT, WIDTH> {
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
impl<const HEIGHT: usize, const WIDTH: usize> Pool<HEIGHT, WIDTH> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for row in &mut self.state {
            for cell in row {
                *cell = rng.gen_bool(0.5);
            }
        }
    }

    pub fn get_state(&self) -> [[bool; WIDTH]; HEIGHT] {
        self.state
    }

    fn count_alive_neighbors(&self, row: u32, column: u32) -> u8 {
        // TODO : maybe wrap edges ?
        let mut count = 0u8;
        let row_start = if row > 0 { row - 1 } else { row };
        let row_end = if row < HEIGHT as u32 - 1 {
            row + 1
        } else {
            row
        };
        let column_start = if column > 0 { column - 1 } else { column };
        let column_end = if column < WIDTH as u32 - 1 {
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
        let mut next_state = self.state;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                next_state[i][j] = Self::get_next_state(
                    self.state[i][j],
                    self.count_alive_neighbors(i as u32, j as u32),
                );
            }
        }
        self.state = next_state;
    }

    pub fn set_cell(&mut self, row: u32, column: u32, state: bool) {
        self.state[row as usize][column as usize] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_step() {
        let mut pool: Pool<3, 3> = Pool::new();
        pool.set_cell(0, 1, true);
        pool.set_cell(1, 1, true);
        pool.set_cell(2, 1, true);
        pool.step();
        assert_eq!(
            pool.state,
            [
                [false, false, false],
                [true, true, true],
                [false, false, false]
            ]
        )
    }
}

use rand::Rng;
use std::fmt;

#[allow(dead_code)]
pub struct Pool {
    /// Alive state of each cell, true is alive.
    state: Vec<Vec<bool>>,
    // TODO : state should be a vec of vec, state = vec![value; dynamic_size]
    // WIDTH and HEIGHT therefore become simple parameters.
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
impl Pool {
    pub fn new(width: u32, height: u32) -> Self {
        const DEFAULT_STATE: bool = false;
        Self {
            state: vec![vec![DEFAULT_STATE; width as usize]; height as usize],
        }
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
        let mut next_state = self.state.clone();
        for i in 0..self.height() {
            for j in 0..self.width() {
                next_state[i as usize][j as usize] = Self::get_next_state(
                    self.state[i as usize][j as usize],
                    self.count_alive_neighbors(i, j),
                );
            }
        }
        self.state = next_state;
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

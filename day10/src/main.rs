//! https://adventofcode.com/2023/day/10
//!

use std::collections::VecDeque;

fn main() {
    let s = std::fs::read_to_string("day10.in").unwrap();
    let mut pm = PipeMap::new(&s);
    println!("{}", pm.search_farthest());
}

#[derive(Debug)]
struct PipeMap {
    tiles: Vec<Vec<char>>,
    memo: Vec<Vec<i32>>,
    q: VecDeque<(usize, usize)>,
}

impl PipeMap {
    pub fn new(s: &str) -> PipeMap {
        let tiles: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let len = tiles.len();
        PipeMap {
            tiles,
            memo: Self::create_clean_memo(len),
            q: VecDeque::new(),
        }
    }

    pub fn find_s(&self) -> (usize, usize) {
        for (row_idx, row_vec) in self.tiles.iter().enumerate() {
            for (col_idx, col_char) in row_vec.iter().enumerate() {
                match col_char {
                    'S' => return (row_idx, col_idx),
                    _ => continue,
                }
            }
        }
        panic!("no s, invalid input!")
    }

    pub fn create_clean_memo(size: usize) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        v.resize(size, -1);
        let mut v_mat = Vec::new();
        v_mat.resize(size, v);
        v_mat
    }

    fn is_pipe(c: char) -> bool {
        c != '.'
    }

    fn tile_within_map(&self, row: i32, col: i32) -> bool {
        row >= 0
            && col >= 0
            && row < self.tiles.len().try_into().unwrap()
            && col < self.tiles.len().try_into().unwrap()
            && Self::is_pipe(self.tiles[row as usize][col as usize])
    }

    fn has_visited(&self, new_row: usize, new_col: usize) -> bool {
        self.memo[new_row][new_col] != -1
    }

    fn go_to_next(
        &mut self,
        row_offset: i32,
        col_offset: i32,
        origin: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (new_row, new_col) = (origin.0 as i32 + row_offset, origin.1 as i32 + col_offset);
        if !self.tile_within_map(new_row, new_col) {
            return None;
        }
        let (new_row, new_col) = (new_row as usize, new_col as usize);
        if self.has_visited(new_row, new_col) {
            return None;
        }
        self.memo[new_row][new_col] = self.memo[origin.0][origin.1] + 1;
        self.q.push_back((new_row, new_col));
        Some((new_row, new_col))
    }

    pub fn tile_unreachable(
        &self,
        origin: (usize, usize),
        row_offset: i32,
        col_offset: i32,
        pattern: &str,
    ) -> bool {
        let (new_row, new_col) = (origin.0 as i32 + row_offset, origin.1 as i32 + col_offset);
        if !self.tile_within_map(new_row, new_col) {
            return true;
        }
        let (new_row_u, new_col_u) = (new_row as usize, new_col as usize);
        pattern.contains(self.tiles[new_row_u][new_col_u])
    }

    fn go_somewhere(
        &mut self,
        origin: (usize, usize),
        row_offset: i32,
        col_offset: i32,
        cannot_be: &str,
    ) -> Option<(usize, usize)> {
        if !self.tile_unreachable(origin, row_offset, col_offset, cannot_be) {
            self.go_to_next(row_offset, col_offset, origin)
        } else {
            None
        }
    }

    fn go_east(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 0, 1, "|FL")
    }

    fn go_west(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 0, -1, "|J7")
    }

    fn go_north(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, -1, 0, "-LJ")
    }

    fn go_south(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 1, 0, "-7F")
    }

    /// use BFS to find the farthest exit.
    pub fn search_farthest(&mut self) -> u64 {
        let start_point = self.find_s();
        self.memo[start_point.0][start_point.1] = 0;
        self.q.push_back(start_point);
        while let Some(origin @ (row, col)) = self.q.pop_front() {
            match self.tiles[row][col] {
                'S' => {
                    self.go_east(origin);
                    self.go_north(origin);
                    self.go_south(origin);
                    self.go_west(origin);
                }
                'J' => {
                    self.go_north(origin);
                    self.go_west(origin);
                }
                'F' => {
                    self.go_east(origin);
                    self.go_south(origin);
                }
                '|' => {
                    self.go_north(origin);
                    self.go_south(origin);
                }
                '-' => {
                    self.go_west(origin);
                    self.go_east(origin);
                }
                'L' => {
                    self.go_north(origin);
                    self.go_east(origin);
                }
                '7' => {
                    self.go_west(origin);
                    self.go_south(origin);
                }
                _ => {
                    panic!("this symbol is undefined")
                }
            }
        }

        self.memo
            .iter()
            .map(|v| v.iter().max().unwrap().clone())
            .max()
            .unwrap()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pipe_map() {
        let s = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut pm = PipeMap::new(s);
        println!("{:?}", pm);
        assert_eq!(pm.find_s(), (2, 0));
        println!("{:?}", pm.search_farthest())
    }
}

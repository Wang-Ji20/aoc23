//! https://adventofcode.com/2023/day/10
//!

use std::collections::VecDeque;

fn main() {
    let s = std::fs::read_to_string("day10.in").unwrap();
    let mut pm = PipeMap::new(&s);
    println!("{}", pm.search_farthest());
    println!("{}", pm.traverse_main_loop().abs());
}

#[derive(Debug)]
struct PipeMap {
    tiles: Vec<Vec<char>>,
    memo: Vec<Vec<i32>>,
    q: VecDeque<(usize, usize)>,
    farthest: u64,
}

impl PipeMap {
    pub fn new(s: &str) -> PipeMap {
        let tiles: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let r_len = tiles.len();
        let c_len = tiles[0].len();
        PipeMap {
            tiles,
            memo: Self::create_clean_memo(r_len, c_len),
            q: VecDeque::new(),
            farthest: 0,
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

    pub fn create_clean_memo(r_len: usize, c_len: usize) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        v.resize(c_len, -1);
        let mut v_mat = Vec::new();
        v_mat.resize(r_len, v);
        v_mat
    }

    fn is_pipe(c: char) -> bool {
        c != '.'
    }

    fn tile_within_map(&self, row: i32, col: i32) -> bool {
        row >= 0
            && col >= 0
            && row < self.tiles.len().try_into().unwrap()
            && col < self.tiles[0].len().try_into().unwrap()
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
    ) -> bool {
        let cannot_be = match (row_offset, col_offset) {
            (0, 1) => "|FL",
            (0, -1) => "|J7",
            (-1, 0) => "-LJ",
            (1, 0) => "-7F",
            _ => panic!("bad input"),
        };
        let (new_row, new_col) = (origin.0 as i32 + row_offset, origin.1 as i32 + col_offset);
        if !self.tile_within_map(new_row, new_col) {
            return true;
        }
        let (new_row_u, new_col_u) = (new_row as usize, new_col as usize);
        cannot_be.contains(self.tiles[new_row_u][new_col_u])
    }

    fn go_somewhere(
        &mut self,
        origin: (usize, usize),
        row_offset: i32,
        col_offset: i32,
    ) -> Option<(usize, usize)> {
        if !self.tile_unreachable(origin, row_offset, col_offset) {
            self.go_to_next(row_offset, col_offset, origin)
        } else {
            None
        }
    }

    fn go_east(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 0, 1)
    }

    fn go_west(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 0, -1)
    }

    fn go_north(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, -1, 0)
    }

    fn go_south(&mut self, origin: (usize, usize)) -> Option<(usize, usize)> {
        self.go_somewhere(origin, 1, 0)
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

        self.farthest = self
            .memo
            .iter()
            .map(|v| v.iter().max().unwrap().clone())
            .max()
            .unwrap()
            .try_into()
            .unwrap();
        self.farthest
    }

    pub fn add_ui(u: usize, i: i32) -> usize {
        (u as i32 + i) as usize
    }

    // part II
    pub fn without_pipe_area(&self, position: (usize, usize)) -> i64 {
        let mut i = 0;
        for (idx, v) in self.memo[position.0].iter().enumerate() {
            if idx == position.1 {
                return i;
            }
            if *v == -1 {
                i += 1;
            }
        }
        i
    }

    // use green's formula
    pub fn traverse_main_loop(&mut self) -> i64 {
        let mut position = self.find_s();
        let mut area = 0_i64;
        loop {
            let pos_val = self.memo[position.0][position.1];
            if pos_val == self.farthest as i32 {
                break;
            }
            for (row_offset, col_offset) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if !self.tile_within_map(
                    row_offset + position.0 as i32,
                    col_offset + position.1 as i32,
                ) {
                    continue;
                }
                if self.tile_unreachable(position, row_offset, col_offset) {
                    continue;
                }
                if self.memo[Self::add_ui(position.0, row_offset)]
                    [Self::add_ui(position.1, col_offset)]
                    == pos_val + 1
                {
                    self.memo[position.0][position.1] = -2;
                    let old_position = position;
                    position = (
                        Self::add_ui(position.0, row_offset),
                        Self::add_ui(position.1, col_offset),
                    );
                    match row_offset {
                        1 => area -= self.without_pipe_area(old_position),
                        -1 => area += self.without_pipe_area(position),
                        _ => continue,
                    }
                }
            }
        }
        let (sr, sc) = self.find_s();
        self.memo[sr][sc] = 0;
        loop {
            let pos_val = self.memo[position.0][position.1];
            if pos_val == 0 {
                break;
            }
            for (row_offset, col_offset) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if !self.tile_within_map(
                    row_offset + position.0 as i32,
                    col_offset + position.1 as i32,
                ) {
                    continue;
                }
                if self.tile_unreachable(position, row_offset, col_offset) {
                    continue;
                }
                if self.memo[Self::add_ui(position.0, row_offset)]
                    [Self::add_ui(position.1, col_offset)]
                    == pos_val - 1
                    || (self.memo[Self::add_ui(position.0, row_offset)]
                        [Self::add_ui(position.1, col_offset)]
                        == self.farthest as i32)
                {
                    self.memo[position.0][position.1] = -2;
                    let old_position = position;
                    position = (
                        Self::add_ui(position.0, row_offset),
                        Self::add_ui(position.1, col_offset),
                    );
                    match row_offset {
                        1 => area -= self.without_pipe_area(old_position),
                        -1 => area += self.without_pipe_area(position),
                        _ => continue,
                    }
                }
            }
        }
        area
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
        println!("{:?}", pm.search_farthest());
        println!("{:?}", pm.traverse_main_loop())
    }

    #[test]
    fn test_area() {
        let s = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let mut pm = PipeMap::new(s);
        pm.search_farthest();
        println!("{:?}", pm);
        println!("{:?}", pm.traverse_main_loop().abs())
    }
}

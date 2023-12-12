use itertools::Itertools;

fn main() {
    let s = std::fs::read_to_string("day11.in").unwrap();
    let cosmo = Cosmo::new(&s);
    println!("{}", cosmo.find_all_distances());
}

type Weight = u64;
type Galaxy = (usize, usize);

fn range_between<F>(g1: Galaxy, g2: Galaxy, get_key: F) -> std::ops::Range<usize>
where
    F: Fn(Galaxy) -> usize,
{
    let k1 = get_key(g1);
    let k2 = get_key(g2);
    match k1 > k2 {
        true => k2..k1,
        false => k1..k2,
    }
}

fn range_between_row(g1: Galaxy, g2: Galaxy) -> std::ops::Range<usize> {
    range_between(g1, g2, |x| x.0)
}

fn range_between_col(g1: Galaxy, g2: Galaxy) -> std::ops::Range<usize> {
    range_between(g1, g2, |x| x.1)
}

struct Cosmo {
    map: Vec<Vec<char>>,
    /// row... col...
    weight: Vec<Weight>,
    row_count: usize,
    col_count: usize,
}

impl Cosmo {
    pub fn col_not_contain_galaxy(map: &Vec<Vec<char>>, col_idx: usize) -> bool {
        for v in map {
            if v[col_idx] == '#' {
                return false;
            }
        }
        true
    }

    pub fn new(s: &str) -> Cosmo {
        let map: Vec<Vec<char>> = s.lines().map(|x| x.chars().collect()).collect();
        let row_count = map.len();
        let col_count = map[0].len();
        let mut weight = [1].repeat(row_count + col_count);
        for (row_idx, row_vec) in map.iter().enumerate() {
            if row_vec.contains(&'#') {
                continue;
            }
            weight[row_idx] = 1000000;
        }
        for col_idx in 0..col_count {
            if Self::col_not_contain_galaxy(&map, col_idx) {
                weight[row_count + col_idx] = 1000000;
            }
        }
        Self {
            map,
            weight,
            row_count,
            col_count,
        }
    }

    pub fn find_galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies = Vec::new();
        for (row_idx, row_vec) in self.map.iter().enumerate() {
            for (col_idx, c) in row_vec.iter().enumerate() {
                match *c {
                    '#' => galaxies.push((row_idx, col_idx)),
                    _ => continue,
                }
            }
        }
        galaxies
    }

    pub fn find_pairs(&self) -> Vec<(Galaxy, Galaxy)> {
        let gs = self.find_galaxies();
        gs.into_iter().tuple_combinations().collect()
    }

    pub fn find_distance(&self, start: Galaxy, end: Galaxy) -> usize {
        let horizon = range_between_col(start, end).fold(0_usize, |acc, x| {
            acc + self.weight[x + self.row_count] as usize
        });
        let vertical =
            range_between_row(start, end).fold(0_usize, |acc, x| acc + self.weight[x] as usize);
        vertical + horizon
    }

    pub fn find_all_distances(&self) -> usize {
        self.find_pairs()
            .iter()
            .fold(0_usize, |acc, x| acc + self.find_distance(x.0, x.1))
    }
}

#[test]
fn test_find_galaxies() {
    let s = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let cosmo = Cosmo::new(s);
    assert_eq!(374, cosmo.find_all_distances())
}

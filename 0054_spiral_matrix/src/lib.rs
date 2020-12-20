use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Direction { R, L, U, D }
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Direction::R => write!(f, "R"),
           Direction::L => write!(f, "L"),
           Direction::U => write!(f, "U"),
           Direction::D => write!(f, "D"),
       }
    }
}

#[derive(Debug)]
struct Spiral {
    current: usize,
    rows: usize,
    cols: usize,
    total: usize,
    direction: Direction,
    offset: HashMap<Direction, usize>
}
impl Spiral {
    fn new(r: usize, c: usize) -> Spiral {
        Spiral { current: 0, rows: r, cols: c, total: r*c, direction: Direction::R, offset: HashMap::new() }
    }

    fn get_index(&self) -> (usize, usize) {
        (usize::from(self.current / self.cols), usize::from(self.current % self.cols))
    }

    pub fn get_next(&mut self) -> (usize, usize) {
        match self.direction {
            Direction::R => {
                let o = self.cols- 1 - *self.offset.entry(Direction::U).or_insert(0);
                if self.get_index().1 == o {
                    self.current += self.cols;
                    self.direction = Direction::D;
                    let o = self.offset.entry(Direction::U).or_insert(0);
                    *o += 1;
                } else {
                    self.current += 1;
                }
            },
            Direction::L => {
                let o = *self.offset.entry(Direction::L).or_insert(0);
                if self.get_index().1 == o {
                    self.current -= self.cols;
                    self.direction = Direction::U;
                    let o = self.offset.entry(Direction::D).or_insert(0);
                    *o += 1;
                } else {
                    self.current -= 1;
                }
            },
            Direction::D => {
                let o = self.rows - 1 - *self.offset.entry(Direction::D).or_insert(0);
                if self.get_index().0 == o {
                    self.current -= 1;
                    self.direction = Direction::L;
                    let o = self.offset.entry(Direction::R).or_insert(0);
                    *o += 1;
                } else {
                    self.current += self.cols;
                }
            }
            Direction::U => {
                let o = *self.offset.entry(Direction::U).or_insert(0);
                if self.get_index().0 == o {
                    self.current += 1;
                    self.direction = Direction::R;
                    let o = self.offset.entry(Direction::L).or_insert(0);
                    *o += 1;
                } else {
                    self.current -= self.cols;
                }
            }
        }
        self.get_index()
    }
}

pub struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut spiral = Spiral::new(matrix.len(), matrix[0].len());
        let mut solution: Vec<i32> = vec![matrix[0][0]];
        println!("{:?}", spiral);
        for _ in 0..spiral.total-1 {
            let index = spiral.get_index();
            println!("current={:0>2} index=({},{}) direction={}", spiral.current, index.0, index.1, spiral.direction);
            let next = spiral.get_next();
            solution.push(matrix[next.0][next.1]);
        }
        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_1() {
        let mut s = Spiral::new(3, 3);
        assert_eq!(s.get_index(), (0, 0));
        s.current = 2;
        assert_eq!(s.get_index(), (0, 2));
        s.current = 3;
        assert_eq!(s.get_index(), (1, 0));
        s.current = 7;
        assert_eq!(s.get_index(), (2, 1));
    }

    #[test]
    fn get_index_2() {
        let mut s = Spiral::new(4, 4);
        assert_eq!(s.get_index(), (0, 0));
        s.current = 2;
        assert_eq!(s.get_index(), (0, 2));
        s.current = 3;
        assert_eq!(s.get_index(), (0, 3));
        s.current = 7;
        assert_eq!(s.get_index(), (1, 3));
    }

    #[test]
    fn example_1() {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::spiral_order(input), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn example_2() {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(Solution::spiral_order(input), vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
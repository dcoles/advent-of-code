use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;
use crate::vector::Vector;

pub type Pos = Vector<i64, 2>;

#[derive(Debug, Clone)]
pub struct Grid {
    data: BTreeMap<Pos, char>,
    end: Pos,
}

impl Grid {
    pub fn range(&self) -> (Range<i64>, Range<i64>) {
        let width = self.end[0] + 1;
        let height = self.end[1] + 1;

        (0..width, 0..height)
    }

    pub fn positions(&self) -> Positions {
        Positions {
            cur: Pos::new([0, 0]),
            end: self.end,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.end[1] {
            for x in 0..=self.end[0] {
                let pos = Pos::new([x, y]);
                write!(f, "{}", *self.data.get(&pos).unwrap_or(&' '))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl FromStr for Grid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = BTreeMap::new();

        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let pos = Pos::new([x as i64, y as i64]);

                data.insert(pos, c);
                max_x = max_x.max(x as i64);
            }
            max_y = max_y.max(y as i64);
        }

        let end = Pos::new([max_x, max_y]);

        Ok(Grid { data, end })
    }
}

impl Index<Pos> for Grid {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        self.data.get(&index).unwrap()
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.data.get_mut(&index).unwrap()
    }
}

pub struct Positions {
    cur: Pos,
    end: Pos,
}

impl Iterator for Positions {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur[0] < self.end[0] {
            self.cur[0] += 1;
            return Some(self.cur);
        }

        self.cur[0] = 0;

        if self.cur[1] < self.end[1] {
            self.cur[1] += 1;
            return Some(self.cur);
        }

        None
    }
}

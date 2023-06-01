use crate::random::random_range;
use std::collections::HashSet;
use std::fmt::{Display, Write};

pub type Position = (usize, usize);
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("⬛ ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);
                    if mine_count != 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }
    pub fn restart(&mut self, width: usize, height: usize, mine_count: usize) {
        // self = new(width, height, mine_count);
        self.width = width;
        self.height = height;
        self.open_fields = HashSet::new();
        self.mines = {
            let mut mines = HashSet::new();

            while mines.len() < mine_count {
                mines.insert((random_range(0, width), random_range(0, height)));
            }
            mines
        };
        self.flagged_fields = HashSet::new();
        self.lost = false;
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1..=(x + 1).min(width))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.lost || self.flagged_fields.contains(&pos) {
            return None;
        }
        if self.open_fields.contains(&pos) {
            let mine_count = self.neighboring_mines(pos);
            let flag_count = self
                .iter_neighbors(pos)
                .filter(|neighbor| self.flagged_fields.contains(neighbor))
                .count() as u8;
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(pos) {
                    if !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }
            return None;
        }

        self.open_fields.insert(pos);
        let is_mine = self.mines.contains(&pos);
        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(pos);
            if mine_count == 0 {
                for neighbor in self.iter_neighbors(pos) {
                    self.open(neighbor);
                }
            }
            Some(OpenResult::NoMine(0))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
    pub fn is_lost(&self) -> bool {
        self.lost
    }
}
#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;
    #[test]
    fn it_works() {
        let mut ms = Minesweeper::new(10, 10, 20);
        ms.open((5, 5));
        ms.toggle_flag((5, 6));
        ms.open((5, 6));
        println!("{}", ms);
        ms.restart(15, 15, 30);
        println!("{}", ms);
    }
}

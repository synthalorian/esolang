use std::io::{self, Read, Write};

use crate::parser::Grid;

pub struct VM {
    grid: Grid,
    stack: Vec<i64>,
    x: usize,
    y: usize,
    dir: u8,
    halted: bool,
}

impl VM {
    pub fn new(grid: Grid) -> Self {
        let (x, y) = grid.start;
        VM {
            grid,
            stack: Vec::new(),
            x,
            y,
            dir: 0,
            halted: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn step(&mut self) {
        let cell = self.grid.cells[self.y][self.x];
        self.execute(cell);
        if self.halted {
            return;
        }
        self.move_knight();
    }

    fn execute(&mut self, cell: char) {
        match cell {
            'H' => self.halted = true,
            '+' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b + a);
            }
            '-' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b - a);
            }
            '*' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b * a);
            }
            '/' => {
                let a = self.pop();
                let b = self.pop();
                if a != 0 {
                    self.push(b / a);
                } else {
                    self.push(0);
                }
            }
            '%' => {
                let a = self.pop();
                let b = self.pop();
                if a != 0 {
                    self.push(b % a);
                } else {
                    self.push(0);
                }
            }
            'p' => {
                let v = self.pop();
                print!("{}", v);
                let _ = io::stdout().flush();
            }
            'P' => {
                let v = self.pop();
                if let Some(c) = char::from_u32(v.max(0) as u32) {
                    print!("{}", c);
                } else {
                    print!("?");
                }
                let _ = io::stdout().flush();
            }
            'r' => {
                let mut buf = [0u8; 1];
                if io::stdin().read_exact(&mut buf).is_ok() {
                    self.push(buf[0] as i64);
                } else {
                    self.push(0);
                }
            }
            '0'..='9' => {
                self.push(cell.to_digit(10).unwrap() as i64);
            }
            'd' => {
                let v = self.pop();
                self.push(v);
                self.push(v);
            }
            's' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a);
                self.push(b);
            }
            'z' => {
                let v = self.pop();
                if v == 0 {
                    self.dir = (self.dir + 1) % 4;
                } else {
                    self.dir = (self.dir + 3) % 4;
                }
            }
            '.' => {}
            _ => {}
        }
    }

    fn move_knight(&mut self) {
        let (dx, dy) = match self.dir {
            0 => (2, 1),
            1 => (1, 2),
            2 => (-2, 1),
            3 => (1, -2),
            _ => (0, 0),
        };

        let w = self.grid.width as i64;
        let h = self.grid.height as i64;

        let nx = ((self.x as i64 + dx) % w + w) % w;
        let ny = ((self.y as i64 + dy) % h + h) % h;

        self.x = nx as usize;
        self.y = ny as usize;
    }

    fn push(&mut self, v: i64) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or(0)
    }
}

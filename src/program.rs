use crate::instructions::*;

use rand::seq::SliceRandom;

pub struct Program {
    pub prog: Vec<Vec<char>>,
    pub height: i32,
    pub width: i32,
    pub coords: (i32, i32),
    pub direction: fn(i32, i32) -> (i32, i32),
    pub stack: Vec<i64>,
    pub active: bool,
    pub jump: bool,
    pub strmode: bool,
}

impl Program {
    pub fn run(&mut self) {
        let mut y = self.coords.1.rem_euclid(self.height);
        let mut x = self.coords.0.rem_euclid(self.width);

        if y < 0 {
            y += self.height;
        }
        if x < 0 {
            x += self.width;
        }

        let mut instruct = self.prog[y as usize][x as usize];

        if self.strmode {
            if instruct == '"' {
                self.strmode = false;
                return;
            }
            self.stack.push(instruct as i64);
            return;
        }

        match instruct {
            'p' => {
                y = self.stack.pop().unwrap_or(0) as i32;
                x = self.stack.pop().unwrap_or(0) as i32;
                let v = self.stack.pop().unwrap_or(0);
                self.prog[y as usize][x as usize] = std::char::from_u32(v as u32).unwrap();
            }

            'g' => {
                y = self.stack.pop().unwrap_or(0) as i32;
                x = self.stack.pop().unwrap_or(0) as i32;

                if x > (self.width as i32) || x < 0 || y > (self.height as i32) || y < 0 {
                    self.stack.push(0);
                } else {
                    self.stack.push(self.prog[y as usize][x as usize] as i64);
                }
            }
            '@' => {
                self.active = false;
                return;
            }
            '#' => {
                self.jump = true;
                return;
            }

            '|' => {
                let val = self.stack.pop().unwrap() == 0;
                if val {
                    instruct = 'v';
                } else {
                    instruct = '^';
                }
            }
            '_' => {
                let val = self.stack.pop().unwrap() == 0;
                if val {
                    instruct = '>';
                } else {
                    instruct = '<';
                }
            }
            '?' => {
                instruct = *vec!['<', '>', 'v', '^']
                    .choose(&mut rand::thread_rng())
                    .unwrap();
            }

            '"' => {
                self.strmode = true;
            }
            '0'..='9' => {
                self.stack
                    .push(instruct.to_digit(10 as u32).unwrap() as i64);
                return;
            }
            _ => (),
        }
        if "<>v^".contains(instruct) {
            self.direction = direction(&instruct).unwrap();
            return;
        }
        if "+-/*%!`:\\$.,&~".contains(instruct) {
            instructs(&instruct, &mut self.stack);
            return;
        }
    }

    pub fn forward(&mut self) {
        self.coords = (self.direction)(self.coords.0, self.coords.1);
        if self.jump {
            self.jump = false;
            self.coords = (self.direction)(self.coords.0, self.coords.1);
        }
    }
}

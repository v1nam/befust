use std::env;
use std::fs;
use std::io;

use rand::seq::SliceRandom;

struct Program {
    prog: Vec<Vec<char>>,
    height: i32,
    width: i32,
    coords: (i32, i32),
    direction: fn(i32, i32) -> (i32, i32),
    stack: Vec<i64>,
    active: bool,
    jump: bool,
    strmode: bool,
}

impl Program {
    fn run(&mut self) {
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
        if instruct == 'p' {
            y = self.stack.pop().unwrap_or(0) as i32;
            x = self.stack.pop().unwrap_or(0) as i32;
            let v = self.stack.pop().unwrap_or(0);
            self.prog[y as usize][x as usize] = std::char::from_u32(v as u32).unwrap();
        }

        if instruct == 'g' {
            y = self.stack.pop().unwrap_or(0) as i32;
            x = self.stack.pop().unwrap_or(0) as i32;

            if x > (self.width as i32) || x < 0 || y > (self.height as i32) || y < 0 {
                self.stack.push(0);
            } else {
                self.stack.push(self.prog[y as usize][x as usize] as i64);
            }
        }
        if instruct == '@' {
            self.active = false;
            return;
        }
        if instruct == '#' {
            self.jump = true;
            return;
        }

        if instruct == '|' {
            let val = self.stack.pop().unwrap() == 0;
            if val {
                instruct = 'v';
            } else {
                instruct = '^';
            }
        }
        if instruct == '_' {
            let val = self.stack.pop().unwrap() == 0;
            if val {
                instruct = '>';
            } else {
                instruct = '<';
            }
        }
        if instruct == '?' {
            instruct = *vec!['<', '>', 'v', '^']
                .choose(&mut rand::thread_rng())
                .unwrap();
        }

        if instruct == '"' {
            self.strmode = true;
        }
        if "0123456789".contains(instruct) {
            self.stack
                .push(instruct.to_digit(10 as u32).unwrap() as i64);
            return;
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

    fn forward(&mut self) {
        self.coords = (self.direction)(self.coords.0, self.coords.1);
        if self.jump {
            self.jump = false;
            self.coords = (self.direction)(self.coords.0, self.coords.1);
        }
    }
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Please provide the file name");
        return;
    }

    args.next();
    let file = args.next().unwrap();
    let s = fs::read_to_string(file).unwrap();
    let mut prog: Vec<Vec<char>> = s
        .lines()
        .filter(|x| x != &"\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut grid_width: i32 = -1;
    for line in prog.iter() {
        if line.len() as i32 > grid_width {
            grid_width = line.len() as i32;
        }
    }
    for line in prog.iter_mut() {
        line.extend_from_slice(&[' '].repeat((grid_width - line.len() as i32) as usize));
    }
    let height = prog.len() as i32;

    let mut system = Program {
        prog,
        height,
        width: grid_width,
        coords: (0, 0),
        direction: direction(&'>').unwrap(),
        stack: Vec::new(),
        active: true,
        jump: false,
        strmode: false,
    };

    while system.active {
        system.run();
        system.forward();
    }
}

fn direction(dir: &char) -> Option<fn(i32, i32) -> (i32, i32)> {
    match dir {
        '>' => Some(|x, y| (x + 1, y)),
        '<' => Some(|x, y| (x - 1, y)),
        'v' => Some(|x, y| (x, y + 1)),
        '^' => Some(|x, y| (x, y - 1)),
        _ => None,
    }
}

fn instructs(inst: &char, sys: &mut Vec<i64>) {
    match inst {
        '+' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(first + second);
        }
        '-' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(second - first);
        }
        '*' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(first * second);
        }
        '/' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(second / first);
        }
        '%' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(second % first);
        }
        '!' => {
            let first = sys.pop().unwrap_or(0);
            sys.push((first == 0) as i64);
        }
        '`' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push((second > first) as i64);
        }
        ':' => {
            let first = sys.pop().unwrap_or(0);
            sys.push(first);
            sys.push(first.clone());
        }
        '\\' => {
            let first = sys.pop().unwrap_or(0);
            let second = sys.pop().unwrap_or(0);
            sys.push(first);
            sys.push(second);
        }
        '$' => match sys.pop() {
            Some(_x) => (),
            None => sys.push(0),
        },
        '.' => {
            print!("{}", sys.pop().unwrap_or(0));
        }
        ',' => {
            print!(
                "{}",
                std::char::from_u32(sys.pop().unwrap_or(0) as u32).unwrap_or('o')
            );
        }
        '&' => {
            let mut num = String::new();
            io::stdin().read_line(&mut num).expect("Invalid input");
            let num: i64 = num.trim().parse().unwrap_or(0);
            sys.push(num);
        }
        '~' => {
            let mut string = String::new();
            io::stdin().read_line(&mut string).expect("Invalid input");
            sys.push(string.trim().chars().next().unwrap() as i64);
        }
        _ => (),
    }
}

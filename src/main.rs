use std::env;
use std::fs;

mod instructions;
mod program;

use instructions::direction;
use program::Program;

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

    println!("\n\x1b[32m-------Program Completed-------\x1b[0m");
}

use std::io;

pub fn direction(dir: &char) -> Option<fn(i32, i32) -> (i32, i32)> {
    match dir {
        '>' => Some(|x, y| (x + 1, y)),
        '<' => Some(|x, y| (x - 1, y)),
        'v' => Some(|x, y| (x, y + 1)),
        '^' => Some(|x, y| (x, y - 1)),
        _ => None,
    }
}

pub fn instructs(inst: &char, sys: &mut Vec<i64>) {
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
            sys.push(first);
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

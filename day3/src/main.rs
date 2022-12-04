use std::fs;
use std::str::Split;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();
    part_1();
    let end = SystemTime::now();
    let elapsed  = end.duration_since(start);


    println!("{}", elapsed.unwrap_or_default().as_micros());
    //part_2();
}

fn part_1() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines: Split<&str> = file_contents.split("\n");
    let mut sum: u128 = 0;

    for line in lines {
        if line.len() > 0 {
            // last line will be empty

            let mut container_1: Vec<char> = Vec::new();
            let mut in_container_2 = false;
            let mut duplicate: char = ' ';

            let container_2_begin_idx: usize = line.len() / 2; // all lines have an even len()
            //print!("{} => ", &container_2_begin_idx);

            for (i, c) in line.chars().enumerate() {
                if in_container_2 == false {
                    container_1.push(c);
                    //print!("{}", &container_1[container_1.len() - 1]);
                } else {
                    if container_1.contains(&c) {
                        duplicate = c;
                        break;
                    }
                }

                if (i + 1) == container_2_begin_idx {
                    in_container_2 = true;
                }
            }

            //print!(" => {}", &duplicate);
            let curr = calculate_priority(duplicate) as u128;
            //print!(" => {}", &curr);

            sum += curr;

            //println!();
        }
    }

    //println!("Sum => {}", &sum);
}

fn part_2() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines: Split<&str> = file_contents.split("\n");
    let mut sum: u128 = 0;

    let mut badge_item = ' ';
    let mut next_group = false;
    let mut count = 0;

    let mut b_1: Vec<char> = Vec::new();
    let mut b_2: Vec<char> = Vec::new();

    for line in lines {
        // last line will be empty
        if next_group {
            //println!("Badge Item => {}", badge_item);
            sum += calculate_priority(badge_item) as u128;

            count = 0;
            b_1.clear();
            b_2.clear();

            next_group = false;
        }

        if !next_group {
            for c in line.chars() {
                if count == 0 {
                    b_1.push(c);
                } else if count == 1 {
                    b_2.push(c);
                } else if count == 2 {
                    if b_1.contains(&c) && b_2.contains(&c) {
                        badge_item = c;
                        break;
                    }
                }
            }
        }

        count += 1;
        if count == 3 {
            next_group = true;
        }
    }

    //println!("Sum => {}", &sum);
}

fn calculate_priority(c: char) -> u32 {
    if c.is_lowercase() {
        match c {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            _ => 0,
        }
    } else {
        match c {
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => 0,
        }
    }
}

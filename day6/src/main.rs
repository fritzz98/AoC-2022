use std::collections::HashSet;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let capacity = 14;

    let mut checker: HashSet<char> = HashSet::with_capacity(capacity);

    for line in lines {
        if line.len() > 0 {
            let chars: Vec<char> = line.chars().collect();

            for i in 0..chars.len() {
                for j in i..(i + capacity) {
                    checker.insert(chars[j]);
                }

                if checker.len() == capacity {
                    println!("{}", i + capacity);
                    break;
                } else {
                    checker.clear();
                }
            }
        }
    }
}

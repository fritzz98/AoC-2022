use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();

    let mut abs_path: Vec<String> = Vec::new();
    let mut dir_size: HashMap<String, u128> = HashMap::new();

    for line in lines {
        if line.len() > 0 {
            let content: Vec<&str> = line.split(" ").collect();

            if content[0] == "$" {
                if content[1] == "cd" {
                    if content[2] != ".." {
                        if !abs_path.is_empty() {
                            abs_path.push(String::from(content[2]));
                            dir_size.entry(gen_key(&abs_path)).or_insert(0);
                        } else {
                            abs_path.push(String::from(content[2]));
                            dir_size.insert(String::from("/"), 0);
                        }
                    } else {
                        abs_path.pop();
                    }
                }
            } else {
                if content[0] != "dir" {
                    let size: u128 = content[0].parse().unwrap();
                    add_cascade(&mut dir_size, &abs_path, size);
                }
            }
        }
    }

    let mut sum = 0u128;

    for (_key, value) in &dir_size {
        if value <= &100000 {
            sum += value;
        } //29.610.082
    }

    let start = SystemTime::now();
    part_1(&dir_size);
    let end = SystemTime::now();
    let elapsed  = end.duration_since(start);

    println!("P1 elapsed time {} microseconds", elapsed.unwrap().as_micros());


    let start = SystemTime::now();
    part_2(dir_size);
    let end = SystemTime::now();
    let elapsed  = end.duration_since(start);

    println!("P2 elapsed time {} microseconds", elapsed.unwrap().as_micros());
}

fn part_1(dir_size: &HashMap<String, u128>) {
    let mut sum = 0u128;

    for (_key, value) in dir_size {
        if value <= &100000 {
            sum += value;
        }
    }

    println!("{sum}");
    //println!("{:#?}", dir_size);
}

fn part_2(dir_size: HashMap<String, u128>) {
    let min_disk_space = 389918u128;
    let mut curr_min = &70000000u128;

    for (_key, value) in &dir_size {
        if value >= &min_disk_space {
            curr_min = if curr_min < value { curr_min } else { value }
        }
    }

    println!("{curr_min}");
}

fn add_cascade(dir_size: &mut HashMap<String, u128>, abs_path: &Vec<String>, size: u128) {
    for i in 1..(abs_path.len() + 1) {
        let curr_path = gen_key_arr(&abs_path[0..i]);
        let old_val = dir_size.entry(curr_path).or_default();
        *old_val += size;
    }
}

fn gen_key(path_vec: &Vec<String>) -> String {
    let mut key = String::new();

    for (i, folder) in path_vec.iter().enumerate() {
        key.push_str(folder);
        if i != 0 {
            key.push('/');
        }
    }

    return key;
}

fn gen_key_arr(path_arr: &[String]) -> String {
    let mut key = String::new();

    for (i, folder) in path_arr.iter().enumerate() {
        key.push_str(folder);
        if i != 0 {
            key.push('/');
        }
    }

    return key;
}

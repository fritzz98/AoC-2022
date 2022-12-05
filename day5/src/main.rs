use std::fs;

fn main() {
    // part_1();
    part_2();
}

fn part_2(){
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut crates: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ];

    for (i, line) in lines.enumerate() {
        if line.len() > 0 {
            let mut prev_index: usize = 0;

            let mut str_next_idx = String::new();
            let mut clear_val = false;

            let mut cp_idx = 0;
            let mut command_params: [usize; 3] = [0; 3];

            for (j, c) in line.chars().enumerate() {
                if i < 10 {
                    if c.is_alphabetic() && c.is_uppercase() {
                        let _ = &crates[map_column_to_index(j)].insert(0, c); // inserts at front of stack
                    }
                } else {
                    if c.is_numeric() {
                        if (j - 1) == prev_index {
                            str_next_idx.push(c);
                        } else {
                            prev_index = j;

                            if clear_val {
                                print!("{} ", str_next_idx);
                                command_params[cp_idx] = str_next_idx.parse().unwrap();
                                cp_idx += 1;
                                str_next_idx.clear();
                            } else {
                                clear_val = true;
                            }

                            str_next_idx.push(c);
                        }
                    }
                }
            }

            if str_next_idx.len() > 0 {
                command_params[cp_idx] = str_next_idx.parse().unwrap();
                println!("{}, {:?}", str_next_idx, command_params);

                exec_command_p2(&mut crates, command_params[0], command_params[1]-1, command_params[2]-1);

            }
        }
    }


    for c in &crates {
        println!("{:?}", &c);
    }
}

fn part_1(){
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut crates: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ];

    for (i, line) in lines.enumerate() {
        if line.len() > 0 {
            let mut prev_index: usize = 0;

            let mut str_next_idx = String::new();
            let mut clear_val = false;

            let mut cp_idx = 0;
            let mut command_params: [usize; 3] = [0; 3];

            for (j, c) in line.chars().enumerate() {
                if i < 10 {
                    if c.is_alphabetic() && c.is_uppercase() {
                        let _ = &crates[map_column_to_index(j)].insert(0, c); // inserts at front of stack
                    }
                } else {
                    if c.is_numeric() {
                        if (j - 1) == prev_index {
                            str_next_idx.push(c);
                        } else {
                            prev_index = j;

                            if clear_val {
                                print!("{} ", str_next_idx);
                                command_params[cp_idx] = str_next_idx.parse().unwrap();
                                cp_idx += 1;
                                str_next_idx.clear();
                            } else {
                                clear_val = true;
                            }

                            str_next_idx.push(c);
                        }
                    }
                }
            }

            if str_next_idx.len() > 0 {
                command_params[cp_idx] = str_next_idx.parse().unwrap();
                println!("{}, {:?}", str_next_idx, command_params);

                exec_command(&mut crates, command_params[0], command_params[1]-1, command_params[2]-1);

            }
        }
    }


    for c in &crates {
        println!("{:?}", &c);
    }
}

fn exec_command_p2(crates: &mut [Vec<char>; 9], amount: usize, from: usize, to: usize) {
    for i in 0..amount {
        let popped = crates[from].pop();
        let insert_at = crates[to].len() as i32 - i as i32;

        crates[to].insert(insert_at as usize, if let Some(val) = popped { val } else { 'X' });
    }
}

fn exec_command(crates: &mut [Vec<char>; 9], mut amount: usize, from: usize, to: usize) {
    loop {
        if amount == 0 {
            break;
        }

        let popped = crates[from].pop();
        crates[to].push(if let Some(val) = popped { val } else { 'X' });
        amount -= 1;
    }
}

fn map_column_to_index(i: usize) -> usize {
    return match i {
        1 => 0,
        5 => 1,
        9 => 2,
        13 => 3,
        17 => 4,
        21 => 5,
        25 => 6,
        29 => 7,
        33 => 8,
        _ => 100, // so that it generates an error
    };
}

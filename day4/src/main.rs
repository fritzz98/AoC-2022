use std::fs;

fn main() {
    //part1();
    part2();
}

fn part2(){
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut sum: u128 = 0;

    for line in lines {
        if line.len() > 0 {

            let pair_range: Vec<&str> = line.split(",").collect(); //  0 => elf1; 1 => elf2;

            let str_interval_1: Vec<&str> = pair_range[0].split("-").collect();
            let str_interval_2: Vec<&str> = pair_range[1].split("-").collect();

            let arr_1: [u128; 2] = [str_interval_1[0].parse().unwrap(), str_interval_1[1].parse().unwrap()];
            let arr_2: [u128; 2] = [str_interval_2[0].parse().unwrap(), str_interval_2[1].parse().unwrap()];


            if arr_1[0] <= arr_2[1] && arr_1[1] >= arr_2[0]{
                sum+=1;
            }

        }
    }

    println!("{}", sum);
}

fn part1(){
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut sum: u128 = 0;

    for line in lines {
        if line.len() > 0 {

            let pair_range: Vec<&str> = line.split(",").collect(); //  0 => elf1; 1 => elf2;

            let str_interval_1: Vec<&str> = pair_range[0].split("-").collect();
            let str_interval_2: Vec<&str> = pair_range[1].split("-").collect();

            let arr_1: [u128; 2] = [str_interval_1[0].parse().unwrap(), str_interval_1[1].parse().unwrap()];
            let arr_2: [u128; 2] = [str_interval_2[0].parse().unwrap(), str_interval_2[1].parse().unwrap()];


            if arr_1[0] >= arr_2[0] && arr_1[1] <= arr_2[1]{
                sum+=1;
            } else if arr_2[0] >= arr_1[0] && arr_2[1] <= arr_1[1]{
                sum+=1;
            }

        }
    }

    println!("{}", sum);
}

use std::fs;

fn main() {
    //part_1();
    part_2();
}

fn part_2() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut matrix: [[u8; 99]; 99] = [[0; 99]; 99];

    for (i, line) in lines.enumerate() {
        if line.len() > 0 {
            for (j, ch) in line.chars().enumerate() {
                let val: u8 = ch.to_digit(10).unwrap() as u8;

                matrix[i][j] = val;
            }
        }
    }

    let mut curr_max = 0usize;
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let scienic = calc_scenic_score(i, j, &matrix);
            curr_max = if scienic > curr_max {
                scienic
            } else {
                curr_max
            };
        }
    }

    println!("{curr_max}");
}

fn part_1() {
    let file_contents = fs::read_to_string("input.txt").expect("error");
    let lines = file_contents.lines();
    let mut matrix: [[u8; 99]; 99] = [[0; 99]; 99];
    let mut visible = 0u128;

    for (i, line) in lines.enumerate() {
        if line.len() > 0 {
            for (j, ch) in line.chars().enumerate() {
                let val: u8 = ch.to_digit(10).unwrap() as u8;

                matrix[i][j] = val;
            }
        }
    }

    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if i == 0 || j == 0 || i == 98 || j == 98 {
                // edge element
                visible += 1;
            } else {
                // not edge element
                calc_visibilty(i, j, &matrix, &mut visible)
            }
        }
    }

    println!("{visible}");
}

fn calc_scenic_score(i: usize, j: usize, matrix: &[[u8; 99]; 99]) -> usize {
    let val = matrix[i][j];

    let mut idx = i;
    let mut first = true;
    let vis_top = loop {
        // top

        if first {
            first = false;

            if idx == 0 {
                break 0;
            }
        }

        idx -= 1;

        if val <= matrix[idx][j] {
            break i - idx;
        }

        if idx == 0 {
            break i - idx;
        }
    };

    let mut idx = i;
    let mut first = true;
    let vis_bot = loop {
        // bot

        if first {
            first = false;

            if idx == 98 {
                break 0;
            }
        }

        idx += 1;

        if val <= matrix[idx][j] {
            break idx - i;
        }

        if idx == 98 {
            break idx - i;
        }
    };

    let mut idx = j;
    let mut first = true;
    let vis_right = loop {
        // right

        if first {
            first = false;

            if idx == 98 {
                break 0;
            }
        }

        idx += 1;

        if val <= matrix[i][idx] {
            break idx - j;
        }

        if idx == 98 {
            break idx - j;
        }
    };

    let mut idx = j;
    let mut first = true;
    let vis_left = loop {
        // left
        if first {
            first = false;

            if idx == 0 {
                break 0;
            }
        }

        idx -= 1;

        if val <= matrix[i][idx] {
            break j - idx;
        }

        if idx == 0 {
            break j - idx;
        }
    };

    vis_top * vis_bot * vis_right * vis_left
}

fn calc_visibilty(i: usize, j: usize, matrix: &[[u8; 99]; 99], visible: &mut u128) {
    // *visible += 1;

    let val = matrix[i][j];

    let mut vis_top = true;
    let mut vis_bot = true;
    let mut vis_left = true;
    let mut vis_right = true;

    let mut idx = i;
    loop {
        // top
        idx -= 1;

        if val <= matrix[idx][j] {
            vis_top = false;
            break;
        }

        if idx == 0 {
            break;
        }
    }

    if vis_top {
        *visible += 1;
        return;
    }

    let mut idx = i;
    loop {
        // bot
        idx += 1;

        if val <= matrix[idx][j] {
            vis_bot = false;
            break;
        }

        if idx == 98 {
            break;
        }
    }

    if vis_bot {
        *visible += 1;
        return;
    }

    let mut idx = j;
    loop {
        // right
        idx += 1;

        if val <= matrix[i][idx] {
            vis_right = false;
            break;
        }

        if idx == 98 {
            break;
        }
    }

    if vis_right {
        *visible += 1;
        return;
    }

    let mut idx = j;
    loop {
        // left
        idx -= 1;

        if val <= matrix[i][idx] {
            vis_left = false;
            break;
        }

        if idx == 0 {
            break;
        }
    }

    if vis_left {
        *visible += 1;
        return;
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use aoc_util::input_reader;

const LANES : usize = 5;

fn main() {
    let mut counts : [u32; LANES] = [0; LANES];
    let lines = input_reader::read_by_lines("./test.txt");
    let n_rows = compute_lane_counts(lines, &mut counts);
    let (gamma, epsilon) = compute_gamma_epsilon(&counts, n_rows);
    println!("\n{} * {} = {}\n", gamma, epsilon, gamma*epsilon);
}


fn compute_lane_counts<I>(lines: I, buff: &mut [u32]) -> u32
where 
    I: IntoIterator<Item = String>
{
    let mut count : u32 = 0; 
    for line in lines {
        compute_line_count(line, buff);
        count += 1;
    }
    count
}

fn compute_line_count(line : String, buff : &mut [u32]) {
    for (i, c) in line.chars().enumerate() {
        match c {
            '1' => buff[i] += 1,
            _ => ()
        }
    }
}

fn compute_gamma_epsilon(lane_count : &[u32], n_rows : u32) -> (i32, i32) {

    let half = n_rows/2; // bug for rounding?
    let mut gamma_chars = Vec::new();
    let mut epsilon_chars = Vec::new();
    for i in 0..LANES {
        let on_count = lane_count[i];
        if on_count > half {
            gamma_chars.push('1');
            epsilon_chars.push('0');
        }
        else {
            gamma_chars.push('0');
            epsilon_chars.push('1');
        }      
    }
    let gamma_str : String = gamma_chars.iter().collect();
    let epsilon_str : String = epsilon_chars.iter().collect();
    let gamma = i32::from_str_radix(&gamma_str, 2);
    let epsilon = i32::from_str_radix(&epsilon_str, 2);
    (gamma.unwrap(), epsilon.unwrap())
}
 
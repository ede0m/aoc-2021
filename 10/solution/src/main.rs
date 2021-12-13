use std::collections::{HashMap,HashSet};
use aoc_util::input_reader;


fn main() {

    let point_map : HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let input = input_reader::read_by_lines("input.txt");
    let mut score = 0;
    for line in input {
        if let Some(corrupting_char) = validate_line(line) {
            score += point_map.get(&corrupting_char).unwrap();
        }
    }
    println!("\nsyntax error score: {}\n", score);

}

// processes a syntax line. returns a corrupting char if corrupted, none if incomplete/valid
fn validate_line(line: String) -> Option<char> {

    let open_set : HashSet<char> = HashSet::from ([
        '(', '[', '{', '<'
    ]);

    let close_open_map : HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);
    
    let mut stack = vec![];
    for c in line.chars() {
        if open_set.contains(&c) {
            stack.push(c.clone());
        }
        else {
            let open_pairing = close_open_map.get(&c).unwrap();
            let check = stack.pop().unwrap();
            if check != *open_pairing {
                return Some(c);
            }
        }
    }
    None
}

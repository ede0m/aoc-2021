use std::collections::{HashMap};
use aoc_util::input_reader;
/*

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....



  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

*/


fn main() {

    let segment_digit_map : HashMap<usize, Vec<char>> = HashMap::from([
        (2, vec!['1']),
        (3, vec!['7']),
        (4, vec!['4']),
        (5, vec!['2', '3', '5']),
        (6, vec!['0', '6', '9']),
        (7, vec!['8'])
    ]);

    let mut easy_digit_count = 0;
    let mut part_2_output_sum = 0;
    let input_lines = input_reader::read_lines_into_vecs::<String>("input.txt", ' ');
    for line_vec in input_lines {
        let mut input = line_vec.split(|x| x.as_bytes()[0] == b'|');
        let signal_patterns : Vec<String> = input.next().unwrap().to_vec();
        let output_patterns = input.next().unwrap().to_vec();
        let signal_segment_map = decode_signals(signal_patterns, &segment_digit_map);
        println!("{:?}", signal_segment_map);
       
        // part 2
        let output_val = parse_output(output_patterns.clone(), &signal_segment_map);
        part_2_output_sum += output_val;

        // part 1
        for d in output_patterns {
            // part 1 (a unique segment count -> digit mapping)
            let d_len = d.len();
            if segment_digit_map.get(&d_len).unwrap().len() == 1 {
                easy_digit_count += 1;
            }
        }
    }
    //println!("\ntotal \"easy\" digits: {}\n", easy_digit_count);
    println!("\ntotal output vals: {}\n", part_2_output_sum);
}

fn filter_signals(n_segmets : usize, signals : Vec<String>) -> Vec<String> {
    signals.iter().filter(|s| s.chars().count() == n_segmets).map(|s| s.clone()).collect()
}

fn contains_both_d1_signal_segments(segment: &str, d1_signal: &str) -> bool {
    segment.find(d1_signal.chars().nth(0).unwrap()).is_some() 
    && segment.find(d1_signal.chars().nth(1).unwrap()).is_some()
}

fn sorted_string(s : &str) -> String {
    let mut chars : Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn parse_output(output_signals : Vec<String>, map : &HashMap<String, char>) -> u32 {
    let mut output_str = String::new();
    for s in output_signals {
        let sorted_signal = sorted_string(&s);
        let char_digit = map.get(&sorted_signal).unwrap();
        output_str.push(*char_digit);
    }
    output_str.parse().unwrap()
}

fn decode_signals(signals : Vec<String>, segment_digit_map : &HashMap<usize, Vec<char>>) -> HashMap<String, char>  {

    let segment_codes = "abcdefg";
    let mut known : HashMap<String, char> = HashMap::new();
    let mut d1_signal : String = String::new();
    // map signals to count, find the easy numbers and d1 signal
    for s in &signals {
        if segment_digit_map.get(&s.len()).unwrap().len() == 1 {
            let digit = segment_digit_map.get(&s.len()).unwrap()[0];
            if digit == '1' { d1_signal = s.clone(); }
            known.insert(sorted_string(s), digit); 
        }
    }

    // whichever (count6) digit that does not contain both 1(count2) letters is 6
    // segment C is the missing letter, position F is the one not missing.
    let mut count6_signals = filter_signals(6, signals.clone());
    let d6_signal_idx = count6_signals.iter().position(|s| {
        !contains_both_d1_signal_segments(s, &d1_signal)
    }).unwrap();

    // segment C is the missing segment from digit 6
    let d6_signal = count6_signals[d6_signal_idx].clone();
    let segment_c_code = segment_codes.chars().filter(|c| {
        !d6_signal.contains(*c)}
    ).last().unwrap();    
    known.insert(sorted_string(&d6_signal), '6');
    count6_signals.remove(d6_signal_idx);

    // whichever (count5) digit that does contain both d1(count2) letters, is 3. 
    let mut count5_signals = filter_signals(5, signals.clone());
    let d3_signal_idx = count5_signals.iter().position(|s| {
        contains_both_d1_signal_segments(s, &d1_signal)
    }).unwrap();
    let d3_signal = count5_signals[d3_signal_idx].clone(); 
    known.insert(sorted_string(&d3_signal), '3');
    count5_signals.remove(d3_signal_idx);
    
    //d5 will not have segment C code. 
    let d5_signal_idx = count5_signals.iter().position(|s| {
        !s.contains(segment_c_code)
    }).unwrap();
    let d5_signal = count5_signals[d5_signal_idx].clone();

    // segment E code will be missing letter from d5 that isn't segment c code. 
    let segment_e_code = segment_codes.chars().filter(|c| {
        *c != segment_c_code && !d5_signal.contains(*c)
    }).last().unwrap();
    known.insert(sorted_string(&d5_signal), '5');
    count5_signals.remove(d5_signal_idx);

    // d2 is only d left in count5 signals
    let d2_signal = count5_signals.last().unwrap().to_owned();
    known.insert(sorted_string(&d2_signal), '2');

    // d9 is just missing e code
    let d9_signal_idx = count6_signals.iter().position(|s| {
        !s.contains(segment_e_code)
    }).unwrap();
    let d9_signal = count6_signals[d9_signal_idx].clone();
    known.insert(sorted_string(&d9_signal), '9');
    count6_signals.remove(d9_signal_idx);

    // d0 is last
    let d0_signal = count6_signals.last().unwrap().to_owned();
    known.insert(sorted_string(&d0_signal), '0');

    known
}


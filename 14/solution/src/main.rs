use aoc_util::input_reader;
use std::collections::HashMap;

fn main() {
    
    let mut polymer_map : HashMap<String, char> = HashMap::new();

    let mut input = input_reader::read_by_lines("input.txt");
    let template = input.next().unwrap();
    input.next(); // skip empty line..
    for l in input {
        let mut data : Vec<&str> = l.split("->").collect();
        let (k, v) = (data[0].trim(), data[1].trim());
        polymer_map.insert(String::from(k), v.chars().next().unwrap());
    }

    let mut template_iter : Vec<char> = template.chars().collect();
    for i in 0..40 {
        let mut build = vec![];
        build.push(template_iter[0]);
        for w in template_iter.windows(2) {
            let s : String = w.iter().collect();
            if let Some(insert) = polymer_map.get(&s) {
                build.push(*insert);
                build.push(w[1]);
            }
            else {
                build.push(w[1]);
            }
        }
        template_iter = build;
    }
    let diff = count_chars_diff(&template_iter);
    println!("\ndiff count: {:?}", diff);
}

fn count_chars_diff(chars : &Vec<char>) -> u32 {
    let mut counter = HashMap::new();
    for c in chars {
        *counter.entry(c).or_insert(1) += 1;
    }
    let max_char_count = counter.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v);
    let min_char_count = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v);
    max_char_count.unwrap() - min_char_count.unwrap()
}

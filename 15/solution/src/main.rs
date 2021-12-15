use aoc_util::input_reader;

fn main() {
    
    let risk_map : Vec<Vec<u32>> = input_reader::read_lines_into_vecs::<u32>("test.txt", None).collect();
    let total_cost_map = build_cost_map(&risk_map);
    let cost = total_cost_map[total_cost_map.len()-1][total_cost_map[0].len()-1] - total_cost_map[0][0];
    println!("min path cost = {:?}\n", cost);
}

fn build_cost_map(risk_map : &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    
    println!("\n");
    let mut total_cost_map : Vec<Vec<u32>> = vec![vec![0; risk_map[0].len()]; risk_map.len()];
    for y in 0..risk_map.len() {
        for x in 0..risk_map[y].len() {
            total_cost_map[y][x] = min_adj_cost(&total_cost_map, &risk_map, (x, y))
        }
    }
    println!("\n");
    for l in &total_cost_map { println!("{:?}", l) };
    println!("\n");
    total_cost_map
}

fn min_adj_cost(total_cost_map : &Vec<Vec<u32>>, risk_map : &Vec<Vec<u32>>, coords : (usize, usize)) -> u32 {
    // only consider adj top and left, since we assume we can only move down and right..
    let mut adj_cost = u32::MAX;
    let this_cost = risk_map[coords.1][coords.0];
    let top_cost = if coords.1 <= 0 {None} else {Some(total_cost_map[coords.1 - 1][coords.0])};
    let left_cost = if coords.0 <= 0 {None} else {Some(total_cost_map[coords.1][coords.0 - 1])};

    //println!("thiscost: {:?}  topcost: {:?}  leftcost: {:?}", this_cost, top_cost, left_cost);
    if top_cost.is_some() && top_cost.unwrap() < adj_cost { adj_cost = top_cost.unwrap()}
    if left_cost.is_some() && left_cost.unwrap() < adj_cost { adj_cost = left_cost.unwrap()}
    if top_cost.is_none() && left_cost.is_none() {adj_cost = 0}
    let min_adj_cost = this_cost + adj_cost;
    //println!("min_adj_cost: {:?}\n", min_adj_cost);
    min_adj_cost
}

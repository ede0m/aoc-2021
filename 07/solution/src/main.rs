use aoc_util::input_reader;

fn main() {
    let input_pos = input_reader::read_line_into_vec::<i32>("input.txt", Some(','));
    let mut min_fuel = i32::MAX;
    let mut optimal_pos: i32 = 0;

    for tp in &input_pos {
        let mut fuel_consumption = 0;
        for cp in &input_pos {
            fuel_consumption += i32::abs(tp - cp);
        }
        if fuel_consumption < min_fuel {
            min_fuel = fuel_consumption;
            optimal_pos = *tp;
        }
    }

    println!(
        "\noptimal fuel consumption {} at position {}\n",
        min_fuel, optimal_pos
    );
}

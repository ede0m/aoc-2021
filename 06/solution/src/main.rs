use std::fmt;
use aoc_util::input_reader;

#[derive(Copy, Clone)]
struct LanternFish {
    days_left: u32,
}

impl fmt::Debug for LanternFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.days_left)
    }
}

impl LanternFish {
    fn new() -> LanternFish {
        LanternFish { days_left: 8 }
    }

    fn new_with_days(days: u32) -> LanternFish {
        LanternFish { days_left: days }
    }

    fn reset(&mut self) {
        self.days_left = 6;
    }

    fn can_reproduce(&self) -> bool {
        if self.days_left <= 0 {
            return true;
        }
        false
    }

    fn live_one_day(&mut self) {
        self.days_left -= 1;
    }
}


fn main() {
    const SIM_DAYS: u32 = 80;
    let initial_fish_state = input_reader::read_line_into_vec::<u32>("input.txt", Some(','));
    let mut fish_state = Vec::new();
    for f in initial_fish_state {
        fish_state.push(LanternFish::new_with_days(f));
    }

    let mut days_simmed = 1;
    while days_simmed <= SIM_DAYS {
        let mut temp_fish_holding = Vec::new();
        for fish in &mut fish_state {
            if fish.can_reproduce() {
                fish.reset();
                temp_fish_holding.push(LanternFish::new());
            } else {
                fish.live_one_day();
            }
        }
        fish_state.append(&mut temp_fish_holding);
        //println!("\n{:?}", fish_state);
        days_simmed += 1;
    }

    println!(
        "\n{} fish after {} days simulated\n",
        fish_state.len(),
        SIM_DAYS
    );
}
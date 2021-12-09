use aoc_util::input_reader;
use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u32>>, // row, col
    dimensions: (usize, usize), // x, y
}

// When derived on structs, PartialOrd compares two instances by comparing 
// the value in each field in the order in which the fields appear in the struct definition.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Spot {
    val: u32,
    x: usize,
    y: usize,
}

struct Adjacent {
    top: Option<Spot>,
    bottom: Option<Spot>,
    left: Option<Spot>,
    right: Option<Spot>
}

impl Adjacent {
    fn mins(&self) -> Vec<Spot> {
        let mut sides = vec![];
        if self.top.is_some() { sides.push(self.top.unwrap()); }
        if self.bottom.is_some() { sides.push(self.bottom.unwrap()); }
        if self.left.is_some() { sides.push(self.left.unwrap()); }
        if self.right.is_some() { sides.push(self.right.unwrap()); }
        // handle ties, so we return many
        let min_val = sides.iter().min().unwrap().val;
        sides.into_iter().filter(|s| s.val == min_val).collect()
    }
}

impl Spot {
    fn new(x: usize, y : usize, val : u32) -> Spot {
        Spot {
            x,
            y,
            val
        }
    }

    fn adjacent(&self, hm : &Map) -> Adjacent {
        let (y_bound, x_bound) = (hm.dimensions.0 - 1, hm.dimensions.1 - 1);
        //println!("y bound: {}. x bound: {}", y_bound, x_bound);
        let top = if self.y <= 0 { None } else { Some(Spot::new(self.x, self.y-1, hm.heights[self.y-1][self.x])) };
        let bottom = if self.y >= y_bound { None } else { Some(Spot::new(self.x, self.y+1, hm.heights[self.y+1][self.x])) };
        let left = if self.x <= 0 { None } else { Some(Spot::new(self.x-1, self.y, hm.heights[self.y][self.x-1])) };
        let right = if self.x >= x_bound { None } else { Some(Spot::new(self.x+1, self.y, hm.heights[self.y][self.x+1])) };
        Adjacent { top, bottom, left, right }
    }

    fn key(&self) -> String {
        let mut key = String::new();
        key.push_str(&self.x.to_string());
        key.push_str(&self.y.to_string());
        key
    }

}

impl Map {
    
    fn new(heights : Vec<Vec<u32>>) -> Map {
        let dimensions = (heights.len(), heights[0].len()); // y, x
        Map {
            heights,
            dimensions  
        }
    }

    fn search_for_low_spots(&self) -> Vec<Spot> {
        let mut low_spots : Vec<Spot> = vec![];
        let mut searched : HashSet<String> = HashSet::new(); // can't get this optimization working for some reason..    
        for (y, row) in self.heights.iter().enumerate() {
            for (x, col_val) in row.iter().enumerate() {
                let start_spot = Spot::new(x, y, *col_val);
                let low_spots_found = self.find_low_spots_from_start_spot(start_spot, &mut searched);
                let mut low_spots_found = low_spots_found.iter().filter(|s| s.is_some()).map(|s| s.unwrap()).collect();
                low_spots.append(&mut low_spots_found); 
            }
        }
        low_spots
    }    
    
    // we take a start spot and return any low spots found from searching from this start spot.
    // this can return multiple spots because we can't gaurantee that there will be a tie for min adjacent spot
    fn find_low_spots_from_start_spot(&self, start_spot : Spot, searched : &mut HashSet<String>) -> Vec<Option<Spot>> {
        
        // if we have search on this path, we will end up at the same low spot..
        // not sure why this doesn't hold..?
        //if searched.contains(&start_spot.key()) { return vec![None]; } 
        
        let this_height = start_spot.val;
        let adj = start_spot.adjacent(self);
        let min_adj_spots = adj.mins(); // ties are handled by returning all adj that have the min value. 
        searched.insert(start_spot.key());
    
        // this is the lowest surrounding spot... base case
        if min_adj_spots.first().unwrap().val > this_height {
            println!("found low spot: {:?}", start_spot);
            return vec![Some(start_spot)];
        }
        else {
            let mut continue_search_spots = vec![];
            for m in min_adj_spots {
                if m.val <= this_height { // should this be <=?
                    let mut low_spots = self.find_low_spots_from_start_spot(m, searched);
                    continue_search_spots.append(&mut low_spots);
                }
            }
            continue_search_spots
        }
    }
}

fn risk_level(spots : Vec<Spot>) -> u32 {
    spots.iter().map(|s| s.val + 1).sum()
}

fn main() {
    let input : Vec<Vec<u32>> = input_reader::read_lines_into_vecs::<u32>("input.txt", None).collect();
    let hm = Map::new(input);
    //println!("\n{:?}\n", hm);
    let mut low_spots = hm.search_for_low_spots();
    low_spots.sort();
    low_spots.dedup();
    //println!("\nlow spots: {:#?}", low_spots);
    println!("\nrisk level: {}\n", risk_level(low_spots));
}

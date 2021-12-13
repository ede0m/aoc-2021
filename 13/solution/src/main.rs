use aoc_util::input_reader;

#[derive(Debug)]
enum FoldDirection {
    None, X, Y
}

struct Point {
    x: u32,
    y: u32,
}

struct Fold {
    dir: FoldDirection,
    index: u32
}

impl Point {
    fn new(input_line : &str) -> Point {
        let x_y : Vec<&str> = input_line.split(",").collect();
        Point {
            x: x_y[0].parse().unwrap(),
            y: x_y[1].parse().unwrap()
        }   
    }
}

impl Fold {
    fn new(input_line: &str) -> Fold {
        let data = input_line.split(" ").nth(2).unwrap();
        let dir_i : Vec<&str> = data.split('=').collect();
        let (dir, i) = (dir_i[0], dir_i[1]);
        let mut fd = FoldDirection::None;
        if dir == "y" { fd = FoldDirection::Y; }        
        else if dir == "x" {fd = FoldDirection::X; } 
        Fold {
            dir: fd,
            index: i.parse().unwrap()
        }
    }
}


fn main() {
    let input_lines : Vec<String> = input_reader::read_by_lines("input.txt").collect();
    let folds : Vec<Fold> = input_lines.iter().filter(|l| l.starts_with("fold")).map(|l| Fold::new(l)).collect();
    let points : Vec<Point> = input_lines.iter().filter(|l| !l.starts_with("fold") && !l.is_empty()).map(|l| Point::new(&l)).collect();

    let mut grid = build_grid(points);
    let first_fold = &folds[0];

    let grid = fold_x(first_fold.index, grid);

    // for f in folds {
    //     grid = match f.dir {
    //         FoldDirection::X => fold_x(f.index, grid),
    //         _ =>  {
    //             println!("{:?} not supported yet..", f.dir);
    //             grid
    //         }
    //     };
    // }
    let flat = grid.iter().flatten();
    let count = flat.filter(|c| **c == '#').count();
    println!("dot count: {}", count);
}

fn build_grid(points : Vec<Point>) -> Vec<Vec<char>> {

    let x_bound = points.iter().max_by_key(|p| p.x).unwrap().x;
    let y_bound = points.iter().max_by_key(|p| p.y).unwrap().y;
    //println!("x bound: {}  y bound: {}", x_bound, y_bound);
    // init the grid
    let mut grid = vec![];
    for _ in 0..y_bound+1 {
        grid.push(vec!['.'; (x_bound+1) as usize]);
    }
    for p in points {
        grid[p.y as usize][p.x as usize] = '#';
    }
    //println!("\n{:#?}\n", grid);
    grid
}

fn fold_x(at: u32, grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut folded = vec![];
    for row in grid {
        let (l, r) = row.split_at(at as usize); 
        let (mut l, mut r) = (l.to_owned(), r.to_owned());
        r.remove(0); // r contans the "at" col, however in this problem we remove this col.  // fold at index col will be lost.
        l.reverse(); // -1 projection
        let max = std::cmp::max(l.len(), r.len()); // use the longest sub vec in case it wasn't folded in the middle..
        // check overlap onto right 
        let mut new = vec![];
        for i in 0..max {
            if i < l.len() && l[i] == '#' || i < r.len() && r[i] == '#' { new.push('#'); }
            else { new.push('.'); }
        }
        folded.push(new);
    }
    // println!("\n");
    // for l in &folded {println!("{:?}", l);}
    // println!("\n");
    folded
}

fn fold_y(at: u32, grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (t, b) = grid.split_at(at as usize);
    let (mut t, mut b) = (t.to_owned(), b.to_owned());
    b.remove(0); // fold at index row will be lost.
    b.reverse();  // fold the bottom half up
    let max = std::cmp::max(t.len(), b.len()); // use the longest sub vec in case it wasn't folded in the middle..
    for i in 0..max {
        // todo: logic for the fold y overlap..

    }
    vec![]
}

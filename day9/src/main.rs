use std::collections::HashMap;

mod data;

static TESTINPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

static TESTINPUT2: &str = "
";

static VERBOSE: bool = false;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 9");
    println!("********************");

    let mut input: &str = data::INPUT;
    for a in std::env::args() {
        if a == "--test" {
            println!("using test data");
            input = TESTINPUT;
            break;
        }
        if a == "--test2" {
            println!("using test data 2");
            input = TESTINPUT2;
            break;
        }
    }
    input = input.trim();

    part1(String::from(input));
    part2(String::from(input));
}
fn part1(input: String) {
    let start_time = std::time::Instant::now();

    let mut reds: Vec<(usize, usize)> = Vec::new();
    for l in input.lines() {
        let mut s = l.split(",");
        let x: usize = s.next().unwrap().parse().unwrap();
        let y: usize = s.next().unwrap().parse().unwrap();

        reds.push((x, y));
    }

    let mut max_area: i64 = 0;

    let c = reds.len();
    for i in 0..c {
        let a = &reds[i];
        for j in i + 1..c {
            let b = &reds[j];
            let width = (a.0 as i64 - b.0 as i64).abs() + 1;
            let height = (a.1 as i64 - b.1 as i64).abs() + 1;
            let mut area = width * height;
            if area < 0 {
                area *= -1;
            }
            if area > max_area {
                if VERBOSE {
                    println!(
                        "overriding area with ({},{}) ({},{}) | w={} h={}",
                        a.0, a.1, b.0, b.1, width, height
                    )
                }
                max_area = area;
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 1 -----------------");
    println!("Anwser is \n{}\n", max_area);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

struct MinMax {
    min: usize,
    max: usize,
}
impl std::fmt::Display for MinMax {
    // This trait requires the `fmt` method with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the `write!` macro to write the formatted output to the formatter `f`.
        write!(f, "(min {}, max {})", self.min, self.max)
    }
}

struct IdxRange {
    index: HashMap<usize, MinMax>,
}

impl IdxRange {
    fn update(&mut self, key: usize, val: usize) {
        if let None = self.index.get(&key) {
            self.index.insert(key, MinMax { min: val, max: val });
        }
        if val < self.index[&key].min {
            self.index.get_mut(&key).unwrap().min = val;
        }
        if val > self.index[&key].max {
            self.index.get_mut(&key).unwrap().max = val;
        }
    }
    fn is_inrange(&self, key: usize, val: usize) -> bool {
        if let None = self.index.get(&key) {
            return false;
        }
        return val >= self.index[&key].min && val <= self.index[&key].max;
    }
}

fn is_inbounds(red_cols: &IdxRange, red_rows: &IdxRange, pos: (usize, usize)) -> bool {
    return red_rows.is_inrange(pos.1, pos.0) && red_cols.is_inrange(pos.0, pos.1);
}

fn part2(input: String) {
    let start_time = std::time::Instant::now();

    let mut reds: Vec<(usize, usize)> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for l in input.lines() {
        let mut s = l.split(",");
        let x: usize = s.next().unwrap().parse().unwrap();
        let y: usize = s.next().unwrap().parse().unwrap();
        if x > width {
            width = x + 1;
        }
        if y > height {
            height = y + 1;
        }

        reds.push((x, y));
    }
    width += 2; // padding
    height += 1;

    // Find min max of rows
    let c = reds.len();
    let mut red_rows = IdxRange {
        index: HashMap::new(),
    };
    let mut red_cols = IdxRange {
        index: HashMap::new(),
    };

    for i in 0..c {
        let x = reds[i].0;
        let y = reds[i].1;

        red_rows.update(y, x);
        red_cols.update(x, y);

        if VERBOSE {
            println!("processing {},{}", x, y);
        }

        // peek the next to link and trace next path
        let mut next_idx = i + 1;
        if next_idx >= c {
            next_idx = 0; // connect back to begining
        }
        let next = &reds[next_idx];
        let next_x = next.0;
        let next_y = next.1;

        // green tiles going horizontaly on x
        let dx = (next_x as i64 - x as i64);
        if next_y == y && dx != 0 {
            if dx > 0 {
                // going from left to right
                for xval in x + 1..next_x {
                    red_rows.update(y, xval);
                    red_cols.update(xval, y);
                }
            } else {
                // going right to left (reverse so we start at next)
                for xval in next_x + 1..x {
                    red_rows.update(y, xval);
                    red_cols.update(xval, y);
                }
            }
            continue;
        }
        // green tiles going vertically on x
        let dy = (next_y as i64 - y as i64);
        if next_x == x && dy != 0 {
            if dy > 0 {
                // going from up from low to high
                for yval in y + 1..next_y {
                    red_cols.update(x, yval);
                    red_rows.update(yval, x);
                }
            } else {
                // going down high to low (reverse so we start at next)
                for yval in next_y + 1..y {
                    red_cols.update(x, yval);
                    red_rows.update(yval, x);
                }
            }
            continue;
        }
    }

    if VERBOSE && c < 100 {
        // Debug print
        let mut state: Vec<Vec<i8>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row: Vec<i8> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(0);
            }
            state.push(row);
        }

        for (y, range) in red_rows.index.iter() {
            for x in range.min..range.max + 1 {
                state[*y][x] = 1
            }
        }
        for (x, range) in red_cols.index.iter() {
            for y in range.min..range.max + 1 {
                state[y][*x] = 1
            }
        }

        println!("\nstate:");
        for y in 0..height {
            for x in 0..width {
                print!("{}", state[y][x]);
            }
            print!("\n");
        }
    }

    if VERBOSE {
        println!("\nred idx by row");
        for (r, val) in red_rows.index.iter() {
            println!("row {} | range {}", r, val);
        }
        println!("\nred idx by col");
        for (r, val) in red_cols.index.iter() {
            println!("col {} | range {}", r, val);
        }
        println!("");
    }

    // Check the areas withing bounds
    let mut max_area: i64 = 0;
    let mut max_a: (usize, usize) = (0, 0);
    let mut max_b: (usize, usize) = (0, 0);

    for i in 0..c {
        let a = &reds[i];
        for j in i + 1..c {
            let b = &reds[j];

            let mut minx = a.0;
            let mut maxx = b.0;
            if minx > maxx {
                minx = b.0;
                maxx = a.0;
            }

            let mut miny = a.1;
            let mut maxy = b.1;
            if miny > maxy {
                miny = b.1;
                maxy = a.1;
            }

            // Check all 4 corners are withing polygon
            if !is_inbounds(&red_cols, &red_rows, (minx, miny)) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (minx, maxy)) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (maxx, miny)) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (maxx, maxy)) {
                continue;
            }

            let width = (a.0 as i64 - b.0 as i64).abs() + 1;
            let height = (a.1 as i64 - b.1 as i64).abs() + 1;
            let mut area: i64 = width * height;
            if area < 0 {
                area *= -1;
            }
            if area > max_area {
                if VERBOSE {
                    println!(
                        "overriding area with ({},{}) ({},{}) | w={} h={} area={}",
                        a.0, a.1, b.0, b.1, width, height, area
                    )
                }
                max_a = (a.0, a.1);
                max_b = (b.0, b.1);
                max_area = area;
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    println!(
        "Anwser is \n{}\n at ({},{}) -> ({},{})",
        max_area, max_a.0, max_a.1, max_b.0, max_b.1
    );
    println!("processing time {}ms", elapsed.as_millis());
    println!("-------------------------------------");
}

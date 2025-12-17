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

static VERBOSE: bool = true;

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
    index: HashMap<usize, Vec<MinMax>>,
}

impl IdxRange {
    fn add_range(&mut self, key: usize, mut start: usize, mut end: usize) {
        if start > end {
            let tmp = start;
            start = end;
            end = tmp;
        }

        if let None = self.index.get(&key) {
            let mut newvec: Vec<MinMax> = Vec::new();
            newvec.push(MinMax {
                min: start,
                max: end,
            });
            self.index.insert(key, newvec);
        }

        // append to existing range if possible
        let c = self.index[&key].len();
        for i in 0..c {
            let r = self.index.get_mut(&key).unwrap();

            if end < r[i].min || start > r[i].max {
                continue;
            }

            // expand left
            if start < r[i].min {
                r.get_mut(i).unwrap().min = start;
            }
            if end > r[i].max {
                // expand right
                r.get_mut(i).unwrap().max = end;
            }

            // updated by extending
            return;
        }

        self.index.get_mut(&key).unwrap().push(MinMax {
            min: start,
            max: end,
        });
    }

    fn is_inrange(&self, key: usize, val: usize) -> bool {
        if let None = self.index.get(&key) {
            return false;
        }
        // technically means is touching an edge so we know it's valid / inside / 1 edge only
        for r in &self.index[&key] {
            if val >= r.min && val <= r.max {
                return true;
            }
        }
        return false;
    }

}

fn is_inbounds(
    red_cols: &IdxRange,
    red_rows: &IdxRange,
    pos: (usize, usize),
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(cached_val) = cache.get(&pos) {
        return *cached_val;
    }
    // count the number of edges hit on the perimeter
    if red_rows.is_inrange(pos.1, pos.0) || red_cols.is_inrange(pos.0, pos.1) {
        if VERBOSE {
            println!(
                "test ({},{}) is in bounds TRUE (by direct hit)",
                pos.0, pos.1
            )
        }
        cache.insert(pos, true);
        return true;
    }

    let edges = red_rows.raycast_ranges_to_zero(pos.1, pos.0)
        + red_cols.raycast_ranges_to_zero(pos.0, pos.1);
    let res = edges % 2 > 0;
    if VERBOSE {
        println!(
            "test ({},{}) is in bounds {} (by raycast count {})",
            pos.0, pos.1, res, edges
        )
    }
    cache.insert(pos, res);
    return res;
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
        let dx = next_x as i32 - x as i32;
        if next_y == y && dx != 0 {
            // add_range will take care of reversing order if dx < 0
            red_rows.add_range(y, x, next_x);
            continue;
        }
        // green tiles going vertically on x
        let dy = next_y as i32 - y as i32;
        if next_x == x && dy != 0 {
            // add_range will take care of reversing order if dx < 0
            red_cols.add_range(x, y, next_y);
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

        for (y, ranges) in red_rows.index.iter() {
            for r in ranges.iter() {
                for x in r.min..r.max + 1 {
                    state[*y][x] = 1
                }
            }
        }
        for (x, ranges) in red_cols.index.iter() {
            for r in ranges.iter() {
                for y in r.min..r.max + 1 {
                    state[y][*x] = 1
                }
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
        for (row, ranges) in red_rows.index.iter() {
            for r in ranges {
                println!("row {} | range {} {}", row, r.min, r.max);
            }
        }
        println!("\nred idx by col");
        for (col, ranges) in red_cols.index.iter() {
            for r in ranges {
                println!("col {} | range {} {}", col, r.min, r.max);
            }
        }
        println!("");
    }

    // Check the areas withing bounds
    let mut max_area: i64 = 0;
    let mut max_a: (usize, usize) = (0, 0);
    let mut max_b: (usize, usize) = (0, 0);
    let mut cache: HashMap<(usize, usize), bool> = HashMap::new();

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
            if !is_inbounds(&red_cols, &red_rows, (minx, miny), &mut cache) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (minx, maxy), &mut cache) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (maxx, miny), &mut cache) {
                continue;
            }
            if !is_inbounds(&red_cols, &red_rows, (maxx, maxy), &mut cache) {
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

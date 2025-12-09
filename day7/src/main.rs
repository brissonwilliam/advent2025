use std::collections::{HashMap, HashSet};

mod data;

static TESTINPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

static TESTINPUT2: &str = "
.......S.......
...............
.......^.......
...............
......^........
...............
";

static VERBOSE: bool = false;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 7");
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
    part2(input);
}

fn part1(mut input: String) {
    let start_time = std::time::Instant::now();
    let mut answer = 0;

    let width = input.lines().next().unwrap().len();
    let width_real = width + 1;
    let height = input.lines().count();

    let bytes = unsafe { input.as_bytes_mut() };
    bytes[(width + 1) + (width / 2)] = b'|';

    for y in 1..height {
        for x in 0..width {
            let pos = y * width_real + x;

            let c = char::from(bytes[pos]);
            let last_line_c = char::from(bytes[pos - width_real]);

            if VERBOSE {
                println!(
                    "processing char {} at ({},{}). pos is {}. last char is {}",
                    c, x, y, pos, last_line_c
                );
            }

            // We either split the beam ,continue the beam or do nothing
            if last_line_c != '|' {
                continue;
            }

            if c == '^' {
                if VERBOSE {
                    println!("beam ^ detected at {} ({}, {})", pos, x, y);
                }

                // split the beam
                // check for adjacent beams
                // Only count if we added a new beam
                let mut did_split = false;

                let previous_c = bytes[pos - 1];
                if (x as i32 - 1) >= 0 && previous_c != b'|' {
                    bytes[pos - 1] = b'|';
                    did_split = true;
                }

                if x + 1 < width && bytes[pos + 1] != b'|' {
                    bytes[pos + 1] = b'|';
                    did_split = true;
                }

                if did_split {
                    answer += 1;
                }
                continue;
            }
            bytes[pos] = b'|';
        }
    }

    if VERBOSE {
        println!("{}", input);
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 1 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

fn explore_outcomes(
    mut input_state: String,
    x: i32,
    y: usize,
    width_real: usize,
    height: usize,
    memory: &mut HashMap<usize, i64>,
) -> i64 {
    let bytes = unsafe { input_state.as_bytes_mut() };

    let mut outcomes = 0;
    for newy in y..height {
        // if we already visited the position, ignore.
        // It will be accounted for from another branch that already visitted this
        let pos = newy * width_real + x as usize;

        if bytes[pos] == b'^' {
            if let Some(&cached_outcomes) = memory.get(&pos) {
                outcomes = cached_outcomes;
                break;
            }

            // Compute left side paths
            // Ignore if it's a ^ neighbor or out of bounds
            if x as i32 - 1 >= 0 && bytes[pos - 1] == b'.' {
                let strcpy = String::from_utf8(bytes.to_vec()).unwrap();
                let outcomes_left =
                    explore_outcomes(strcpy, x - 1, newy, width_real, height, memory);
                if outcomes_left == 0 {
                    // dead end branch, no more splitters
                    outcomes += 1
                } else {
                    outcomes += outcomes_left
                }
            }

            // Compute right side paths
            // Ignore if it's a ^ neighbor or out of bounds
            if x as usize + 1 < (width_real - 1) && bytes[pos + 1] == b'.' {
                let strcpy = String::from_utf8(bytes.to_vec()).unwrap();
                let outcomes_right =
                    explore_outcomes(strcpy, x + 1, newy, width_real, height, memory);

                if outcomes_right == 0 {
                    // there will be no next values, only 2 dead end branches
                    outcomes += 1;
                } else {
                    outcomes += outcomes_right
                }
            }

            memory.insert(pos, outcomes);
            break;
        }
        bytes[pos] = b'|';
    }

    if VERBOSE {
        println!(
            "finished processing outcomes={},x={},y={}\n{}",
            outcomes, x, y, input_state,
        );
    }

    return outcomes;
}

fn part2(input: &str) {
    let start_time = std::time::Instant::now();

    let width = input.lines().next().unwrap().len();
    let width_real = width + 1;
    let height = input.lines().count();

    let mut memory: HashMap<usize, i64> = HashMap::new();

    let mut cpy = String::new();
    cpy.push_str(input);

    let bytes = unsafe { cpy.as_bytes_mut() };
    let x = width / 2;
    bytes[width_real + x] = b'|';

    let strcpy = String::from_utf8(bytes.to_vec()).unwrap();
    let answer = explore_outcomes(strcpy, x as i32, 1, width_real, height, &mut memory);

    println!("visited {}", memory.len());

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

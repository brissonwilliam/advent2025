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
    }
    input = input.trim();

    part1(String::from(input));
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

            if bytes[pos] == b'^' {
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

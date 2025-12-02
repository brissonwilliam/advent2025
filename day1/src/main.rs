use std::{fs, io::Read};

static TESTDATA: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

static VERBOSE: bool = false;

fn main() {
    println!("*******************");
    println!("ADVENT 2025 DAY 1");
    println!("*******************");
    part_one();
    part_two();
}

fn part_one() {
    println!("-------------------");
    println!("PART 1");
    println!("-------------------");

    // Read the input
    let mut filename = "input.txt";
    let mut args = std::env::args();
    if args.any(|a| return a == "--test") {
        println!("using test.txt");
        filename = "test.txt";
    }
    let mut f = fs::File::open(filename).unwrap(); // #yolo unwrap() (cloudflare level security right here)
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);

    // evaluate time for funz. Ignore file io
    let start_time = std::time::Instant::now();

    let mut pos: i16 = 50;
    let mut c: u16 = 0;
    let mut czero: i16 = 0;
    for l in input.lines() {
        // omit empty lines
        if l == "" {
            println!("ignoring empty line at {}", c);
            continue;
        }
        if l.len() < 2 {
            panic!("invalid line input {}", l);
        }
        let first_char = l.chars().nth(0).unwrap();
        let offset_str: String = l.chars().skip(1).collect();
        let offset: i16 = offset_str.parse().unwrap();

        let sign: i16;
        if first_char == 'L' || first_char == 'l' {
            sign = -1
        } else if first_char == 'R' || first_char == 'r' {
            sign = 1
        } else {
            panic!("invalid line input {}", l);
        }

        // instead of simulating every value change
        // be efficient: ignore full spins and just take the remainder of all 100 possible values
        let remainder = offset % 100;

        // Then juste apply remainder (or small offset in instruction) and re align when out of
        // bounds to  [0..99]
        // At this point we know we can only stay in same range, go pass upper bound 99, pass under bound 0,
        // or land on zero
        let newpos = pos + (sign * remainder);
        if newpos > 99 {
            pos = newpos - 100;
        } else if newpos < 0 {
            pos = newpos + 100;
        } else {
            pos = newpos;
        }
        if pos == 0 {
            czero += 1;
        }
        if VERBOSE {
            println!("instruction {} |  {} | {} | total: {}", c, l, pos, czero);
        }

        c += 1;
    }

    let elapsed = start_time.elapsed().as_micros();
    println!(
        "finished processing {} lines in {}us (without file io)",
        c, elapsed,
    );
    println!("Answer is: {}", czero);
}

fn part_two() {
    println!("-------------------");
    println!("PART 2");
    println!("-------------------");

    // Read the input
    let mut filename = "input.txt";
    let mut args = std::env::args();
    if args.any(|a| return a == "--test") {
        println!("using test.txt");
        filename = "test.txt";
    }
    let mut f = fs::File::open(filename).unwrap(); // #yolo unwrap() (cloudflare level security right here)
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);

    // evaluate time for funz. Ignore file io
    let start_time = std::time::Instant::now();

    let mut pos: i16 = 50;
    let mut c: u16 = 0;
    let mut czero: i16 = 0;
    for l in input.lines() {
        // omit empty lines
        if l == "" {
            println!("ignoring empty line at {}", c);
            continue;
        }
        if l.len() < 2 {
            panic!("invalid line input {}", l);
        }
        let first_char = l.chars().nth(0).unwrap();
        let offset_str: String = l.chars().skip(1).collect();
        let offset: i16 = offset_str.parse().unwrap();

        let sign: i16;
        if first_char == 'L' || first_char == 'l' {
            sign = -1
        } else if first_char == 'R' || first_char == 'r' {
            sign = 1
        } else {
            panic!("invalid line input {}", l);
        }

        // Same thing as part 1
        // but now we compute each time we go through 0 (even if not stopping)
        // Do so by computing full cycles with integer division
        // then like before, treat the remainder (whilst considering passes to zero)
        if offset > 99 {
            if VERBOSE {
                println!("  big offset, adding {}", offset / 100);
            }
            czero += offset / 100;
        }
        let remainder = offset % 100;

        // Here we know we can only go pass 0 one time or zero times
        let newpos = pos + (sign * remainder);
        if newpos > 99 {
            // doesn't count if last pos was 0, we did not cross it for real
            if pos != 0 {
                czero += 1;
            }
            pos = newpos - 100;
        } else if newpos < 0 {
            if pos != 0 {
                czero += 1;
            }
            pos = newpos + 100;
        } else {
            if newpos == 0 && pos != 0 {
                czero += 1
            }
            pos = newpos;
        }
        if VERBOSE {
            println!(
                "instruction {} | {} | pos: {} | total: {}",
                c, l, pos, czero
            );
        }

        c += 1;
    }

    println!(
        "finished processing {} lines in {}us (without file io)",
        c,
        start_time.elapsed().as_micros()
    );
    println!("Answer is: {}", czero);
}

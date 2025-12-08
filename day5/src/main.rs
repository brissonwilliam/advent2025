use std::collections;

mod data;

static TESTINPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
static TESTINPUT2: &str = "
3-5
10-14
16-20
12-18
9-21

1
5
8
11
17
32
";

static VERBOSE: bool = false;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 5");
    println!("********************");

    let mut input: &str = "";
    for a in std::env::args() {
        if a == "--test" {
            println!("using test data");
            input = TESTINPUT;
            break;
        }
        if a == "--test2" {
            println!("using test data2");
            input = TESTINPUT2;
            break;
        }
        input = data::INPUT;
    }
    input = input.trim();

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let start_time = std::time::Instant::now();

    let mut fresh: Vec<(i64, i64)> = Vec::new();
    let mut using_fresh = true;
    let mut answer = 0;
    for l in input.lines() {
        if l == "" {
            if VERBOSE {
                println!("found new line. Done processing fresh ranges");
            }
            using_fresh = false;
            continue;
        }

        if using_fresh {
            // Process fresh
            let mut s = l.split("-");
            let start: i64 = s.next().unwrap().parse().unwrap();
            let end: i64 = s.next().unwrap().parse().unwrap();
            if VERBOSE {
                println!("storing ({}, {})", start, end);
            }
            fresh.push((start, end));
            continue;
        }
        // Process spoiled
        // Lookup the fresh vec, only spoiled if we can't find it
        let spoiled: i64 = l.parse().unwrap();
        let mut found = false;
        for frange in fresh.iter() {
            if spoiled >= frange.0 && spoiled <= frange.1 {
                found = true;
                break;
            }
        }
        if found {
            if VERBOSE {
                println!("found spoiled {}", spoiled);
            }
            answer += 1;
        }
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 1 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

fn part2(input: &str) {
    let start_time = std::time::Instant::now();

    let mut fresh: Vec<(i64, i64)> = Vec::new();
    for l in input.lines() {
        if l == "" {
            if VERBOSE {
                println!("found new line. Done processing fresh ranges");
            }
            break;
        }

        let mut s = l.split("-");
        let start: i64 = s.next().unwrap().parse().unwrap();
        let end: i64 = s.next().unwrap().parse().unwrap();
        fresh.push((start, end));
        if VERBOSE {
            println!("processed line {}", l)
        }
    }
    // Merge compatible values
    fresh.sort_by(|a, b| return a.0.partial_cmp(&b.0).unwrap());

    let mut merged_fresh: Vec<(i64, i64)> = Vec::new();
    let mut iter = fresh.into_iter();
    merged_fresh.push(iter.next().unwrap());

    if VERBOSE {
        println!("Merging values of sorted vector");
        if VERBOSE {
            let last_merge = merged_fresh.last().unwrap();
            println!("({},{})", last_merge.0, last_merge.1);
        }
    }
    for frange in iter {
        if VERBOSE {
            println!("({},{})", frange.0, frange.1);
        }
        let last_merge = merged_fresh.last_mut().unwrap();
        // has overlapping values&
        let overlaps = frange.0 <= last_merge.1;
        if !overlaps {
            merged_fresh.push(frange);
            continue;
        }

        // extend
        if frange.1 > last_merge.1 {
            last_merge.1 = frange.1;
        }
    }

    // Compute the final distinct values
    if VERBOSE {
        println!("Computing final values of merged vector");
    }
    let mut answer = 0;
    for frange in merged_fresh {
        if VERBOSE {
            println!("({}, {})", frange.0, frange.1);
        }
        answer += (frange.1 - frange.0) + 1;
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

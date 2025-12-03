static INPUT: &str = "989133-1014784,6948-9419,13116184-13153273,4444385428-4444484883,26218834-26376188,224020-287235,2893-3363,86253-115248,52-70,95740856-95777521,119-147,67634135-67733879,2481098640-2481196758,615473-638856,39577-47612,9444-12729,93-105,929862406-930001931,278-360,452131-487628,350918-426256,554-694,68482544-68516256,419357748-419520625,871-1072,27700-38891,26-45,908922-976419,647064-746366,9875192107-9875208883,3320910-3352143,1-19,373-500,4232151-4423599,1852-2355,850857-889946,4943-6078,74-92,4050-4876";

static TESTINPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

static VERBOSE: bool = false;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 2");
    println!("********************");

    let input: &str;
    let mut args = std::env::args();
    if args.any(|a| return a == "--test") {
        println!("using test data");
        input = TESTINPUT;
    } else {
        input = INPUT;
    }

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let start_time = std::time::Instant::now();

    let mut answer: i64 = 0;
    let mut invalid_ids: String = String::new();

    for range in input.split(",") {
        let start_end = range.split_once("-").unwrap();
        let mut start = String::from(start_end.0).parse::<i64>().unwrap();
        let end = String::from(start_end.1).parse::<i64>().unwrap();

        if start > end {
            panic!(
                "PANIC: invalid range, end must be less or eq to start at {}. End: {} Start: {}",
                range, end, start,
            );
        }

        while start <= end {
            let s = start.to_string();
            let bad_id_seq = analyze_id(&s);
            if let Some(value) = bad_id_seq {
                invalid_ids.push_str(value);
                invalid_ids.push('\n');
                answer += start;
            }
            start += 1
        }
    }

    let elapsed = start_time.elapsed().as_millis();
    println!("------------ PART 1 -----------------");
    if VERBOSE {
        println!("invaild ids:");
        println!("{}", invalid_ids);
    }
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}ms", elapsed,);
    println!("-------------------------------------");
}

fn analyze_id(id: &str) -> Option<&str> {
    // If uneven nb of characters, there's no way we have a repeating sequence of 2 so the id is valid
    if id.len() % 2 != 0 {
        if VERBOSE {
            println!("valid id: uneven id len {}", id);
        }
        return None;
    }

    // check if it's all the same like "11111" or "88". Don't count single digit like "1"
    let mut chars = id.chars();
    let mut last_char = chars.next().unwrap();
    let mut all_same_chars = true;
    for c in chars {
        if c != last_char {
            all_same_chars = false;
            break;
        }
        last_char = c
    }
    if all_same_chars && id.len() > 1 {
        if VERBOSE {
            println!("INVALID ID DETECTED: all same chars detected for {}", id);
        }
        return Some(id);
    }

    // Looke for repeating sequences like 123123 or 44554455

    // Make sure that each half on the right side, at 'i', matches the left side
    // For example: 1212
    //              0123
    // id[0] == id[2 + 0] ->  1 == 1
    // id[1] == id[2 + 1] ->  2 == 2
    let mut i = 0;
    let half_idx = id.len() / 2;
    while i < half_idx {
        let left_char: char = id.chars().nth(i).unwrap();
        let right_char: char = id.chars().nth(half_idx + i).unwrap();
        if left_char != right_char {
            if VERBOSE {
                println!(
                    "valid id: left and right parts of id {} at idx {} do not match. Left : {} Right : {}",
                    id, i, left_char, right_char,
                );
            }
            return None;
        }
        i += 1;
    }

    if VERBOSE {
        println!(
            "INVALID ID DETECTED: left and right halves match for {}",
            id
        );
    }
    return Some(id);
}

fn part2(input: &str) {
    let start_time = std::time::Instant::now();

    let mut answer: i64 = 0;
    let mut invalid_ids: String = String::new();

    for range in input.split(",") {
        let start_end = range.split_once("-").unwrap();
        let mut start = String::from(start_end.0).parse::<i64>().unwrap();
        let end = String::from(start_end.1).parse::<i64>().unwrap();

        if start > end {
            panic!(
                "PANIC: invalid range, end must be less or eq to start at {}. End: {} Start: {}",
                range, end, start,
            );
        }

        while start <= end {
            let s = start.to_string();
            let bad_id_seq = analyze_id_2(&s);
            if let Some(value) = bad_id_seq {
                invalid_ids.push_str(value);
                invalid_ids.push('\n');
                answer += start;
            }
            start += 1
        }
    }

    let elapsed = start_time.elapsed().as_millis();
    println!("------------ PART 2 -----------------");
    if VERBOSE {
        println!("invaild ids:");
        println!("{}", invalid_ids);
    }
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}ms", elapsed,);
    println!("-------------------------------------");
}

fn analyze_id_2(id: &str) -> Option<&str> {
    // check if it's all the same like "11111" or "88". Don't count single digit like "1"
    let mut chars = id.chars();
    let mut last_char = chars.next().unwrap();
    let mut all_same_chars = true;
    for c in chars {
        if c != last_char {
            all_same_chars = false;
            break;
        }
        last_char = c
    }
    if all_same_chars && id.len() > 1 {
        if VERBOSE {
            println!("INVALID ID DETECTED: all same chars detected for {}", id);
        }
        return Some(id);
    }

    // Looke for repeating sequences like 123123123 (3 sequences, 9 chars) or 44554455
    // Start by decomposing each part of values until half of the input (because a seq of len 7
    // would not fit in 9 chars, only lower than 5)
    let mut sequences: Vec<&str> = Vec::new();
    let mut i = 1; // ignore the first char, we already checked if we what the same 1 char repating
    // N times earlier
    while i < id.len() / 2 {
        let seq = &id[0..i + 1]; // uper excluded, avoid 0..0 so +1
        sequences.push(seq);
        i += 1;
    }

    for seq in sequences {
        // if we can't split in even parts of seq length, ignore it
        let seqlen = seq.len();
        if id.len() % seqlen != 0 {
            continue;
        }

        // See if one the sequence is repeated for the entire length of id
        let mut j = 0;
        let potential_repeats = id.len() / seqlen;
        while j < potential_repeats {
            let start = j * seqlen;
            let end = start + seqlen;
            let part = &id[start..end];
            if part != seq {
                break;
            }
            j += 1;
        }

        if j >= potential_repeats {
            // all sequences matched !
            if VERBOSE {
                println!(
                    "INVALID ID DETECTED: repeating sequences for id: {} | seq : {}",
                    id, seq
                );
            }
            return Some(id);
        }
        // sequence not repeated, continue to next one
    }
    // Did not find any repeating seq, it is a valid id
    if VERBOSE {
        println!("valid id {}", id);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_id_2_returns_invalid_id() -> Result<(), String> {
        for test in [
            "55",
            "6464",
            "123123",
            "38593859",
            "118855118855",
            "118855118855",
            "451451",
            "11",
            "1212",
            "9999",
            "123123123",
            "55555",
            "111",
        ] {
            println!("-- test analyze id 2 returns invalid id {} --", test);
            let out = match analyze_id_2(test) {
                Some(val) => val,
                None => panic!(
                    "[TEST FAIL] {} should be an invalid id but was not (got None)",
                    test
                ),
            };
            assert_eq!(out, test.to_string())
        }
        Ok(())
    }
    #[test]
    fn test_analyze_id_returns_invalid_id() -> Result<(), String> {
        for test in [
            "55",
            "6464",
            "123123",
            "38593859",
            "118855118855",
            "118855118855",
            "451451",
            "11",
            "1212",
            "9999",
        ] {
            println!("-- test analyze id returns invalid id {} --", test);
            let out = match analyze_id(test) {
                Some(val) => val,
                None => panic!(
                    "[TEST FAIL] {} should be an invalid id but was not (got None)",
                    test
                ),
            };
            assert_eq!(out, test.to_string())
        }
        Ok(())
    }

    #[test]
    fn test_analyze_id_returns_none() -> Result<(), String> {
        for test in ["111", "9088", "51", "123", "1", "9091", "9081", "123123123"] {
            println!("-- test analyze id returns none with valid id {} --", test);
            match analyze_id(test) {
                Some(_) => panic!(
                    "[TEST FAIL] {} should be a valid id but was detected as invalid (got Some)",
                    test
                ),
                None => (),
            };
        }
        Ok(())
    }
}

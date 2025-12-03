static input: &str = "989133-1014784,6948-9419,13116184-13153273,4444385428-4444484883,26218834-26376188,224020-287235,2893-3363,86253-115248,52-70,95740856-95777521,119-147,67634135-67733879,2481098640-2481196758,615473-638856,39577-47612,9444-12729,93-105,929862406-930001931,278-360,452131-487628,350918-426256,554-694,68482544-68516256,419357748-419520625,871-1072,27700-38891,26-45,908922-976419,647064-746366,9875192107-9875208883,3320910-3352143,1-19,373-500,4232151-4423599,1852-2355,850857-889946,4943-6078,74-92,4050-4876";

static VERBOSE: bool = true;

fn main() {
    let mut answer = String::new();

    for range in input.split(",") {
        let start_end = range.split_once("-").unwrap();
        let mut start = String::from(start_end.0).parse::<i32>().unwrap();
        let end = String::from(start_end.1).parse::<i32>().unwrap();

        if end > start {
            panic!(
                "invalid range, end must be less or eq to start at {}",
                range
            );
        }

        while start <= end {
            let s = start.to_string();
            let bad_id_seq = analyze_id(&s);
            if let Some(value) = bad_id_seq {
                answer.push_str(value);
            }
            start += 1
        }
    }

    println!("Anwser is \n{}\n", answer)
}

fn analyze_id(id: &str) -> Option<&str> {
    if id.len() < 2 {
        if VERBOSE {
            println!("INPUT TOO SMALL TO PROCESS {}", id)
        }
        return None;
    }

    // check each string char starting at idx 1
    // Make sure it's equal to previous char, or following a patern (+- 1)
    // Keep track of the last known sequence to allow things like 6969 or 420420
    // If the latest sequence is ever different than last known one
    // - a char doesn't match as we build it
    // - exceeds length
    // then naturally we don't have a repeating sequence
    let mut seq = String::new();
    let mut last_seq = String::new();
    let mut chars = id.chars();
    let mut last_char = chars.next().unwrap();
    seq.push(last_char);

    if VERBOSE {
        println!(
            "processing char {} | last char {} | seq {} | last seq {} | ",
            last_char, "none", seq, last_seq
        )
    }

    for c in chars {
        seq.push(c);

        let is_last_char_diff = is_char_sequence_break(c, last_char);
        if last_seq.len() > 0 {
            // swap the sequence only if there is a previous one to swap with, we don't want 6464
            // to break on the second char
            if is_last_char_diff {
                if VERBOSE {
                    println!(
                        "SEQUENCE BREAK DETECTED | char {} | last char {}",
                        c, last_char
                    )
                }
                // sequence break
                last_seq = seq;
                seq = String::new();
            }

            if seq.len() > last_seq.len() {
                if VERBOSE {
                    println!(
                        "processed char {} | last char was {} | seq {} | last seq {} | SEQUENCE LONGER THAN PREVIOUS DETECTED",
                        c, last_char, seq, last_seq
                    )
                }
                return None;
            }
            // compare the char to previous sequence, making sure it doesn't differ
            let idx = seq.len() - 1;
            let compare_last_seq_char = last_seq.chars().nth(idx).unwrap();
            if c != compare_last_seq_char {
                if VERBOSE {
                    println!(
                        "processed char {} | last char was {} | seq {} | last seq {} | SEQUENCE UNEQUAL DETECTED",
                        c, last_char, seq, last_seq
                    )
                }
                return None;
            }
        }
        if VERBOSE {
            println!(
                "processed char {} | last char was {} | seq {} | last seq {} | ",
                c, last_char, seq, last_seq
            )
        }
        last_char = c;
    }

    if last_seq.len() == 0 {
        // No multi sequence, just 1.
        // Just return as invalid id if it's one long sequence of the same char
        let mut seqanalyze = seq.chars();
        last_char = seqanalyze.next().unwrap();

        for c in seq.chars() {
            if c != last_char {
                if VERBOSE {
                    println!(
                        "Single sequence {} VALID ID DETECTED | char {} | last char {}",
                        id, c, last_char
                    )
                }
                return None;
            }
        }
    }

    if VERBOSE {
        println!("{} INVALID ID DETECTED", id,)
    }
    // seq was one full sequence, or always equal to the last one
    // meaning we know for sure it is an invalid id
    return Some(id);
}

fn is_char_sequence_break(c: char, last_char: char) -> bool {
    return (c as i32) > (last_char as i32) + 1 || (c as i32) < (last_char as i32) - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_id_returns_invalid_id() -> Result<(), String> {
        for test in [
            "55",
            "6464",
            "123123",
            "123123123",
            "38593859",
            "118855118855",
            "118855118855",
            "11",
            "1212",
            "99999",
            "9088",
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
        for test in ["51", "123", "1", "9091", "9081"] {
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

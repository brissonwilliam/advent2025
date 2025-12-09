mod data;

static TESTINPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

static VERBOSE: bool = false;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 6");
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

    part1(input);
    part2(input);
}

struct Col {
    values: Vec<i64>,
    val_str: Vec<String>,
    max_val_len: usize,
    operator: String,
    total: i64,
}

impl Col {
    fn compute_total(&mut self) -> i64 {
        let op = self.operator.as_str();
        let mut iter = self.values.iter();
        let mut total: i64 = *iter.next().unwrap();
        for v in iter {
            match op {
                "*" => total *= v,
                "+" => total += v,
                "-" => total -= v,
                _ => panic!("unsupported operator {}", op),
            }
        }
        self.total = total;
        return total;
    }
}

fn part1(input: &str) {
    let start_time = std::time::Instant::now();

    // holds the number of vertical elements (rows, -1 because last row is sign)
    let n_vertical_sum = input.lines().count() - 1;
    let n_horizontal = input.lines().next().unwrap().split_whitespace().count();

    let mut table: Vec<Col> = Vec::with_capacity(n_horizontal);
    for _ in 0..n_horizontal {
        table.push(Col {
            values: Vec::with_capacity(n_vertical_sum),
            operator: String::new(),
            val_str: Vec::new(),
            max_val_len: 0,
            total: 0,
        });
    }
    if VERBOSE {
        println!(
            "n_vertical_sum {} | n_horizontal {}",
            n_vertical_sum, n_horizontal
        );
    }

    let mut row = 0;
    for l in input.lines() {
        if VERBOSE {
            println!("processing row idx {} | {}", l, row);
        }
        if row >= n_vertical_sum {
            // treat the operator row
            let mut col = 0;
            for val in l.split_whitespace() {
                table[col].operator = String::from(val);
                col += 1;
            }
        } else {
            // treat the values, store them in the column vector
            let mut col = 0;
            for val in l.split_whitespace() {
                table[col].values.push(val.parse().unwrap());
                col += 1;
            }
        }
        row += 1;
    }

    // compute totals
    let mut answer = 0;
    for c in table.iter_mut() {
        answer += c.compute_total();
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 1 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

fn part2(input: &str) {
    let start_time = std::time::Instant::now();

    // holds the number of vertical elements (rows, -1 because last row is sign)
    let n_vertical_sum = input.lines().count() - 1;
    let n_horizontal = input.lines().next().unwrap().split_whitespace().count();

    let mut table: Vec<Col> = Vec::with_capacity(n_horizontal);
    for col in 0..n_horizontal {
        // apparently, in input data, length is dynamic per column, not constant for the whole set
        // so resolve it for each col
        let mut max_len = 0;
        for row in 0..n_vertical_sum {
            let len = input
                .lines()
                .nth(row)
                .unwrap()
                .split_whitespace()
                .nth(col)
                .unwrap()
                .len();
            if len > max_len {
                max_len = len;
            }
        }
        table.push(Col {
            values: Vec::with_capacity(n_vertical_sum),
            operator: String::new(),
            val_str: Vec::with_capacity(n_vertical_sum),
            max_val_len: max_len,
            total: 0,
        });
    }
    if VERBOSE {
        println!(
            "n_vertical_sum {} | n_horizontal {}",
            n_vertical_sum, n_horizontal,
        );
    }

    for row in 0..n_vertical_sum {
        // treat the values, store them in the column vector
        let mut offset = 0;
        for col in 0..n_horizontal {
            let start = offset; // +1 space sperator per previous col, always
            let end = start + table[col].max_val_len; // don't include the space separator 
            let s = &input.lines().nth(row).unwrap()[start..end];
            if VERBOSE {
                println!(
                    "pushing {} at row {} col  {} | max col is {} | offset is {}",
                    s, row, col, table[col].max_val_len, offset,
                )
            }
            table[col].val_str.push(String::from(s));
            offset += table[col].max_val_len + 1;
        }
    }
    // treat the operator row
    let mut col = 0;
    for val in input.lines().last().unwrap().split_whitespace() {
        table[col].operator = String::from(val);
        col += 1;
    }

    // now that we have all values vertically stored, convert str to integer
    // and sum up!
    for c in table.iter_mut() {
        let mut i = c.max_val_len - 1;
        while i as i32 >= 0 {
            let mut newval = String::new();
            for v_str in c.val_str.iter() {
                let digit_str = &v_str.as_str()[i..i + 1];
                /*
                if VERBOSE {
                    println!("v_str {} -> digit_str {}", v_str, digit_str);
                }
                */
                newval.push_str(digit_str);
            }
            /*
            if VERBOSE {
                println!("pushing new val {}", newval);
            }
            */
            c.values.push(newval.trim().parse().unwrap());
            i -= 1;
        }
    }

    // compute totals
    let mut answer = 0;
    for c in table.iter_mut() {
        let total = c.compute_total();
        /*
        if VERBOSE {
            println!("total: {}", total)
        }
        */
        answer += total;
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

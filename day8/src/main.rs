mod data;

static TESTINPUT: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

static TESTINPUT2: &str = "
";

static VERBOSE: bool = true;

fn main() {
    println!("********************");
    println!("ADVENT 2025 DAY 8");
    println!("********************");

    let mut input: &str = data::INPUT;
    let mut limit = 1000;
    for a in std::env::args() {
        if a == "--test" {
            println!("using test data");
            input = TESTINPUT;
            limit = 10;
            break;
        }
        if a == "--test2" {
            println!("using test data 2");
            input = TESTINPUT2;
            break;
        }
    }
    input = input.trim();

    println!("limit {}", limit);

    part1(String::from(input), limit);
}

struct Junction {
    x: i64,
    y: i64,
    z: i64,
    idx_circuit: usize,
}

impl Junction {
    fn distance_from(&self, j: &Junction) -> i64 {
        let dx: i64 = j.x - self.x;
        let dy: i64 = j.y - self.y;
        let dz: i64 = j.z - self.z;

        let total: i64 = (dx * dx + dy * dy + dz * dz) as i64;
        return total;
    }
}

struct Circuit {
    count: i32,
}

fn part1(input: String, limit: i32) {
    let start_time = std::time::Instant::now();

    let mut junctions = parse_intput(input);
    let c = junctions.len();

    // pre-alloc circuits
    let mut circuits: Vec<Circuit> = Vec::with_capacity(c);

    // Evaluate distances. This is O(n^2) but we really gotta go through everything.
    let mut distances: Vec<(usize, usize, i64)> = Vec::with_capacity(c); // idx_a, idx_b, distance
    for i in 0..c {
        circuits.push(Circuit { count: 1 });
        for j in i + 1..c {
            // no need to revisit under i
            if i == j {
                continue;
            }
            let j1 = &junctions[i];
            let j2 = &junctions[j];
            let distance = j1.distance_from(j2);
            distances.push((i, j, distance));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));
    if VERBOSE {
        println!("--- sorted pairs ---");
        for d in distances.iter() {
            let j1 = &junctions[d.0];
            let j2 = &junctions[d.1];
            println!(
                "({},{},{}) | ({},{},{}) | distance {}",
                j1.x, j1.y, j1.z, j2.x, j2.y, j2.z, d.2,
            )
        }
        println!("--------------------")
    }

    // Build circuits by using the smallest distances
    // A circuit holds an array of junctions
    // A junction can only be in 1 circuit at a time
    // We are asked to only look at a limited number junctions to make circuits
    let mut conn_count = 0;
    for d in distances.iter() {
        if VERBOSE {
            println!("**conn_count {}", conn_count);
            for circ in circuits.iter() {
                print!(" csize={} | ", circ.count);
            }
            print!("\n");
        }
        if conn_count >= limit {
            if VERBOSE {
                println!("**conn_count {} REACHED LIMIT", conn_count)
            }
            break;
        }

        let j1 = &junctions[d.0];
        let c1 = j1.idx_circuit;
        let j2 = &junctions[d.1];
        let c2 = j2.idx_circuit;

        // If the current junction is already in a circuit
        // Check if we can add neighbor (if it's not set)
        // Otherwise do nothing
        if c1 == c2 {
            if VERBOSE {
                println!(
                    "junction ({},{},{}) HAS CIRCUIT {}, neighbor ({},{},{}) HAS CIRCUIT {}. BOTH in same circuits (NOOP)",
                    j1.x, j1.y, j1.z, c1, j2.x, j2.y, j2.z, c2
                )
            }
            continue;
        }

        if c1 != d.0 && c2 == d.1 {
            if VERBOSE {
                println!(
                    "junction ({},{},{}) HAS CIRCUIT, neighbor ({},{},{}) NO CIRCUIT circuit. Adding it",
                    j1.x, j1.y, j1.z, j2.x, j2.y, j2.z,
                )
            }
            junctions[d.1].idx_circuit = c1;
            circuits[c1].count += 1;
            circuits[c2].count -= 1;
            conn_count += 1;
            continue;
        }

        if c1 == d.0 && c2 != d.1 {
            if VERBOSE {
                println!(
                    "junction ({},{},{}) NO CIRCUIT , neighbor ({},{},{}) HAS circuit. Adding it",
                    j1.x, j1.y, j1.z, j2.x, j2.y, j2.z,
                )
            }
            junctions[d.0].idx_circuit = c2;
            circuits[c2].count += 1;
            circuits[c1].count -= 1;
            conn_count += 1;
            continue;
        }

        // Both junctions without circuit / are own their own. Merge them
        if c1 == d.0 && c2 == d.1 {
            if VERBOSE {
                println!(
                    "junction ({},{},{}) NO CIRCUIT, neighbor ({},{},{}) NO CIRCUIT. MERGE INTO {}",
                    j1.x, j1.y, j1.z, j2.x, j2.y, j2.z, c1
                )
            }

            junctions[d.1].idx_circuit = c1;
            circuits[c1].count += 1;
            circuits[c2].count -= 1;
            conn_count += 1;
            continue;
        }

        panic!("undefined pair processing");
    }

    // At last! Get the top 3 circuits to make the response
    circuits.sort_by(|a, b| return a.count.cmp(&b.count).reverse());

    if VERBOSE {
        println!("made {} circuits in total", circuits.len());
        for i in 0..circuits.len() {
            let circuit = &circuits[i];
            if circuit.count == 0 {
                break;
            }
            if VERBOSE {
                println!("circuit {} has {}", i, circuit.count)
            }
        }
    }

    let mut answer = 1;
    for i in 0..circuits.len() {
        if i == 3 {
            break;
        }
        let circuit = &circuits[i];
        answer *= circuit.count;
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 1 -----------------");
    println!("Anwser is \n{}\n", answer);
    println!("processing time {}us", elapsed.as_micros());
    println!("-------------------------------------");
}

fn parse_intput(input: String) -> Vec<Junction> {
    let mut junctions: Vec<Junction> = Vec::with_capacity(1000);
    let mut i = 0;
    for l in input.lines() {
        let mut vals = l.split(",");
        let j = Junction {
            x: vals.next().unwrap().parse().unwrap(),
            y: vals.next().unwrap().parse().unwrap(),
            z: vals.next().unwrap().parse().unwrap(),
            idx_circuit: i,
        };
        junctions.push(j);
        i += 1;
    }
    return junctions;
}

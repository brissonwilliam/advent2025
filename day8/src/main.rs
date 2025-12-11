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
    println!("ADVENT 2025 DAY 7");
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

    part1(String::from(input), limit);
}

struct Junction {
    x: i32,
    y: i32,
    z: i32,
    idx_nearest: Option<usize>,
    distance_nearest: f64,
    idx_circuit: Option<usize>,
}

impl Junction {
    fn distance_from(&self, j: &Junction) -> f64 {
        let dx: i32 = (j.x - self.x) * (j.x - self.x);
        let dy: i32 = (j.y - self.y) * (j.y - self.y);
        let dz: i32 = (j.z - self.z) * (j.z - self.z);

        let total: i64 = (dx + dy + dz) as i64;
        return (total as f64).sqrt();
    }
}

struct Circuit {
    count: i32,
}

fn part1(input: String, limit: i32) {
    let start_time = std::time::Instant::now();

    let mut junctions = parse_intput(input);
    let mut circuits: Vec<Circuit> = Vec::with_capacity(100);

    let c = junctions.iter().count();

    // Evaluate distances. This is O(n^2) but we really gotta go through everything.
    // Prepare sorted indexes in advance for next step
    let mut sorted_idx: Vec<usize> = Vec::with_capacity(c);
    for i in 0..c {
        sorted_idx.push(i);
        let j = junctions.iter_mut().nth(i).unwrap();
        let mut min_d = f64::MAX;

        for idx in 0..c {
            if i == idx {
                // do not process self, we will get 0
                continue;
            }
            let distance = j.distance_from(j);
            if distance < min_d {
                min_d = distance;
                j.idx_nearest = Some(idx);
                j.distance_nearest = distance;
            }
        }
    }

    // Build circuits by using the smallest distances
    // A circuit holds an array of junctions
    // A junction can only be in 1 circuit at a time
    // We are asked to only look at a limited number junctions to make circuits

    // We can't sort the origin vector because that would mess up idx_nearest
    // Instead, build an vec idx of indexes
    sorted_idx.sort_by(|a, b| {
        let junc_a = junctions.iter().nth(a.to_owned()).unwrap();
        let junc_b = junctions.iter().nth(b.to_owned()).unwrap();
        return junc_a.distance_nearest.total_cmp(&junc_b.distance_nearest);
    });

    let mut conn_count = 0;
    for i in 0..c {
        if conn_count >= limit {
            if VERBOSE {
                println!(
                    "reached limit after analysing {} junctions and making {} connections",
                    c, conn_count,
                )
            }
            break;
        }

        let idx_smallest = sorted_idx.iter().nth(i as usize).unwrap();
        let junction = junctions.iter().nth(*idx_smallest).unwrap();

        // Ignore if the junction was already put in a circuit before (being referenced)
        if let Some(idx_circuit) = junction.idx_circuit {
            if VERBOSE {
                println!(
                    "skip junction {} ({},{},{}) already in circuit {}",
                    idx_smallest, junction.x, junction.y, junction.z, idx_circuit
                )
            }
            continue;
        }

        let idx_nearest = junction.idx_nearest.unwrap();
        let junction_nearest = junctions.iter().nth(idx_nearest).unwrap();

        if VERBOSE {
            println!(
                "j idx {} pos ({},{},{}) refers to ({},{},{}) neighbor in circuit {}",
                idx_smallest,
                junction.x,
                junction.y,
                junction.z,
                junction_nearest.x,
                junction_nearest.y,
                junction_nearest.z,
                junction_nearest.idx_circuit.unwrap_or(9999999),
            )
        }

        // Look at what the junction is referring to if any
        // If the referred nearest junction is already in a circuit, simply
        // add this one to the circuit
        // Otherwise, make a new circuit
        if let Some(idx_circuit) = junction_nearest.idx_circuit {
            let circuit = circuits.iter_mut().nth(idx_circuit).unwrap();
            circuit.count += 1;
            junctions.iter_mut().nth(*idx_smallest).unwrap().idx_circuit = Some(idx_circuit);
        } else {
            let idx_circuit = circuits.iter().count();
            if VERBOSE {
                println!("pushing new circuit {}", idx_circuit);
            }
            circuits.push(Circuit { count: 2 });
            junctions.iter_mut().nth(*idx_smallest).unwrap().idx_circuit = Some(idx_circuit);
            junctions.iter_mut().nth(idx_nearest).unwrap().idx_circuit = Some(idx_circuit);
        }
        conn_count += 1;
    }

    // At last! Get the top 3 circuits to make the response
    circuits.sort_by(|a, b| return a.count.cmp(&b.count));

    if VERBOSE {
        println!("made {} circuits in total", circuits.iter().count());
    }

    let mut answer = 1;
    for i in 0..3 {
        let circuit = circuits.iter().nth(i).unwrap();
        if VERBOSE {
            println!("circuit {} has {}", i, circuit.count)
        }
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
    for l in input.lines() {
        let mut vals = l.split(",");
        let j = Junction {
            x: vals.next().unwrap().parse().unwrap(),
            y: vals.next().unwrap().parse().unwrap(),
            z: vals.next().unwrap().parse().unwrap(),
            idx_nearest: None,
            distance_nearest: 0.0,
            idx_circuit: None,
        };
        junctions.push(j);
    }
    return junctions;
}

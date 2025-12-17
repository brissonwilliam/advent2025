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
7,1
11,1
11,7
9,7
9,5
5,5
5,7
3,7
3,5
2,5
2,3
7,3
";

static VERBOSE: bool = false;

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

    let mut reds: Vec<(i64, i64)> = Vec::new();
    for l in input.lines() {
        let mut s = l.split(",");
        let x: i64 = s.next().unwrap().parse().unwrap();
        let y: i64 = s.next().unwrap().parse().unwrap();

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

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

struct Edge {
    start: Point,
    end: Point,
}

fn new_edge(start: &mut Point, end: &mut Point) -> Edge {
    if start.y == end.y {
        if end.x < start.x {
            let tmp = start.x;
            start.x = end.x;
            end.x = tmp;
        }
    } else if start.x == end.x {
        if end.y < start.y {
            let tmp = start.y;
            start.y = end.y;
            end.y = tmp;
        }
    }

    return Edge {
        start: start.to_owned(),
        end: end.to_owned(),
    };
}

fn parse_line(l: &str) -> Point {
    let mut s = l.split(",");
    let x: i64 = s.next().unwrap().parse().unwrap();
    let y: i64 = s.next().unwrap().parse().unwrap();
    return Point { x, y };
}

fn part2(input: String) {
    let start_time = std::time::Instant::now();

    let mut reds: Vec<Point> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut i = 0;
    let lines = input.lines().count();
    let mut last_point = parse_line(input.lines().nth(lines - 1).unwrap());
    while i < lines {
        let l = input.lines().nth(i).unwrap();
        let end = parse_line(l);
        if end.x > width {
            width = end.x + 1;
        }
        if end.y > height {
            height = end.y + 1;
        }

        reds.push(end.clone());
        edges.push(new_edge(&mut last_point, &mut end.clone()));
        last_point = end;

        i += 1;
    }

    // Check the areas within bounds
    struct Candidate {
        a: Point,
        b: Point,
        area: i64,
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    let c = reds.len();

    for i in 0..c {
        let a = &reds[i];
        // Quick test to exclude out of bounds of poly rectangle
        for j in i + 1..c {
            let b = &reds[j];

            let width = (b.x as i32 - a.x as i32).abs() + 1;
            let height = (b.y as i32 - a.y as i32).abs() + 1;
            let mut area: i64 = width as i64 * height as i64;
            if area < 0 {
                area *= -1;
            }

            candidates.push(Candidate {
                a: Point { x: a.x, y: a.y },
                b: Point { x: b.x, y: b.y },
                area,
            });
        }
    }

    // sort candidates by area descending
    // stop as soon as we find one
    candidates.sort_by(|a, b| return a.area.cmp(&b.area).reverse());

    let mut resp: Option<&Candidate> = None;
    for cand in candidates.iter() {
        if !intersects(&cand.a, &cand.b, &edges) {
            resp = Some(cand);
            break;
        }
    }

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    if VERBOSE && width < 100 {
        print_state(&edges, width, height);
    }
    // not 16300900
    // not 4730060646
    // should be 14398...
    let answer = resp.unwrap();
    println!(
        "Anwser is \n{}\n at ({},{}) -> ({},{})",
        answer.area, answer.a.x, answer.a.y, answer.b.x, answer.b.y
    );
    println!("processing time {}ms", elapsed.as_millis());
    println!("-------------------------------------");
}

fn intersects(a: &Point, b: &Point, edges: &Vec<Edge>) -> bool {
    let rec_minx = a.x.min(b.x);
    let rec_maxx = a.x.max(b.x);
    let rec_miny = a.y.min(b.y);
    let rec_maxy = a.y.max(b.y);

    for e in edges {
        // we know rectangles are always drawn from perimeter, not some arbitrary
        // positions. So we can just test intersection with edges
        // AABB collision testing
        let edge_minx = e.start.x.min(e.end.x);
        let edge_maxx = e.start.x.max(e.end.x);
        let edge_miny = e.start.y.min(e.end.y);
        let edge_maxy = e.start.y.max(e.end.y);
        if rec_minx < edge_maxx
            && rec_maxx > edge_minx
            && rec_miny < edge_maxy
            && rec_maxy > edge_miny
        {
            return true;
        }
    }

    // did not intersect with any edge
    return false;
}

fn print_state(edges: &Vec<Edge>, width: i64, height: i64) {
    // Debug print
    let mut state: Vec<Vec<i8>> = Vec::with_capacity(height as usize);
    for _ in 0..height {
        let mut row: Vec<i8> = Vec::with_capacity(width as usize);
        for _ in 0..width {
            row.push(0);
        }
        state.push(row);
    }

    for e in edges.iter() {
        println!(
            "edge ({},{}) - ({},{})",
            e.start.x, e.start.y, e.end.x, e.end.y
        );
        if e.start.y == e.end.y {
            for x in e.start.x..e.end.x + 1 {
                state[e.start.y as usize][x as usize] = 1;
            }
        } else if e.start.x == e.end.x {
            for y in e.start.y..e.end.y + 1 {
                state[y as usize][e.start.x as usize] = 1;
            }
        }
    }

    println!("\nstate:");
    for y in 0..height {
        for x in 0..width {
            print!("{}", state[y as usize][x as usize]);
        }
        print!("\n");
    }
}

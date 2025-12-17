use std::collections::HashMap;

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

static VERBOSE: bool = true;

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

    let mut reds: Vec<(usize, usize)> = Vec::new();
    for l in input.lines() {
        let mut s = l.split(",");
        let x: usize = s.next().unwrap().parse().unwrap();
        let y: usize = s.next().unwrap().parse().unwrap();

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
    x: usize,
    y: usize,
}

struct Edge {
    start: Point,
    end: Point,
    lenx: i32,
    leny: i32,
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
        lenx: (end.x as i32 - start.x as i32),
        leny: (end.y as i32 - start.y as i32),
    };
}

fn parse_line(l: &str) -> Point {
    let mut s = l.split(",");
    let x: usize = s.next().unwrap().parse().unwrap();
    let y: usize = s.next().unwrap().parse().unwrap();
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

    if VERBOSE && width < 100 {
        print_state(&edges, width, height);
    }

    // Check the areas within bounds
    let mut cache: HashMap<Point, bool> = HashMap::new();
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

            if !is_valid_rec(a, b, &edges, &mut cache) {
                continue;
            }

            let width = (a.x as i32 - b.x as i32).abs() + 1;
            let height = (a.y as i32 - b.y as i32).abs() + 1;
            let mut area: i64 = width as i64 * height as i64;
            if area < 0 {
                area *= -1;
            }

            candidates.push(Candidate {
                a: a.clone(),
                b: b.clone(),
                area,
            });
        }
    }

    // sort candidates by area descending
    // stop as soon as we find one
    candidates.sort_by(|a, b| return a.area.cmp(&b.area).reverse());
    let answer = candidates.first().unwrap();

    let elapsed = start_time.elapsed();
    println!("------------ PART 2 -----------------");
    if VERBOSE && width < 100 {
        print_state(&edges, width, height);
    }
    // not 16300900
    // not 4730060646
    // should be 14398...
    println!(
        "Anwser is \n{}\n at ({},{}) -> ({},{})",
        answer.area, answer.a.x, answer.a.y, answer.b.x, answer.b.y
    );
    println!("processing time {}ms", elapsed.as_millis());
    println!("-------------------------------------");
}

fn is_valid_rec(a: &Point, b: &Point, edges: &Vec<Edge>, cache: &mut HashMap<Point, bool>) -> bool {
    let mut minx = a.x;
    let mut maxx = b.x;
    if minx > maxx {
        minx = b.x;
        maxx = a.x;
    }

    let mut miny = a.y;
    let mut maxy = b.y;
    if miny > maxy {
        miny = b.y;
        maxy = a.y;
    }

    // Check all 4 corners are within polygon
    let corners: [Point; 4] = [
        Point { x: minx, y: miny },
        Point { x: minx, y: maxy },
        Point { x: maxx, y: miny },
        Point { x: maxx, y: maxy },
    ];
    for c in corners {
        // check cache
        if let Some(cached_val) = cache.get(&c) {
            if *cached_val {
                continue;
            }
            return false;
        }

        let is_within_poly = corner_is_in_poly(&c, edges);
        cache.insert(c, is_within_poly);
        if !is_within_poly {
            // stop processing this rectangle corners
            return false;
        }
        // continue testing other corners
    }

    /*
    // Now validate each point in perimeter of the rectangle.
    // This is to avoid shapes like these
    // where the obbtom would be caught from 3,5 to 11,7
    // 000000000000
    // 000000011111
    // 000000010001
    // 001111110001
    // 001000000001
    // 001+01111101
    // 000101---101
    // 000111---11+
    //       |||
    //       bad
    for y in [miny, maxy] {
        for x in minx..maxx + 1 {
            let pos = Point { x, y };
            // check cache
            if let Some(cached_val) = cache.get(&pos) {
                if *cached_val {
                    continue;
                }
                return false;
            }

            let is_within_poly = corner_is_in_poly(&pos, edges);
            cache.insert(pos, is_within_poly);
            if !is_within_poly {
                // stop processing this rectangle corners
                return false;
            }
        }
    }

    for x in [minx, maxx] {
        for y in miny..maxy + 1 {
            let pos = Point { x, y };
            // check cache
            if let Some(cached_val) = cache.get(&pos) {
                if *cached_val {
                    continue;
                }
                return false;
            }

            let is_within_poly = corner_is_in_poly(&pos, edges);
            cache.insert(pos, is_within_poly);
            if !is_within_poly {
                // stop processing this rectangle corners
                return false;
            }
        }
    }
    */

    return true;
}

fn corner_is_in_poly(c: &Point, edges: &Vec<Edge>) -> bool {
    // check if within y range
    let mut hits = 0;
    for e in edges.iter() {
        // If we are at a corner already,
        // computing hits will cause issues
        // Example of 3 edges, would result in 3 hits
        // .X##.
        // .#.#
        // Just exit early we don,t need to test other edges
        /*
        if *c == e.start || *c == e.end {
            if VERBOSE {
                println!("corner ({},{}) in poly ({})", c.x, c.y, true)
            }
            return true;
        }
        */
        let within_y = c.y >= e.start.y && c.y <= e.end.y;
        let within_x = c.x >= e.start.x && c.x <= e.end.x;

        // Avoid division by zero
        // If is on an edge, don't ray cast to avoid cases like this
        // where 2 edges would be hit (but is inside)
        // .##X##
        // .#...#
        if e.leny == 0 && c.y == e.start.y && within_x {
            if VERBOSE {
                println!(
                    "corner ({},{}) in poly (true) after flat y within x of edge ({},{})({},{})",
                    c.x, c.y, e.start.x, e.start.y, e.end.x, e.end.y
                );
            }
            return true;
        }

        if e.lenx == 0 && c.x == e.start.x && within_y {
            if VERBOSE {
                println!(
                    "corner ({},{}) in poly (true) after flat x within y of edge ({},{})({},{})",
                    c.x, c.y, e.start.x, e.start.y, e.end.x, e.end.y
                );
            }
            return true;
        }

        // Do not test edge on x if we're not within y
        if !within_y {
            continue;
        }

        // Last way to compute: raycast on x
        // uses a equation derivate of y(p) = mx + b to test if point intersects the line
        // https://www.youtube.com/watch?v=TA8XQgiao4M
        let m = e.leny as f32 / e.lenx as f32;
        let intersect_x = (c.y - e.start.y) as f32 / m + e.start.x as f32;
        // let intersect_y = p.y; // always true, we tested earlier

        // Since we know we're within y, if the point is on the left of the intersection
        // we know it would hit the edge by casting a ray to the right
        let intersects = (c.x as f32) <= intersect_x;
        if intersects {
            hits += 1;
        }
    }

    let is_within_poly = hits % 2 != 0;
    if VERBOSE {
        println!(
            "corner ({},{}) in poly ({}) after {} hits",
            c.x, c.y, is_within_poly, hits
        );
    }

    return is_within_poly;
}

fn print_state(edges: &Vec<Edge>, width: usize, height: usize) {
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
                state[e.start.y][x] = 1;
            }
        } else if e.start.x == e.end.x {
            for y in e.start.y..e.end.y + 1 {
                state[y][e.start.x] = 1;
            }
        }
    }

    println!("\nstate:");
    for y in 0..height {
        for x in 0..width {
            print!("{}", state[y][x]);
        }
        print!("\n");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_inside_poly() -> Result<(), String> {
        let input = TESTINPUT2.trim();
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
        print_state(&edges, width, height);

        for x in 7..11 + 1 {
            let y = 1;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 7..11 + 1 {
            let y = 2;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 2..11 + 1 {
            let y = 3;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 2..11 + 1 {
            let y = 4;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 2..11 + 1 {
            let y = 5;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 9..11 + 1 {
            let y = 6;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 9..11 + 1 {
            let y = 7;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 3..5 + 1 {
            let y = 6;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }
        for x in 3..5 + 1 {
            let y = 7;
            println!("-- test {},{} should be within poly --", x, y);
            let res = corner_is_in_poly(&Point { x, y }, &edges);
            assert_eq!(true, res)
        }

        println!("-- test {},{} should NOT be within poly --", 7, 7);
        let res = corner_is_in_poly(&Point { x: 7, y: 6 }, &edges);
        assert_eq!(false, res);
        println!("-- test {},{} should NOT be within poly --", 7, 7);
        let res = corner_is_in_poly(&Point { x: 7, y: 7 }, &edges);
        assert_eq!(false, res);
        println!("-- test {},{} should NOT be within poly --", 7, 7);
        let res = corner_is_in_poly(&Point { x: 7, y: 8 }, &edges);
        assert_eq!(false, res);
        println!("-- test {},{} should NOT be within poly --", 7, 7);
        let res = corner_is_in_poly(&Point { x: 6, y: 7 }, &edges);
        assert_eq!(false, res);

        Ok(())
    }
}

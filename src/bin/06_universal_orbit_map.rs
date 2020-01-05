use std::collections::HashMap;

type Orbits<'a, 'b> = HashMap<&'a str, Vec<&'b str>>;

fn parse_orbit(s: &str) -> (&str, &str) {
    let splits: Vec<_> = s.splitn(2, ')').collect();
    match splits[..] {
        [a, b] => (a, b),
        _ => panic!("bad orbit {}", s),
    }
}

fn parse_orbits<'a>(s: &'a str) -> (Option<&str>, Option<&'a str>, Orbits<'a, 'a>) {
    let mut you_orbit = None;
    let mut san_orbit = None;
    let mut orbit = HashMap::new();

    for line in s.lines() {
        let (a, b) = parse_orbit(line);
        if b == "YOU" {
            you_orbit = Some(a);
        } else if b == "SAN" {
            san_orbit = Some(a);
        }
        orbit.entry(a).or_insert_with(Vec::new).push(b);
        orbit.entry(b).or_insert_with(Vec::new).push(a);
    }

    (you_orbit, san_orbit, orbit)
}

fn main() {
    use adventofcode::search::{bfs, Gen};

    let input = adventofcode::read_input_file();
    let (you_orbit, san_orbit, orbits) = parse_orbits(&input);
    let neigh = |s: &str| orbits[s].iter().cloned();

    let goals = bfs("COM", orbits.len(), neigh, |_| true).goals;
    let sum = goals.iter().map(|&(_, d)| d).sum::<Gen>();
    println!("{}", sum);

    match (you_orbit, san_orbit) {
        (Some(yo), Some(so)) => {
            let result = bfs(yo, 1, neigh, |x| x == so);
            if result.goals.is_empty() {
                println!("impossible");
            } else {
                println!("{}", result.gen);
            }
        }
        _ => println!("nonexistent"),
    }
}

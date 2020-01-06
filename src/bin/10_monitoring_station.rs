use std::collections::{HashMap, HashSet};

const TO_DESTROY: usize = 200;

type Pos = (i32, i32);
type Field<'a> = (&'a HashSet<Pos>, i32, i32);

fn station(field: Field) -> (Pos, u32) {
    let (asteroids, _, _) = field;
    let mut detect = HashMap::new();
    let poses: Vec<_> = asteroids.iter().collect();
    for (i, &a1) in poses.iter().enumerate() {
        for &a2 in poses.iter().skip(i + 1) {
            if detects(field, *a1, *a2) {
                *detect.entry(a1).or_insert(0) += 1;
                *detect.entry(a2).or_insert(0) += 1;
            }
        }
    }
    let (a, b) = detect.iter().max_by_key(|(_, &v)| v).unwrap();
    (**a, *b)
}

fn detects(field: Field, a1: Pos, a2: Pos) -> bool {
    let (y1, x1) = a1;
    let (y2, x2) = a2;
    let dy = y2 - y1;
    let dx = x2 - x1;
    let g = gcd(dy, dx);

    g == 1 || asteroid_in_direction(field, a1, dy / g, dx / g) == Some(a2)
}

fn asteroid_in_direction(field: Field, start: Pos, dy: i32, dx: i32) -> Option<Pos> {
    let (asteroids, max_y, max_x) = field;
    let (mut y, mut x) = start;
    y += dy;
    x += dx;

    while 0 <= y && y <= max_y && 0 <= x && x <= max_x {
        if asteroids.contains(&(y, x)) {
            return Some((y, x));
        }
        y += dy;
        x += dx;
    }

    None
}

fn nth_destroyed(asteroids: &HashSet<Pos>, station: Pos, n: usize) -> Pos {
    let mut has_at_least = HashMap::new();
    let mut in_dir = HashMap::new();

    let (station_y, station_x) = station;

    for &(y, x) in asteroids {
        let dy = y - station_y;
        let dx = x - station_x;

        let (quadrant, angle) = if dy < 0 && dx >= 0 {
            (0, Rational::new(dx, -dy))
        } else if dy >= 0 && dx > 0 {
            (1, Rational::new(dy, dx))
        } else if dy > 0 && dx <= 0 {
            (2, Rational::new(-dx, dy))
        } else if dy <= 0 && dx < 0 {
            (3, Rational::new(-dy, -dx))
        } else {
            panic!("no quadrant for {} {}", dy, dx);
        };

        let asts_in_dir = in_dir.entry((quadrant, angle)).or_insert_with(Vec::new);
        asts_in_dir.push((y, x));

        *has_at_least
            .entry((asts_in_dir.len(), quadrant))
            .or_insert(0) += 1;
    }

    let mut remain = n;
    let mut round = 1;
    let mut quadrant = 0;
    while has_at_least[&(round, quadrant)] < remain {
        remain -= has_at_least[&(round, quadrant)];
        quadrant += 1;
        if quadrant == 4 {
            quadrant = 0;
            round += 1;
        }
    }

    let candidate_keys = in_dir.iter().filter_map(|((quad, angle), v)| {
        if *quad == quadrant && v.len() >= round {
            Some((*quad, *angle))
        } else {
            None
        }
    });
    let mut candidate_keys: Vec<_> = candidate_keys.collect();
    candidate_keys.sort_unstable();
    let at_angle = in_dir.get_mut(&candidate_keys[remain - 1]).unwrap();

    let manhattan = |&(y, x): &Pos| (station_y - y).abs() + (station_x - x).abs();
    if round == 1 {
        at_angle.drain(..).min_by_key(manhattan).unwrap()
    } else {
        at_angle.sort_unstable_by_key(manhattan);
        at_angle[round - 1]
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let (a, b) = (a.abs(), b.abs());
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Rational {
    n: i32,
    d: i32,
}

impl Rational {
    fn new(n: i32, d: i32) -> Self {
        let g = gcd(n, d);
        Self { n: n / g, d: d / g }
    }
}

impl std::cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.n * other.d).cmp(&(other.n * self.d))
    }
}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut asteroids = HashSet::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, row) in adventofcode::read_input_file().lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '#' {
                continue;
            }

            max_x = std::cmp::max(max_x, x);
            max_y = std::cmp::max(max_y, y);

            asteroids.insert((
                i32::try_from(y).expect("y too big"),
                i32::try_from(x).expect("x too big"),
            ));
        }
    }

    // don't need to try_from + expect since max_* are equal to elements of asteroids.
    let (station, num_observed) = station((&asteroids, max_y as i32, max_x as i32));
    println!("{}", num_observed);

    if asteroids.len() <= TO_DESTROY {
        println!("bad {}", asteroids.len());
        return;
    }

    asteroids.remove(&station);

    let (y, x) = nth_destroyed(&asteroids, station, TO_DESTROY);
    println!("{}", x * 100 + y);
}

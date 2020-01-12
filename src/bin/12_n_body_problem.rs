fn step(poses: &mut [i32], vels: &mut [i32]) {
    use std::cmp::Ordering;

    for (i, p1) in poses.iter().enumerate() {
        for (j, p2) in poses.iter().skip(i + 1).enumerate() {
            match p1.cmp(p2) {
                Ordering::Less => {
                    vels[i] += 1;
                    vels[i + 1 + j] -= 1;
                }
                Ordering::Greater => {
                    vels[i] -= 1;
                    vels[i + 1 + j] += 1;
                }
                Ordering::Equal => (),
            }
        }
    }
    for (pos, vel) in poses.iter_mut().zip(vels.iter()) {
        *pos += vel;
    }
}

fn run1k(poses: &[i32]) -> Vec<(i32, i32)> {
    let mut poses = poses.to_vec();
    let mut vels = vec![0; poses.len()];

    for _ in 0..1000 {
        step(&mut poses, &mut vels);
    }

    poses.into_iter().zip(vels.into_iter()).collect()
}

fn period(poses0: &[i32]) -> u64 {
    let mut poses = poses0.to_vec();
    let mut vels = vec![0; poses.len()];
    step(&mut poses, &mut vels);
    let mut t = 1;

    while vels.iter().any(|&x| x != 0) {
        t += 1;
        step(&mut poses, &mut vels);
    }

    if poses == poses0 {
        t
    } else {
        t * 2
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

fn main() {
    let moons = adventofcode::read_input_lines(adventofcode::numbers);
    let len = moons.get(0).map_or(0, Vec::len);
    if moons.iter().any(|moon| moon.len() != len) {
        panic!("uneven lengths {:?}", moons);
    }
    let moon_dims = (0..len).map(|i| moons.iter().map(|m| m[i]).collect());
    let moon_dims: Vec<Vec<_>> = moon_dims.collect();

    let moons1k: Vec<_> = moon_dims.iter().map(|dim| run1k(dim)).collect();
    let energy = (0..moons.len()).map(|i| {
        let mut pe = 0;
        let mut ke = 0;
        for md in &moons1k {
            let (p, v) = md[i];
            pe += p.abs();
            ke += v.abs();
        }
        pe * ke
    });
    println!("{}", energy.sum::<i32>());

    let period = moon_dims.iter().map(|dim| period(dim)).fold(1, lcm);
    println!("{}", period);
}

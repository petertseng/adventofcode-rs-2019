#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Technique {
    DealWithIncrement(i64),
    Reverse,
    Cut(i64),
}

impl Technique {
    const NOOP: Self = Self::Cut(0);

    fn apply(&self, pos: i64, deck_size: i64) -> i64 {
        match *self {
            Technique::DealWithIncrement(incr) => (pos * incr) % deck_size,
            Technique::Cut(n) => (pos - n).rem_euclid(deck_size),
            Technique::Reverse => deck_size - 1 - pos,
        }
    }

    fn unapply(&self, pos: i64, deck_size: i64) -> i64 {
        match *self {
            Technique::DealWithIncrement(inc) => {
                big_mult(pos, modular_inverse(inc, deck_size), deck_size)
            }
            Technique::Cut(n) => (pos + n) % deck_size,
            Technique::Reverse => deck_size - 1 - pos,
        }
    }
}

fn simplify_at(techs: &mut [Technique], deck_size: i64, i: usize) -> Option<()> {
    use Technique::*;

    let tech1 = techs[i];
    let tech2 = techs[i + 1];

    match (tech1, tech2) {
        (DealWithIncrement(inc1), DealWithIncrement(inc2)) => {
            techs[i] = DealWithIncrement(big_mult(inc1, inc2, deck_size));
            techs[i + 1] = Technique::NOOP;
        }
        (Cut(n), DealWithIncrement(inc)) => {
            techs[i] = DealWithIncrement(inc);
            techs[i + 1] = Cut(big_mult(n, inc, deck_size));
        }
        (Cut(n1), Cut(n2)) => {
            techs[i] = Cut((n1 + n2) % deck_size);
            techs[i + 1] = Technique::NOOP;
        }
        (Reverse, DealWithIncrement(inc)) => {
            techs[i] = DealWithIncrement(deck_size - inc);
            techs[i + 1] = Cut(inc);
        }
        (Reverse, Cut(n)) => {
            techs[i] = Cut(-n);
            techs[i + 1] = Reverse;
        }
        (Reverse, Reverse) => {
            techs[i] = Technique::NOOP;
            techs[i + 1] = Technique::NOOP;
        }
        _ => (),
    };

    Some(())
}

fn simplify(techs: &[Technique], deck_size: i64) -> Vec<Technique> {
    let mut techs = techs.to_vec();
    while has_dup(&techs) {
        for i in 0..(techs.len() - 1) {
            simplify_at(&mut techs, deck_size, i);
        }
        techs.retain(|&t| t != Technique::NOOP)
    }
    techs
}

fn has_dup(techs: &[Technique]) -> bool {
    let mut dwi = false;
    let mut cut = false;
    let mut rev = false;

    for tech in techs {
        let r = match tech {
            Technique::DealWithIncrement(_) => &mut dwi,
            Technique::Cut(_) => &mut cut,
            Technique::Reverse => &mut rev,
        };
        if *r {
            return true;
        }
        *r = true;
    }

    false
}

fn repeat(techs1: &[Technique], deck_size: i64, num_shuffles: u64) -> Vec<Technique> {
    let mut techs = simplify(techs1, deck_size);
    let mut bits = std::collections::HashMap::new();
    let mut power = 1;

    while power <= num_shuffles {
        bits.insert(power, techs.clone());
        power <<= 1;
        techs.extend(techs.clone().iter());
        techs = simplify(&techs, deck_size);
    }

    let relevant_bits = bits.keys().filter(|&k| k & num_shuffles != 0);
    let combined: Vec<_> = relevant_bits.flat_map(|k| bits[k].clone()).collect();
    simplify(&combined, deck_size)
}

fn apply(techs: &[Technique], deck_size: i64, pos: i64) -> i64 {
    techs.iter().fold(pos, |p, &tech| tech.apply(p, deck_size))
}

fn unapply(techs: &[Technique], deck_size: i64, pos: i64) -> i64 {
    let i = techs.iter().rev();
    i.fold(pos, |p, &tech| tech.unapply(p, deck_size))
}

fn big_mult(a: i64, b: i64, m: i64) -> i64 {
    ((i128::from(a) * i128::from(b)).rem_euclid(i128::from(m))) as i64
}

fn modular_inverse(a: i64, modulus: i64) -> i64 {
    let mut t = (0, 1);
    let mut r = (modulus, a);
    while r.1 != 0 {
        let q = r.0 / r.1;
        t = (t.1, t.0 - q * t.1);
        r = (r.1, r.0 - q * r.1);
    }

    if r.0 > 1 {
        panic!("bad {} {} {:?} {:?}", a, modulus, t, r);
    }

    t.0 % modulus
}

fn parse_tech(s: &str) -> Technique {
    let words: Vec<_> = s.split_whitespace().collect();
    match words[..] {
        ["deal", "with", "increment", n] => {
            Technique::DealWithIncrement(n.parse().expect("bad incr"))
        }
        ["deal", "into", "new", "stack"] => Technique::Reverse,
        ["cut", n] => Technique::Cut(n.parse().expect("bad cut")),
        _ => panic!("bad tech {}", s),
    }
}

fn main() {
    let techs = adventofcode::read_input_lines(parse_tech);
    let test = techs.len() < 40;
    let deck_size1 = if test { 10 } else { 10007 };
    let simp = simplify(&techs, deck_size1);

    if test {
        let cards = (0..deck_size1).map(|n| (apply(&simp, deck_size1, n), n));
        let cards: std::collections::HashMap<_, _> = cards.collect();
        for i in 0..deck_size1 {
            if i != 0 {
                print!(" ");
            }
            print!("{}", cards[&i]);
        }
        println!();
        return;
    }

    println!("{}", apply(&simp, deck_size1, 2019));

    #[allow(clippy::unreadable_literal)]
    let deck_size2 = 119315717514047;
    #[allow(clippy::unreadable_literal)]
    let repeated = repeat(&techs, deck_size2, 101741582076661);
    println!("{}", unapply(&repeated, deck_size2, 2020));
}

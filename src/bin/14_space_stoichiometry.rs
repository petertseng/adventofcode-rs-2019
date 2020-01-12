use std::collections::HashMap;

type Recipes<'a, 'b> = HashMap<&'a str, (u64, Vec<(u64, &'b str)>)>;

fn ore_to_make<'a>(
    recipes: &Recipes<'a, 'a>,
    things: &HashMap<&'a str, u64>,
    leftovers: &mut HashMap<&'a str, u64>,
) -> u64 {
    let mut things = things.clone();

    while things.len() != 1 || !things.contains_key("ORE") {
        let mut new_things = HashMap::new();
        new_things.insert("ORE", things.remove("ORE").unwrap_or(0));

        for (thing, amount_needed) in things.iter() {
            let mut amount_needed = *amount_needed;

            if let Some(&leftover) = leftovers.get(thing) {
                let use_leftover = std::cmp::min(leftover, amount_needed);
                amount_needed -= use_leftover;
                leftovers.insert(thing, leftover - use_leftover);
            }

            let (produced, ref inputs) = recipes[thing];
            let times = (amount_needed + produced - 1) / produced;
            if times == 0 {
                continue;
            }

            *leftovers.entry(thing).or_insert(0) += produced * times - amount_needed;
            for (needed, input) in inputs {
                *new_things.entry(input).or_insert(0) += needed * times;
            }
        }

        things = new_things;
    }

    things[&"ORE"]
}

fn bsearch<F>(mut low: u64, mut high: u64, f: F) -> u64
where
    F: Fn(u64) -> bool,
{
    while low <= high {
        let mid = low + (high - low) / 2;
        if f(mid) {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    high
}

fn parse_recipe<'a>(recipes: &mut Recipes<'a, 'a>, line: &'a str) {
    let splits: Vec<_> = line.splitn(2, " => ").collect();
    if splits.len() != 2 {
        panic!("bad recipe {}", line);
    }
    let (n, r) = parse(splits[1]);
    if recipes.contains_key(&r) {
        panic!("already have {}", r);
    }
    recipes.insert(r, (n, splits[0].split(", ").map(parse).collect()));
}

fn parse(s: &str) -> (u64, &str) {
    let splits: Vec<_> = s.split_whitespace().collect();
    if splits.len() != 2 {
        panic!("bad part {}", s);
    }
    let n = splits[0].parse().expect("bad number");
    (n, splits[1])
}

fn main() {
    let s = adventofcode::read_input_file();
    let mut recipes = HashMap::new();

    for l in s.lines() {
        parse_recipe(&mut recipes, l);
    }

    let ore_to_make_fuel = |f: u64| {
        let mut hm = HashMap::new();
        hm.insert("FUEL", f);
        ore_to_make(&recipes, &hm, &mut HashMap::new())
    };

    let ore1 = ore_to_make_fuel(1);
    println!("{}", ore1);

    let trillion = 1_000_000_000_000u64;
    let low = trillion / ore1;
    let too_much = |f: u64| ore_to_make_fuel(f) > trillion;
    let high = std::iter::successors(Some(low), |x| Some(x * 2))
        .find(|&x| too_much(x))
        .unwrap();
    println!("{}", bsearch(low, high, too_much));
}

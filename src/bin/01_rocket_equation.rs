fn fuel(mass: u32) -> Option<u32> {
    if mass > 5 {
        Some(mass / 3 - 2)
    } else {
        None
    }
}

fn fuel_for_fuel(mass: u32) -> u32 {
    std::iter::successors(fuel(mass), |f1| fuel(*f1)).sum()
}

fn main() {
    let masses =
        adventofcode::read_input_lines(|line| line.parse::<u32>().expect("can't parse integer"));

    println!("{}", masses.iter().cloned().filter_map(fuel).sum::<u32>());
    println!("{}", masses.into_iter().map(fuel_for_fuel).sum::<u32>());
}

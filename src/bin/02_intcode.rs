fn run(mem: &[i64], noun: i64, verb: i64) -> i64 {
    let mut ic = adventofcode::intcode::Computer::new(mem);
    ic.set(1, noun);
    ic.set(2, verb);
    ic.cont();
    ic.get(0)
}

fn find_nv(mem: &[i64]) -> (i64, i64) {
    let base = run(mem, 0, 0);
    let delta_noun = run(mem, 1, 0) - base;
    let delta_verb = run(mem, 0, 1) - base;

    #[allow(clippy::unreadable_literal)]
    let target = 19690720;

    if delta_noun > delta_verb {
        let n = (target - base) / delta_noun;
        (n, (target - base - delta_noun * n) / delta_verb)
    } else {
        let v = (target - base) / delta_verb;
        ((target - base - delta_verb * v) / delta_noun, v)
    }
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();
    println!("{}", run(&mem, 12, 2));

    let (noun, verb) = find_nv(&mem);
    println!("{}", noun * 100 + verb);
}

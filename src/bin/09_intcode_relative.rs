fn run(mem: &[i64], input: i64) -> i64 {
    let mut ic = adventofcode::intcode::Computer::new(mem);
    ic.funopt();
    ic.cont_in(input);
    let key = ic.output.pop().expect("didn't output anything");
    if !ic.output.is_empty() {
        panic!("excess outputs {:?}", ic.output);
    }
    key
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    println!("{}", run(&mem, 1));
    println!("{}", run(&mem, 2));
}

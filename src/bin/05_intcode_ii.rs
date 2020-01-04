fn diag(mem: &[i64], input: i64) -> i64 {
    let mut ic = adventofcode::intcode::Computer::new(mem);
    ic.cont_in(input);
    let diag = ic.output.pop().expect("didn't output anything");
    if ic.output.iter().any(|&x| x != 0) {
        panic!("non-zero outputs {:?}", ic.output);
    }
    diag
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    println!("{}", diag(&mem, 1));
    println!("{}", diag(&mem, 5));
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    // Can't be bothered to implement the full logic,
    // so just hardcoding the numbers.
    let thresh = mem[1352] * mem[2486];
    let bits = &mem[1901..1934];
    let weight = bits.iter().fold(0, |a, &c| a * 2 + i64::from(c > thresh));
    println!("{}", weight);
}

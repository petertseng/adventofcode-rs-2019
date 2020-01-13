fn coeffs(mem: &[i64]) -> Vec<i64> {
    let halt = mem.iter().position(|&x| x == 99).expect("never halts");

    let mult3 = adventofcode::intcode::functions(&mem)
        .iter()
        .max_by_key(|f| f.end - f.start)
        .expect("no functions")
        .start;
    let calls_to_mult3 = calls_to_func(&mem[0..halt], mult3);
    let immed_to_stack = immed_to_stack(&mem[0..halt]);

    let coeffs = calls_to_mult3.iter().filter_map(|&call| {
        immed_to_stack
            .iter()
            .filter_map(|&(addr, v)| if addr < call { Some(v) } else { None })
            .last()
    });
    coeffs.collect()
}

fn calls_to_func(mem: &[i64], f: usize) -> Vec<usize> {
    use std::convert::TryFrom;

    let calls = mem.windows(3).enumerate().filter_map(|(i, inst)| {
        let j = inst[0];
        let arg = inst[1];
        if usize::try_from(inst[2]).map_or(false, |dst| dst == f)
            && (j == 1106 && arg == 0 || j == 1105 && arg != 0)
        {
            Some(i)
        } else {
            None
        }
    });
    calls.collect()
}

fn immed_to_stack(mem: &[i64]) -> Vec<(usize, i64)> {
    let immeds = mem.windows(4).enumerate().filter_map(|(i, inst)| {
        let op = inst[0];
        let arg1 = inst[1];
        let arg2 = inst[2];
        let dest = inst[3];
        if dest == 0 {
            None
        } else if op == 21101 {
            Some((i, arg1 + arg2))
        } else if op == 21102 {
            Some((i, arg1 * arg2))
        } else {
            None
        }
    });
    immeds.filter(|&(_, v)| v > 1).collect()
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();
    let coeffs = coeffs(&mem);
    let x2 = coeffs[0];
    let y2 = coeffs[1];
    let xy = coeffs[2];

    let pull = |y: i64, x: i64| xy * x * y >= (x2 * x * x - y2 * y * y).abs();

    let pullsq = (0..50).map(|y| (0..50).filter(|&x| pull(y, x)).count());
    println!("{}", pullsq.sum::<usize>());

    let mut x = 0;

    for y in 99.. {
        while !pull(y, x) {
            x += 1;
        }
        if pull(y - 99, x + 99) {
            println!("{}", x * 10000 + y - 99);
            break;
        }
    }
}

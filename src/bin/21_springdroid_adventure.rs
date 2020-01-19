fn exactly_one<T: std::fmt::Debug>(name: &str, mut it: impl Iterator<Item = T>) -> T {
    let x = it.next();
    if let Some(y) = it.next() {
        let rest: Vec<_> = it.collect();
        panic!("too many {}: {:?}, {:?}, {:?}", name, x.unwrap(), y, rest);
    }
    x.unwrap_or_else(|| panic!("no {}", name))
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    let damages = mem.windows(3).filter_map(|inst| {
        if inst[0] == 4 && inst[2] == 99 {
            usize::try_from(inst[1]).ok()
        } else {
            None
        }
    });

    let mut base = exactly_one("damage location", damages);

    while mem[base] == 0 {
        base += 1
    }

    let mut damage = 0;

    for &len in &[7, 153] {
        if mem[base + len] != 0 {
            panic!("bad {:?}", &mem[base..=(base + len)]);
        }
        for i in 0..len {
            let addr = base + i;
            let bits = usize::try_from(mem[addr]).expect("bad bits");
            if bits == 0 || bits > 255 {
                panic!("bad bits {}", bits);
            }
            let mut remain_bits = bits;
            for j in 0..9 {
                if remain_bits & 1 == 0 {
                    damage += addr * bits * (18 - j);
                }
                remain_bits >>= 1;
            }
        }
        base += len + 1;

        println!("{}", damage);
    }
}

use std::convert::TryFrom;

type Pos = (usize, usize);

fn auto_dust(mem: &[i64], scaffold: &[Pos], width: usize) -> usize {
    let (_, dust_update) = dust(mem);
    let du0 = dust_update.start;

    // Tried doing this w/ exactly_one,
    // but it caused too much indentation,
    // since I either need to do an at_most_one in a filter_map,
    // or a filter_map in a flat_map
    let mut base_addr_candidates = Vec::new();

    for (i, inst) in mem[dust_update].windows(8).enumerate() {
        let i = i + du0;
        let op1 = inst[0];
        let op2 = inst[4];
        if op2 >= 20000 || [op1, op2].iter().any(|op| op % 100 != 1 && op % 100 != 2) {
            continue;
        }
        let dst2 = usize::try_from(inst[7]).unwrap_or(i);
        if dst2 != i + 9 && dst2 != i + 10 {
            continue;
        }
        let dst1 = inst[3];
        if dst1 == 0 {
            continue;
        }
        let a21 = inst[5];
        let a22 = inst[6];
        if a21 != dst1 && a22 != dst1 {
            continue;
        }
        for &(v, mode) in &[(inst[1], (op1 / 100) % 10), (inst[2], (op1 / 1000) % 10)] {
            if mode == 1 && v > 0 {
                if let Ok(uv) = usize::try_from(v) {
                    base_addr_candidates.push(uv);
                }
            }
        }
    }

    let scaffold_base_addr = exactly_one("scaffold base address", base_addr_candidates.iter());

    scaffold
        .iter()
        .enumerate()
        .map(|(i, (y, x))| scaffold_base_addr + x + y * width + x * y + i + 1)
        .sum()
}

fn dust(mem: &[i64]) -> (usize, std::ops::Range<usize>) {
    use adventofcode::intcode::functions;

    let dust = exactly_one(
        "dust location",
        mem.windows(3).filter_map(|inst| {
            if inst[0] == 4 && inst[2] == 99 {
                usize::try_from(inst[1]).ok()
            } else {
                None
            }
        }),
    );

    let dust_update = exactly_one(
        "dust update",
        functions(mem).into_iter().filter(|f| {
            mem[f.clone()].windows(4).any(|inst| {
                let a = inst[0];
                a < 10000
                    && (a % 100 == 1 || a % 100 == 2)
                    && usize::try_from(inst[3]).map_or(false, |d| d == dust)
            })
        }),
    );

    (dust, dust_update)
}

fn read_intcode_map(mem: &[i64]) -> (Vec<Pos>, usize, usize) {
    let (_, dust_update) = dust(mem);

    let mut width = 0;
    for inst in mem[dust_update].windows(4) {
        if inst[0] % 100 != 2 {
            continue;
        }
        let modes = modes(inst[0]);
        if modes == (1, 2) {
            width = inst[1];
            break;
        }
        if modes == (2, 1) {
            width = inst[2];
            break;
        }
    }
    let width = usize::try_from(width).expect("bad width");

    let mut dot = true;
    let mut pos = 0;
    let mut scaffold = std::collections::HashSet::new();
    let mut inter = std::collections::HashMap::new();

    let start = usize::try_from(std::cmp::max(mem[7], mem[8])).expect("bad start");
    let end = usize::try_from(std::cmp::max(mem[11], mem[12])).expect("bad end");

    for len in &mem[start..end] {
        let len = usize::try_from(*len).expect("bad len");
        if dot {
            pos += len;
        } else {
            for _ in 0..len {
                scaffold.insert(pos);
                if scaffold.contains(&(pos - 1)) && scaffold.contains(&(pos - width)) {
                    inter.insert(pos, 1);
                }
                *inter.entry(pos - 1).or_insert(0) += 1;
                *inter.entry(pos - width).or_insert(0) += 1;
                pos += 1;
            }
        }
        dot = !dot;
    }

    let scaffold = scaffold.iter().map(|&x| (x / width, x % width)).collect();
    let aligns = inter.iter().map(|(&pos, &v)| {
        if v == 3 {
            (pos / width) * (pos % width)
        } else {
            0
        }
    });

    (scaffold, width, aligns.sum())
}

fn read_ascii_map(img: &str) -> (Vec<Pos>, usize, usize) {
    let lines: Vec<Vec<_>> = img.lines().map(|l| l.chars().collect()).collect();

    let width = lines.iter().map(Vec::len).max().unwrap_or(0);

    let aligns = lines.windows(3).enumerate().flat_map(|(y, rows)| {
        rows[1].windows(3).enumerate().map(move |(x, w)| {
            let inter = w == ['#', '#', '#'] && rows[0][x + 1] == '#' && rows[2][x + 1] == '#';
            if inter {
                (y + 1) * (x + 1)
            } else {
                0
            }
        })
    });

    let scaffolds = lines.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == '#' || c == '^' {
                Some((y, x))
            } else {
                None
            }
        })
    });

    (scaffolds.collect(), width, aligns.sum())
}

fn modes(op: i64) -> (i64, i64) {
    ((op / 100) % 10, (op / 1000) % 10)
}

fn exactly_one<T: std::fmt::Debug>(name: &str, mut it: impl Iterator<Item = T>) -> T {
    let x = it.next();
    if let Some(y) = it.next() {
        let rest: Vec<_> = it.collect();
        panic!("too many {}: {:?}, {:?}, {:?}", name, x.unwrap(), y, rest);
    }
    x.unwrap_or_else(|| panic!("no {}", name))
}

fn main() {
    let maybe_img = adventofcode::read_input_file();
    let (mem, map) = if maybe_img.contains('#') {
        (None, read_ascii_map(&maybe_img))
    } else {
        let mem = adventofcode::read_input_file_or_intcode();
        let map = read_intcode_map(&mem);
        (Some(mem), map)
    };

    let (_, _, align) = map;
    println!("{}", align);

    if let Some(mem) = mem {
        let (scaffold, width, _) = map;
        println!("{}", auto_dust(&mem, &scaffold, width));
    } else {
        // TODO
        println!("possible");
    }
}

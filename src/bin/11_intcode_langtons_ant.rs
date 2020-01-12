use adventofcode::intcode::Computer;
use std::collections::HashSet;

type Pos = (i32, i32);
type Dir = (i32, i32);

fn turn_right(dir: Dir) -> Dir {
    let (dy, dx) = dir;
    (dx, -dy)
}

fn turn_left(dir: Dir) -> Dir {
    let (dy, dx) = dir;
    (-dx, dy)
}

fn run<F>(origin_white: bool, mut draw: F) -> (HashSet<Pos>, HashSet<Pos>)
where
    F: FnMut(bool) -> Option<Vec<(bool, bool)>>,
{
    let mut whites = HashSet::new();
    let mut visit = HashSet::new();
    let mut pos = (0, 0);
    let mut dir = (-1, 0);

    if origin_white {
        whites.insert(pos);
    }

    while let Some(pairs) = draw(whites.contains(&pos)) {
        for (white, right) in pairs {
            visit.insert(pos);
            if white {
                whites.insert(pos);
            } else {
                whites.remove(&pos);
            }

            dir = (if right { turn_right } else { turn_left })(dir);
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    (whites, visit)
}

fn run_ic(mem: &[i64], origin_white: bool) -> (HashSet<Pos>, HashSet<Pos>) {
    let mut ic = Computer::new(mem);
    run(origin_white, |w| {
        if ic.is_halted() {
            None
        } else {
            ic.cont_in(i64::from(w));
            Some(pairs(&mut ic.output))
        }
    })
}

fn run_precomp(mem: &[i64]) -> (HashSet<Pos>, HashSet<Pos>) {
    let (mut initial, num_rounds, mut state) = understand_ant(mem);
    let cycle_len = state.len();
    let mut used_initial = false;
    let mut i = 0;
    run(false, |w| {
        if !used_initial {
            used_initial = true;
            Some(initial.drain(..).collect())
        } else if i < num_rounds {
            let prev = std::mem::replace(&mut state[i % cycle_len], w);
            i += 1;
            Some(vec![(!w, w != prev)])
        } else {
            None
        }
    })
}

fn understand_ant(mem: &[i64]) -> (Vec<(bool, bool)>, usize, Vec<bool>) {
    let mut ic = Computer::new(mem);
    ic.cont_in(0);
    let initial_pairs = pairs(&mut ic.output);

    let pos = ic.pos();
    ic.cont_in(1);
    while ic.pos() != pos {
        ic.cont_in(1);
    }

    let pairs = pairs(&mut ic.output);
    let (whites, rights): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();

    if whites.iter().any(|&x| x) {
        panic!("doesn't always invert colour: {:?}", whites);
    }

    let halt = mem.iter().position(|&x| x == 99).expect("never halts");
    let cmp = mem[halt - 7];
    if cmp % 100 != 7 && cmp % 100 != 8 {
        panic!("compare isn't a compare: {}", cmp);
    }
    if cmp / 100 != 10 && cmp / 100 != 100 {
        panic!("compare doesn't compare pos to immed: {}", cmp);
    }
    let jmp = mem[halt - 7 + 4];
    if jmp != 1005 && jmp != 1006 {
        panic!("jump isn't a jump: {}", cmp);
    }
    let cmpdst = mem[halt - 7 + 3];
    let jmparg = mem[halt - 7 + 5];
    if cmpdst != jmparg {
        panic!("jump doesn't test comparison result: {} {}", cmpdst, jmparg);
    }
    let jmpdst = mem[halt - 7 + 6];

    use std::convert::TryFrom;

    if usize::try_from(jmpdst).expect("bad jump destination") != pos {
        panic!(
            "jump destination isn't initial position: {} {}",
            jmpdst, pos
        );
    }

    let n = usize::try_from(mem[halt - 7 + if cmp / 100 == 1 { 1 } else { 2 }] - 1).expect("bad n");
    (initial_pairs, n * rights.len(), rights)
}

fn pairs(outputs: &mut Vec<i64>) -> Vec<(bool, bool)> {
    let bit = |i: i64| match i {
        0 => false,
        1 => true,
        _ => panic!("bad {}", i),
    };
    let v: Vec<_> = outputs.drain(0..(outputs.len() & !1)).collect();
    let pairs = (0..(v.len() / 2)).map(|i| (bit(v[i * 2]), bit(v[i * 2 + 1])));
    pairs.collect()
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    let (_, visit) = if true {
        run_precomp(&mem)
    } else {
        run_ic(&mem, false)
    };
    println!("{}", visit.len());

    let (white, _) = run_ic(&mem, true);
    let (ys, xs): (Vec<_>, Vec<_>) = white.iter().cloned().unzip();
    let range = |vs: &[i32]| {
        let min = *vs.iter().min().unwrap_or(&0);
        let max = *vs.iter().max().unwrap_or(&0);
        min..=max
    };
    for y in range(&ys) {
        for x in range(&xs) {
            print!("{}", if white.contains(&(y, x)) { '#' } else { ' ' })
        }
        println!();
    }
}

pub fn each_perm<F, T>(v: &mut [T], mut f: F)
where
    F: FnMut(&[T]),
{
    each_perm_gen(v.len(), v, &mut f);
}

fn each_perm_gen<F, T>(k: usize, v: &mut [T], f: &mut F)
where
    F: FnMut(&[T]),
{
    if k == 1 {
        f(v);
        return;
    }

    each_perm_gen(k - 1, v, f);
    for i in 0..(k - 1) {
        if k % 2 == 0 {
            v.swap(i, k - 1);
        } else {
            v.swap(0, k - 1);
        }
        each_perm_gen(k - 1, v, f);
    }
}

fn const_input(mem: &[i64], phase: i64, input: i64) -> Vec<i64> {
    let mut ic = adventofcode::intcode::Computer::new(mem);
    ic.cont_in(phase);
    while !ic.is_halted() {
        ic.cont_in(input);
    }
    ic.output
}

struct Amp {
    m: Vec<i64>,
    b: Vec<i64>,
}

impl Amp {
    fn new(mem: &[i64], phase: u8) -> Self {
        let b = const_input(mem, i64::from(phase), 0);
        let y = const_input(mem, i64::from(phase), 1);
        let m = y.iter().zip(b.iter()).map(|(y, b)| y - b).collect();
        Self { m, b }
    }
}

const NUM_AMPS: u8 = 5;

fn chain(mem: &[i64], min_phase: u8) -> i64 {
    let mut phases: Vec<_> = (min_phase..(min_phase + NUM_AMPS)).collect();
    let amps: Vec<_> = phases.iter().map(|&phase| Amp::new(mem, phase)).collect();

    let mut lengths = std::collections::HashSet::new();
    for amp in &amps {
        lengths.insert(amp.b.len());
        lengths.insert(amp.m.len());
    }
    if lengths.len() != 1 {
        panic!("Inconsistent lengths: {:?}", lengths);
    }

    let num_rounds = *lengths.iter().next().unwrap();

    let mut max = std::i64::MIN;

    each_perm(&mut phases, |perm| {
        let mut signal = 0;
        for i in 0..num_rounds {
            for phase in perm {
                let amp = &amps[usize::from(phase - min_phase)];
                signal = amp.m[i] * signal + amp.b[i];
            }
        }
        max = std::cmp::max(signal, max);
    });

    max
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    println!("{}", chain(&mem, 0));
    println!("{}", chain(&mem, 5));
}

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Computer<'a> {
    pos: usize,
    relative_base: i64,
    romem: &'a [i64],
    rwmem: HashMap<usize, i64>,
    halt: bool,
    block: bool,
    input: Vec<i64>,
    pub output: Vec<i64>,
}

impl<'a> Computer<'a> {
    pub fn new(mem: &'a [i64]) -> Self {
        Self {
            pos: 0,
            relative_base: 0,
            romem: mem,
            rwmem: HashMap::new(),
            halt: false,
            block: false,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    pub fn get(&self, i: usize) -> i64 {
        self.rwmem.get(&i).cloned().unwrap_or_else(|| {
            if i < self.romem.len() {
                self.romem[i]
            } else {
                0
            }
        })
    }

    pub fn set(&mut self, i: usize, v: i64) {
        self.rwmem.insert(i, v);
    }

    pub fn step(&mut self) {
        let opcode = self.get(self.pos);
        let (num_params, v1, v2, aout) = self.params(opcode);

        let mut jump = None;

        match opcode % 100 {
            1 => self.set(aout, v1 + v2),
            2 => self.set(aout, v1 * v2),
            3 => match self.input.pop() {
                Some(v) => self.set(aout, v),
                None => self.block = true,
            },
            4 => self.output.push(v1),
            5 => {
                if v1 != 0 {
                    jump = Some(v2)
                }
            }
            6 => {
                if v1 == 0 {
                    jump = Some(v2)
                }
            }
            7 => self.set(aout, i64::from(v1 < v2)),
            8 => self.set(aout, i64::from(v1 == v2)),
            9 => self.relative_base += v1,
            99 => self.halt = true,
            _ => panic!("Unknown opcode {}", opcode),
        }

        if !self.block {
            if let Some(j) = jump {
                self.pos = usize::try_from(j).expect("invalid jump target");
            } else {
                self.pos += 1 + usize::from(num_params);
            }
        }
    }

    pub fn cont(&mut self) {
        while !self.halt && !self.block {
            self.step();
        }
    }

    pub fn cont_in(&mut self, input: i64) {
        self.input.push(input);
        self.block = false;
        self.cont();
    }

    fn in_param(&self, offset: usize, mode: i64) -> i64 {
        let v = self.get(self.pos + offset) + i64::from(mode == 2) * self.relative_base;

        if mode == 1 {
            return v;
        }
        if mode != 0 && mode != 2 {
            panic!("unknown read mode {}", mode);
        }

        self.get(usize::try_from(v).expect("invalid read target"))
    }

    fn params(&self, opcode: i64) -> (u8, i64, i64, usize) {
        let (num_inputs, has_output) = match opcode % 100 {
            1 | 2 => (2, true),
            3 => (0, true),
            4 => (1, false),
            5 | 6 => (2, false),
            7 | 8 => (2, true),
            9 => (1, false),
            99 => (0, false),
            _ => panic!("Unknown opcode {}", opcode),
        };
        let num_params = num_inputs + u8::from(has_output);

        let mode_divisor = [100, 1_000, 10_000, 100_000];

        if opcode >= mode_divisor[usize::from(num_params)] {
            panic!("excess modes for {}", opcode);
        }

        let v1 = if num_inputs >= 1 {
            self.in_param(1, (opcode / 100) % 10)
        } else {
            0
        };
        let v2 = if num_inputs >= 2 {
            self.in_param(2, (opcode / 1000) % 10)
        } else {
            0
        };
        let o = if has_output {
            let write_mode = (opcode / mode_divisor[usize::from(num_inputs)]) % 10;

            if write_mode != 0 && write_mode != 2 {
                panic!("unknown write mode {}", write_mode);
            }

            let v = self.get(self.pos + 1 + usize::from(num_inputs))
                + i64::from(write_mode == 2) * self.relative_base;
            usize::try_from(v).expect("invalid write target")
        } else {
            0
        };

        (num_params, v1, v2, o)
    }
}

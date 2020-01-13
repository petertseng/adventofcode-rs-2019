use std::collections::HashMap;

pub fn functions(mem: &[i64]) -> Vec<std::ops::Range<usize>> {
    let mut calls = Vec::new();
    let mut rets = Vec::new();
    for (i, inst) in mem.windows(2).enumerate() {
        let op = inst[0];
        let arg = inst[1];
        if op == 109 && arg > 0 {
            calls.push(i);
        } else if op == 2106 && arg == 0 || op == 2105 && arg != 0 {
            rets.push(i)
        }
    }
    let pair_ret = |ret: usize| {
        calls
            .iter()
            .filter(|&&call| call < ret)
            .max()
            .map(|&m| m..ret)
    };
    rets.into_iter().filter_map(pair_ret).collect()
}

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

    funopt: bool,
    cached_funcalls: HashMap<(usize, i64), i64>,
    inflight_funcalls: HashMap<i64, (usize, i64)>,
    prev_stored_ret_addr: bool,
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

            funopt: false,
            cached_funcalls: HashMap::new(),
            inflight_funcalls: HashMap::new(),
            prev_stored_ret_addr: false,
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

    pub fn funopt(&mut self) {
        self.funopt = true;
    }

    pub fn step(&mut self) {
        let opcode = self.get(self.pos);
        let (num_params, v1, v2, aout) = self.params(opcode);

        let mut just_stored_ret_addr = false;
        let mut jump = None;

        match opcode % 100 {
            1 => {
                self.set(aout, v1 + v2);
                if let Ok(v) = usize::try_from(v1 + v2) {
                    just_stored_ret_addr = v == self.pos + 7;
                }
            }
            2 => {
                self.set(aout, v1 * v2);
                if let Ok(v) = usize::try_from(v1 * v2) {
                    just_stored_ret_addr = v == self.pos + 7;
                }
            }
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
            if let Some(mut j) = jump {
                if self.funopt {
                    j = self.funopt_jumped(j);
                }
                self.pos = usize::try_from(j).expect("invalid jump target");
            } else {
                self.pos += 1 + usize::from(num_params);
            }
        }

        self.prev_stored_ret_addr = just_stored_ret_addr;
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

    fn funopt_jumped(&mut self, jump_target: i64) -> i64 {
        let rb = match usize::try_from(self.relative_base) {
            Ok(rb) => rb,
            _ => return jump_target,
        };

        if jump_target == self.get(rb) {
            if let Some(inflight) = self.inflight_funcalls.remove(&self.relative_base) {
                // RET
                let returned = self.get(rb + 1);
                self.cached_funcalls.insert(inflight, returned);
                return jump_target;
            }
        }

        if self.prev_stored_ret_addr {
            // CALL
            let uj = jump_target as usize;
            let arg = self.get(rb + 1);
            if let Some(&cached_result) = self.cached_funcalls.get(&(uj, arg)) {
                // Cached - hijack jump target
                self.set(rb + 1, cached_result);
                return self.get(rb);
            } else {
                // New - store, do not hijack
                self.inflight_funcalls.insert(self.relative_base, (uj, arg));
            }
        }

        jump_target
    }
}

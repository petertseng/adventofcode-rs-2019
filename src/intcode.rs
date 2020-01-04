use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct Computer<'a> {
    pos: usize,
    romem: &'a [i64],
    rwmem: HashMap<usize, i64>,
    halt: bool,
}

impl<'a> Computer<'a> {
    pub fn new(mem: &'a [i64]) -> Self {
        Self {
            pos: 0,
            romem: mem,
            rwmem: HashMap::new(),
            halt: false,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    pub fn get(&self, i: usize) -> i64 {
        self.rwmem.get(&i).cloned().unwrap_or_else(|| self.romem[i])
    }

    pub fn set(&mut self, i: usize, v: i64) {
        self.rwmem.insert(i, v);
    }

    pub fn step(&mut self) {
        let opcode = self.get(self.pos);
        let (num_params, v1, v2, aout) = self.params(opcode);

        match opcode {
            1 => self.set(aout, v1 + v2),
            2 => self.set(aout, v1 * v2),
            99 => self.halt = true,
            _ => panic!("Unknown opcode {}", opcode),
        }

        self.pos += 1 + usize::from(num_params);
    }

    pub fn cont(&mut self) {
        while !self.halt {
            self.step();
        }
    }

    fn in_param(&self, offset: usize) -> i64 {
        let v = self.get(self.pos + offset);
        self.get(usize::try_from(v).expect("invalid read target"))
    }

    fn params(&self, opcode: i64) -> (u8, i64, i64, usize) {
        let (num_inputs, has_output) = match opcode {
            1 | 2 => (2, true),
            99 => (0, false),
            _ => panic!("Unknown opcode {}", opcode),
        };
        let num_params = num_inputs + u8::from(has_output);

        let v1 = if num_inputs >= 1 { self.in_param(1) } else { 0 };
        let v2 = if num_inputs >= 2 { self.in_param(2) } else { 0 };
        let o = if has_output {
            let v = self.get(self.pos + 1 + usize::from(num_inputs));
            usize::try_from(v).expect("invalid write target")
        } else {
            0
        };

        (num_params, v1, v2, o)
    }
}

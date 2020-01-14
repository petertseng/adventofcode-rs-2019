use std::collections::HashMap;

pub trait Unsigned: Copy + Default + std::fmt::Display + Eq + std::hash::Hash + Ord {
    fn inc(&mut self);
}

macro_rules! unsigned_impl {
    ($u:tt) => {
        impl Unsigned for $u {
            fn inc(&mut self) {
                *self += 1
            }
        }
    };
}

unsigned_impl!(u8);
unsigned_impl!(u16);
unsigned_impl!(u32);
unsigned_impl!(u64);
unsigned_impl!(u128);
unsigned_impl!(usize);

pub struct Monotone<P, T> {
    prio: P,
    qs: HashMap<P, Vec<T>>,
    size: usize,
}

impl<P: Unsigned, T> Monotone<P, T> {
    pub fn new() -> Self {
        Self {
            prio: P::default(),
            qs: HashMap::new(),
            size: 0,
        }
    }

    pub fn singleton(prio: P, elt: T) -> Self {
        let mut qs = HashMap::new();
        qs.insert(prio, vec![elt]);
        Self { prio, qs, size: 1 }
    }

    pub fn insert(&mut self, prio: P, elt: T) {
        if prio < self.prio {
            panic!("non-monotonic add: {} vs {}", prio, self.prio);
        }
        self.qs.entry(prio).or_insert_with(Vec::new).push(elt);
        self.size += 1
    }

    pub fn remove_min(&mut self) -> Option<(P, T)> {
        if self.size == 0 {
            return None;
        }

        while self.qs.get(&self.prio).map_or(true, Vec::is_empty) {
            self.prio.inc();
        }

        self.size -= 1;

        self.qs
            .get_mut(&self.prio)
            .and_then(Vec::pop)
            .map(|t| (self.prio, t))
    }
}

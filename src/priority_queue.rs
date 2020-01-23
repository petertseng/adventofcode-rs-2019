use std::collections::HashMap;

pub struct PriorityQueue<P, T> {
    elts: Vec<T>,
    prio: HashMap<T, P>,
    idx: HashMap<T, usize>
}

impl<P: Copy + Ord, T: Copy + Eq + std::hash::Hash> PriorityQueue<P, T> {
    pub fn new() -> Self {
        Self {
            elts: Vec::new(),
            prio: HashMap::new(),
            idx: HashMap::new(),
        }
    }

    pub fn insert(&mut self, elt: T, prio: P) {
        self.prio.insert(elt, prio);
        let mut i = self.idx.get(&elt).map_or_else(|| self.elts.len(), |&x| x);

        // since self.elts[i] = elt doesn't work if i is out of bounds:
        if i == self.elts.len() {
            self.elts.push(elt);
        }

        while i > 0 && self.prio[&self.elts[(i - 1) / 2]] >= prio {
            self.elts[i] = self.elts[(i - 1) / 2];
            self.idx.insert(self.elts[i], i);
            i = (i - 1) / 2;
        }

        self.elts[i] = elt;
        self.idx.insert(elt, i);
    }

    pub fn remove_min(&mut self) -> Option<T> {
        let last = self.elts.pop()?;
        let removed = *self.elts.get(0).unwrap_or(&last);

        self.prio.remove(&removed).unwrap();
        self.idx.remove(&removed).unwrap();

        if !self.elts.is_empty() {
            self.down(0, last);
        }

        Some(removed)
    }

    fn down(&mut self, mut i: usize, v: T) {
        let orig_prio = self.prio[&v];
        while let Some(l_v) = self.elts.get(2 * i + 1) {
            let mut smallest_i = i;
            let mut smallest_v = v;
            let mut smallest_prio = orig_prio;
            if self.prio[l_v] < smallest_prio {
                smallest_i = 2 * i + 1;
                smallest_v = *l_v;
                smallest_prio = self.prio[l_v];
            }
            if let Some(r_v) = self.elts.get(2 * i + 2) {
                if self.prio[r_v] < smallest_prio {
                    smallest_i = 2 * i + 2;
                    smallest_v = *r_v;
                }
            }
            if smallest_i == i {
                break;
            }
            self.elts[i] = smallest_v;
            self.idx.insert(smallest_v, i);
            i = smallest_i;
        }
        self.elts[i] = v;
        self.idx.insert(v, i);
    }
}

use std::collections::HashMap;

pub type Gen = u32;

pub struct Result<T> {
    pub gen: Gen,
    pub goals: Vec<(T, Gen)>,
    pub prev: HashMap<T, T>,
}

pub fn bfs<T: Copy + Eq + std::hash::Hash, F, G, I>(
    start: T,
    num_goals: usize,
    mut neighbours: F,
    goal: G,
) -> Result<T>
where
    F: FnMut(T) -> I,
    G: Fn(T) -> bool,
    I: IntoIterator<Item = T>,
{
    let mut goals = Vec::new();
    let mut current_gen = vec![start];
    let mut prev = HashMap::new();
    prev.insert(start, start);
    let mut gen = 0;

    'outer: while !current_gen.is_empty() {
        let mut next_gen = Vec::new();
        for cand in current_gen {
            if goal(cand) {
                goals.push((cand, gen));
                if goals.len() >= num_goals {
                    break 'outer;
                }
            }

            for neigh in neighbours(cand) {
                if prev.contains_key(&neigh) {
                    continue;
                }
                next_gen.push(neigh);
                prev.insert(neigh, cand);
            }
        }

        if !next_gen.is_empty() {
            gen += 1;
        }
        current_gen = next_gen;
    }

    prev.remove(&start);

    Result { gen, goals, prev }
}

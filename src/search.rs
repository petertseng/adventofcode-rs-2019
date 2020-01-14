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

pub type Dist = u32;

pub fn astar<T: Copy + Eq + std::hash::Hash, F, G, H, I>(
    start: T,
    neighbours: F,
    heuristic: H,
    goal: G,
) -> Option<Dist>
where
    F: Fn(T) -> I,
    H: Fn(T) -> Dist,
    G: Fn(T) -> bool,
    I: IntoIterator<Item = (Dist, T)>,
{
    use crate::priority_queue::Monotone;
    use std::collections::HashSet;

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut closed = HashSet::new();
    let mut open = Monotone::singleton(heuristic(start), start);
    //let mut prev = HashMap::new();

    while let Some((_, current)) = open.remove_min() {
        if !closed.insert(current) {
            continue;
        }

        if goal(current) {
            // Could use returned priority of the node,
            // but just going to use g_score for uniformity.
            return Some(g_score[&current]);
        }

        for (ndist, neighbour) in neighbours(current) {
            if closed.contains(&neighbour) {
                continue;
            }
            let tentative_g_score = g_score[&current] + ndist;
            if g_score
                .get(&neighbour)
                .map_or(false, |&g| tentative_g_score >= g)
            {
                continue;
            }

            //prev.insert(neighbour, current);
            g_score.insert(neighbour, tentative_g_score);
            open.insert(tentative_g_score + heuristic(neighbour), neighbour)
        }
    }

    None
}

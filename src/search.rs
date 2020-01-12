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

// This is a lot of work just so that T doesn't have to be Ord.
// Wonder if it'll ever matter (use A* on a T that isn't Ord)
struct AStarNode<T> {
    prio: Dist,
    x: T,
}

impl<T> std::cmp::Ord for AStarNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note other vs self here, since BinaryHeap is a max heap,
        // and I want smaller priority values to come out first.
        // Could use std::cmp::Reverse, but don't feel like it.
        (other.prio).cmp(&self.prio)
    }
}

impl<T> std::cmp::PartialOrd for AStarNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::cmp::Eq for AStarNode<T> {}

impl<T> std::cmp::PartialEq for AStarNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.prio == other.prio
    }
}

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
    use std::collections::{BinaryHeap, HashSet};

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut closed = HashSet::new();
    let mut open = BinaryHeap::new();
    open.push(AStarNode {
        prio: heuristic(start),
        x: start,
    });
    //let mut prev = HashMap::new();

    while let Some(AStarNode { x: current, .. }) = open.pop() {
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
            open.push(AStarNode {
                prio: tentative_g_score + heuristic(neighbour),
                x: neighbour,
            })
        }
    }

    None
}

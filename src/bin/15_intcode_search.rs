use adventofcode::search::{bfs, Result};
use std::collections::HashMap;

type Pos = (i32, i32);

fn adj(p: Pos) -> [Pos; 4] {
    let (y, x) = p;
    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
}

fn explore(mem: &[i64]) -> HashMap<Pos, i64> {
    let mut statuses = HashMap::new();
    let mut computers = HashMap::new();
    computers.insert((0, 0), adventofcode::intcode::Computer::new(mem));

    // Since I need to mutably borrow computers + statuses in neighbours,
    // won't be able to use them in goal.
    // So just explore the whole maze w/ const false goal.
    let neigh = |pos: Pos| {
        let adj = adj(pos);
        let neighs = adj.iter().enumerate().filter_map(|(i, &npos)| {
            let status = *statuses.entry(npos).or_insert_with(|| {
                use std::convert::TryFrom;

                let mut ic = computers[&pos].clone();
                ic.cont_in(i64::try_from(i).unwrap() + 1);
                if ic.output.len() != 1 {
                    panic!("bad output {:?}", ic.output);
                }
                let status = ic.output[ic.output.len() - 1];
                ic.output.clear();
                computers.insert(npos, ic);
                status
            });
            if status < 0 || status > 2 {
                panic!("bad status {} at {:?}", status, npos);
            }
            if status != 0 {
                Some(npos)
            } else {
                None
            }
        });
        neighs.collect::<Vec<_>>()
    };

    bfs((0, 0), std::usize::MAX, neigh, |_| false);

    statuses
}

fn search(
    statuses: &HashMap<Pos, i64>,
    start: Pos,
    num_goals: usize,
    goal_status: i64,
) -> Result<Pos> {
    let neigh = |pos: Pos| {
        let adj = adj(pos);
        let can_move = adj.iter().filter(|npos| statuses[npos] != 0);
        can_move.cloned().collect::<Vec<_>>()
    };
    bfs(start, num_goals, neigh, |pos| statuses[&pos] == goal_status)
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    let statuses = explore(&mem);

    let result = search(&statuses, (0, 0), 1, 2);
    println!("{}", result.gen);

    let oxygen = result.goals.get(0).unwrap().0;
    let result = search(&statuses, oxygen, std::usize::MAX, -1);
    println!("{}", result.gen);
}

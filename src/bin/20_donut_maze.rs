use adventofcode::search::{astar, bfs, Dist, Gen};
use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);
type Portal = (char, char);
type Portals = HashMap<(Portal, PortalType), Pos>;
type PortalToPortal = HashMap<Pos, Vec<(Pos, Gen, i8)>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PortalType {
    Inner,
    Outer,
}

impl PortalType {
    fn ddepth(self) -> i8 {
        match self {
            PortalType::Inner => 1,
            PortalType::Outer => -1,
        }
    }
}

impl std::ops::Not for PortalType {
    type Output = PortalType;

    fn not(self) -> Self::Output {
        match self {
            PortalType::Inner => PortalType::Outer,
            PortalType::Outer => PortalType::Inner,
        }
    }
}

fn search(start: Pos, end: Pos, dists: &PortalToPortal, depth_mult: i8) -> Option<Dist> {
    let portal_pairs = i16::try_from((dists.len() - 2) / 2).expect("too many portals");
    let half_up = (portal_pairs + 1) / 2;
    let max_depth = portal_pairs / 2 * half_up;

    let neigh = |(pos, depth): (Pos, i16)| {
        dists[&pos].iter().filter_map(move |&(npos, dist, ddepth)| {
            let ndepth = depth + i16::from(ddepth) * i16::from(depth_mult);
            if 0 <= ndepth && ndepth <= max_depth {
                Some((dist, (npos, ndepth)))
            } else {
                None
            }
        })
    };

    astar((start, 0), neigh, |_| 0, |x| x == (end, 0))
}

fn parse_maze(s: &str) -> (Pos, Pos, PortalToPortal) {
    let (dots, mut portals, portal_entrances) = portals(s);

    let mut outer = |id: Portal| {
        if let Some(d) = portals.get(&(id, PortalType::Inner)) {
            panic!("inner {:?}: {:?}", id, d);
        }
        portals
            .remove(&(id, PortalType::Outer))
            .unwrap_or_else(|| panic!("no outer {:?}", id))
    };

    let start = outer(('A', 'A'));
    let end = outer(('Z', 'Z'));
    let portals = portals;
    let dists = portal_entrances.iter().map(|&pos| {
        let neigh = |(y, x): Pos| {
            [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
                .iter()
                .filter(|npos| dots.contains(npos))
                .cloned()
                .collect::<Vec<_>>()
        };
        let goal = |cand: Pos| cand != pos && portal_entrances.contains(&cand);
        let goals = bfs(pos, std::usize::MAX, neigh, goal).goals;
        (pos, goals.into_iter().map(|(g, d)| (g, d, 0)).collect())
    });
    let mut dists: PortalToPortal = dists.collect();

    for (&(id, ptype), &pos) in &portals {
        let pos_dists = dists.get_mut(&pos).unwrap();
        pos_dists.push((portals[&(id, !ptype)], 1, ptype.ddepth()));
    }

    (start, end, dists)
}

fn portals(s: &str) -> (HashSet<Pos>, Portals, HashSet<Pos>) {
    let mut letters = HashMap::new();
    let mut dots = HashSet::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, row) in s.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '.' {
                dots.insert((y, x));
            }
            if ('A'..='Z').contains(&cell) {
                use std::cmp::max;
                letters.insert((y, x), cell);
                max_y = max(max_y, y);
                max_x = max(max_x, x);
            }
        }
    }

    let (letters, dots, max_x, max_y) = (letters, dots, max_x, max_y);

    let portal = |pos: Pos, letter: char| {
        let (y, x) = pos;
        let mut portal = None;
        let mut trydir = |dot_pos: Pos, other_letter_pos: Pos| {
            if !dots.contains(&dot_pos) {
                return;
            }
            let other_letter = letters[&other_letter_pos];
            let (dy, dx) = dot_pos;
            let id = if dy < y || dx < x {
                (other_letter, letter)
            } else {
                (letter, other_letter)
            };
            let (oy, ox) = other_letter_pos;
            let portal_type = if oy == 0 || oy == max_y || ox == 0 || ox == max_x {
                PortalType::Outer
            } else {
                PortalType::Inner
            };
            if let Some(other_portal) = portal.replace((dot_pos, id, portal_type)) {
                panic!(
                    "too many portals for {:?}: {:?} {:?}",
                    pos, portal, other_portal
                );
            }
        };
        if y > 0 {
            trydir((y - 1, x), (y + 1, x));
            trydir((y + 1, x), (y - 1, x));
        }
        if x > 0 {
            trydir((y, x - 1), (y, x + 1));
            trydir((y, x + 1), (y, x - 1));
        }
        portal
    };

    let mut portals = HashMap::new();
    let mut portal_entrances = HashSet::new();

    // Letters with dots next to them
    for (&pos, &letter) in &letters {
        if let Some((dot_pos, id, ptype)) = portal(pos, letter) {
            if let Some(prev) = portals.insert((id, ptype), dot_pos) {
                panic!(
                    "dup portal for {:?} {:?}: {:?} vs {:?}",
                    id, ptype, dot_pos, prev
                );
            }
            portal_entrances.insert(dot_pos);
        }
    }

    (dots, portals, portal_entrances)
}

fn main() {
    let s = adventofcode::read_input_file();
    let (start, end, dists) = parse_maze(&s);
    let pt = |depth_mult: i8| {
        if let Some(t) = search(start, end, &dists, depth_mult) {
            println!("{}", t);
        } else {
            println!("impossible");
        }
    };
    pt(0);
    pt(1);
}

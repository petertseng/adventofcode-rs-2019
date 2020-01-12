use adventofcode::search::{astar, bfs, Dist};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

type Pos = (usize, usize);
type KeyPos = u8;

#[derive(Clone, Copy, Debug)]
struct Key {
    pos: KeyPos,
    dist: Dist,
    keys: u32,
    doors: u32,
}

fn key_to_key(
    walls: &HashSet<Pos>,
    keys: &HashMap<Pos, u8>,
    doors: &HashMap<Pos, u8>,
    sources: &[Pos],
) -> Vec<HashMap<KeyPos, Key>> {
    let idx: HashMap<_, _> = sources
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, KeyPos::try_from(i).expect("too many keys")))
        .collect();

    let bfs_from = |src: Pos| {
        let have_new_key = |pos: Pos| pos != src && keys.contains_key(&pos);
        let neigh = |pos: Pos| {
            if have_new_key(pos) {
                Vec::new()
            } else {
                let adj = adj(pos);
                adj.iter().filter(|n| !walls.contains(n)).cloned().collect()
            }
        };
        bfs(src, std::usize::MAX, neigh, have_new_key)
    };

    let keys_and_doors = |pos: Pos, prev: &HashMap<Pos, Pos>| {
        let mut ks = 0;
        let mut ds = 0;
        for cur in std::iter::successors(Some(&pos), |cur| prev.get(cur)) {
            if let Some(k) = keys.get(cur) {
                ks |= 1u32 << k;
            } else if let Some(d) = doors.get(cur) {
                ds |= 1u32 << d;
            }
        }
        (ks, ds)
    };

    let k2k = sources.iter().map(|&src| {
        let result = bfs_from(src);
        let keys = result.goals.iter().map(|&(pos, dist)| {
            let (keys, doors) = keys_and_doors(pos, &result.prev);
            let k = Key {
                pos: idx[&pos],
                dist,
                keys,
                doors,
            };
            (idx[&pos], k)
        });
        keys.collect()
    });
    k2k.collect()
}

fn all_pairs(keys_from: &mut [HashMap<KeyPos, Key>]) -> Vec<Vec<Key>> {
    let add_if_better = |kfi: &mut HashMap<KeyPos, Key>, ik: Key, kj: Key, j: KeyPos| {
        let new_dist = ik.dist + kj.dist;
        if kfi.get(&j).map_or(false, |ij| ij.dist <= new_dist) {
            return;
        }
        let k = Key {
            pos: kj.pos,
            dist: new_dist,
            keys: ik.keys | kj.keys,
            doors: ik.doors | kj.doors,
        };
        kfi.insert(j, k);
    };

    let n = KeyPos::try_from(keys_from.len()).expect("too many keys");

    for k in 0..n {
        for i in 0..n {
            let ui = usize::from(i);
            if k == i {
                continue;
            }
            let ik = match keys_from[ui].get(&k) {
                Some(&ik) => ik,
                None => continue,
            };
            for j in 0..n {
                if i == j || k == j {
                    continue;
                }
                let kj = match keys_from[usize::from(k)].get(&j) {
                    Some(&kj) => kj,
                    None => continue,
                };
                add_if_better(&mut keys_from[ui], ik, kj, j);
            }
        }
    }

    let sort_by_dist = |ks: &mut HashMap<KeyPos, Key>| {
        let mut vs: Vec<_> = ks.drain().map(|(_, v)| v).collect();
        vs.sort_unstable_by_key(|k| k.dist);
        vs.reverse();
        vs
    };
    keys_from.iter_mut().map(sort_by_dist).collect()
}

fn all_keys_time(keys_from: &[Vec<Key>], num_keys: usize, robots: &[u8]) -> Option<Dist> {
    if num_keys > 32 {
        panic!("too many keys, {} > 32", num_keys);
    }
    let all_keys = (1 << num_keys) - 1;

    let bits_per_robot = num_bits(keys_from.len());
    if bits_per_robot * robots.len() + num_keys > 64 {
        panic!(
            "too many bits: {} per robot * {} robots + {} keys > 64",
            bits_per_robot,
            robots.len(),
            num_keys
        );
    }
    let robot_mask = (1 << bits_per_robot) - 1;
    let robot_base: Vec<_> = (0..robots.len())
        .map(|i| bits_per_robot * i + num_keys)
        .collect();

    let decode = |robots_and_keys: u64| {
        let keys = (robots_and_keys & all_keys) as u32;
        let base_and_keys_from = robot_base.iter().map(move |&base| {
            let robot = (robots_and_keys >> base) & robot_mask;
            (base, keys_from[robot as usize].iter())
        });
        (keys, base_and_keys_from)
    };

    let start: u64 = robots
        .iter()
        .zip(robot_base.iter())
        .map(|(&bot, &base)| u64::from(bot) << base)
        .sum::<u64>();

    let neigh = |robots_and_keys: u64| {
        let (keys, base_and_keys_from) = decode(robots_and_keys);
        base_and_keys_from.flat_map(move |(base, keys_from)| {
            let other_robots = robots_and_keys & !(robot_mask << base);
            keys_from.filter_map(move |key| {
                if key.keys | keys == keys || key.doors | keys != keys {
                    None
                } else {
                    let s = other_robots | (u64::from(key.pos) << base) | u64::from(key.keys);
                    Some((key.dist, s))
                }
            })
        })
    };

    let heur = |robots_and_keys: u64| {
        let (keys, base_and_keys_from) = decode(robots_and_keys);
        let dists = base_and_keys_from.map(|(_, mut keys_from)| {
            let not_picked_up = keys_from.find(|k| k.keys | keys != keys);
            not_picked_up.map_or(0, |k| k.dist)
        });
        dists.sum()
    };

    astar(start, neigh, heur, |robots_and_keys| {
        robots_and_keys & all_keys == all_keys
    })
}

// Known that this will never be called with y = 0 or x = 0,
// so no need to check for wrap-around via 0 - 1.
fn adj(pos: Pos) -> [Pos; 4] {
    let (y, x) = pos;
    [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
}

fn num_bits(n: usize) -> usize {
    let bytes = std::mem::size_of::<usize>() * 8;
    let zeroes = usize::try_from(n.leading_zeros()).expect("too many leading zeroes");
    bytes - zeroes
}

fn ignore<T>(_: T) {}

fn main() {
    let s = adventofcode::read_input_file();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut robots = Vec::new();
    let mut walls = HashSet::new();

    //let width = s.lines().map(str::len).max().unwrap_or(0);

    for (y, row) in s.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let pos = (y, x);
            match cell {
                '.' => (),
                '#' => ignore(walls.insert(pos)),
                '@' => robots.push(pos),
                'a'..='z' => ignore(keys.insert(pos, cell as u8 - b'a')),
                'A'..='Z' => ignore(doors.insert(pos, cell as u8 - b'A')),
                _ => panic!("unknown {}", cell),
            };
        }
    }

    let (diagonal, orthogonal, can_part_2) = if robots.len() == 1 {
        let (y, x) = robots[0];
        let diagonal = vec![
            (y - 1, x - 1),
            (y - 1, x + 1),
            (y + 1, x - 1),
            (y + 1, x + 1),
        ];
        let orthogonal = adj((y, x));
        let surround_clear = diagonal
            .iter()
            .chain(orthogonal.iter())
            .all(|s| !walls.contains(s));
        (diagonal, orthogonal.to_vec(), surround_clear)
    } else {
        (Vec::new(), Vec::new(), false)
    };

    let pt = |t: Option<Dist>| {
        if let Some(t) = t {
            println!("{}", t);
        } else {
            println!("impossible");
        }
    };

    if can_part_2 {
        let nrobots = u8::try_from(diagonal.len()).expect("too many robots");
        for orth in orthogonal {
            walls.insert(orth);
        }
        let sources: Vec<_> = robots
            .into_iter()
            .chain(diagonal.iter().cloned())
            .chain(keys.keys().cloned())
            .collect();
        let mut k2k = key_to_key(&walls, &keys, &doors, &sources);

        let mut k2k1 = k2k.clone();
        let add_pair = |add_to: &mut [HashMap<KeyPos, Key>], i: KeyPos, j: KeyPos, dist: Dist| {
            let k = Key {
                pos: 0,
                dist,
                keys: 0,
                doors: 0,
            };
            add_to[usize::from(i)].insert(j, Key { pos: j, ..k });
            add_to[usize::from(j)].insert(i, Key { pos: i, ..k });
        };
        for i in 1..=nrobots {
            let adds: Vec<_> = k2k1[usize::from(i)]
                .iter()
                .map(|(&k, &v)| (k, Key { pos: i, ..v }))
                .collect();
            for (k, v) in adds {
                k2k1[usize::from(k)].insert(i, v);
            }
            add_pair(&mut k2k1, 0, i, 2);
            for j in 1..=nrobots {
                use std::cmp::{max, min};

                if i == j {
                    continue;
                }
                let (y1, x1) = diagonal[usize::from(i) - 1];
                let (y2, x2) = diagonal[usize::from(j) - 1];
                let diff = |a: usize, b: usize| max(a, b) - min(a, b);
                let d = diff(y1, y2) + diff(x1, x2);
                add_pair(&mut k2k1, i, j, Dist::try_from(d).expect("too far apart"));
            }
        }

        pt(all_keys_time(&all_pairs(&mut k2k1), keys.len(), &[0]));
        let robots: Vec<_> = (1..=nrobots).collect();
        pt(all_keys_time(&all_pairs(&mut k2k), keys.len(), &robots));
    } else {
        let nrobots = u8::try_from(robots.len()).expect("too many robots");
        let sources: Vec<_> = robots.into_iter().chain(keys.keys().cloned()).collect();
        let mut k2k = key_to_key(&walls, &keys, &doors, &sources);
        let robots: Vec<_> = (0..nrobots).collect();
        pt(all_keys_time(&all_pairs(&mut k2k), keys.len(), &robots));
    }
}

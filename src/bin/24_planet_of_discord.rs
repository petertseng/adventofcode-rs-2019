use std::collections::HashMap;

const SIDE_LEN: u8 = 5;

const BITS_PER_NEIGHBOUR_COUNT: u8 = 2;
const NEIGHBOUR_COUNT_MASK: u64 = (1 << BITS_PER_NEIGHBOUR_COUNT) - 1;

// It seems that 3 is slightly faster than 4 here.
const GROUP_SIZE: u8 = 3;
const BITS_PER_NEIGHBOUR_COUNT_GROUP: u8 = BITS_PER_NEIGHBOUR_COUNT * GROUP_SIZE;
const NEIGHBOUR_COUNT_GROUP_MASK: u64 = (1 << BITS_PER_NEIGHBOUR_COUNT_GROUP) - 1;
const ALIVE_GROUP_MASK: u32 = (1 << GROUP_SIZE) - 1;

type SingleNeighbours = Vec<(DLevel, u8)>;
type NeighbourGroup = (u32, HashMap<u32, Vec<(DLevel, u64)>>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DLevel {
    Out,
    Same,
    In,
}

trait Neighbour {
    fn count_neighbours(&self, level: usize, grid: u32, neigh_count: &mut [u64]);
}

// Using NeighbourGroup doesn't help as much as it does for other languages,
// but does help a little bit.
// To allow easy switching between the two, I left this code intact.
impl Neighbour for Vec<SingleNeighbours> {
    fn count_neighbours(&self, level: usize, grid: u32, neigh_count: &mut [u64]) {
        let mut work = grid;
        let mut pos = 0;
        while work != 0 {
            if work & 1 != 0 {
                for (dlevel, npos) in &self[pos] {
                    let nlevel = match dlevel {
                        DLevel::Out => level,
                        DLevel::Same => level + 1,
                        DLevel::In => level + 2,
                    };
                    let existing_count = neigh_count[nlevel];
                    let npos_shift = BITS_PER_NEIGHBOUR_COUNT * npos;
                    let count_at_npos = (existing_count >> npos_shift) & NEIGHBOUR_COUNT_MASK;
                    if count_at_npos != NEIGHBOUR_COUNT_MASK {
                        neigh_count[nlevel] = existing_count + (1 << npos_shift);
                    }
                }
            }
            work >>= 1;
            pos += 1;
        }
    }
}

impl Neighbour for Vec<NeighbourGroup> {
    fn count_neighbours(&self, level: usize, grid: u32, neigh_count: &mut [u64]) {
        for (mask, neigh) in self {
            let masked = grid & mask;
            for (dlevel, neigh_contribs) in &neigh[&masked] {
                let nlevel = match dlevel {
                    DLevel::Out => level,
                    DLevel::Same => level + 1,
                    DLevel::In => level + 2,
                };
                let existing = neigh_count[nlevel];
                if existing == 0 {
                    neigh_count[nlevel] = *neigh_contribs;
                } else {
                    let a = existing & 0xaaaa_aaaa_aaaa_aaaa;
                    let b = existing & 0x5555_5555_5555_5555;
                    let c = neigh_contribs & 0xaaaa_aaaa_aaaa_aaaa;
                    let d = neigh_contribs & 0x5555_5555_5555_5555;
                    let bd = b & d;
                    let upper_bits = a | c | (bd << 1);
                    let alow = a >> 1;
                    let clow = c >> 1;
                    let lower_bits = (b ^ d) | (alow & clow) | (bd & (alow | clow));
                    neigh_count[nlevel] = upper_bits | lower_bits;
                }
            }
        }
    }
}

fn grow_bugs<N: Neighbour>(grids: &[u32], neigh: &N, group_cache: &[u32]) -> Vec<u32> {
    let mut neigh_count = vec![0u64; grids.len() + 2];

    for (level, grid) in grids.iter().enumerate() {
        neigh.count_neighbours(level, *grid, &mut neigh_count);
    }

    let mut lmin = 0;
    while neigh_count[lmin] == 0 {
        lmin += 1;
    }
    let mut lmax = neigh_count.len() - 1;
    while neigh_count[lmax] == 0 {
        lmax -= 1;
    }

    let mk_new_level = |level: usize| {
        let mut ncs = neigh_count[level];
        let mut current_alive = if level > 0 {
            *grids.get(level - 1).unwrap_or(&0)
        } else {
            0
        };
        let mut pos = 0;
        let mut new_level = 0;
        while ncs != 0 {
            let nc = ncs & NEIGHBOUR_COUNT_GROUP_MASK;
            let now_alive = current_alive & ALIVE_GROUP_MASK;
            let bits = (nc << GROUP_SIZE) | u64::from(now_alive);
            new_level |= group_cache[bits as usize] << pos;
            ncs >>= BITS_PER_NEIGHBOUR_COUNT_GROUP;
            current_alive >>= GROUP_SIZE;
            pos += GROUP_SIZE;
        }
        new_level
    };

    (lmin..=lmax).map(mk_new_level).collect()
}

fn group_neigh_map(neigh_map: &[SingleNeighbours]) -> Vec<NeighbourGroup> {
    let mid_coord = SIDE_LEN / 2;

    let mut groups = HashMap::new();

    for pos in 0..(SIDE_LEN * SIDE_LEN) {
        let (y, x) = (pos / SIDE_LEN, pos % SIDE_LEN);
        let on_vert_edge = y == 0 || y == SIDE_LEN - 1;
        let on_horiz_edge = x == 0 || x == SIDE_LEN - 1;
        let group = if on_vert_edge && on_horiz_edge {
            0
        } else if on_vert_edge {
            1
        } else if on_horiz_edge {
            2
        } else if y == mid_coord || x == mid_coord {
            3
        } else {
            4
        };
        groups.entry(group).or_insert_with(Vec::new).push(pos)
    }

    let groups = groups.values().map(|group| {
        let mask: u32 = group.iter().map(|&x| 1u32 << x).sum();
        let neigh = (0..(1u32 << group.len())).map(|n| {
            let bits = group.iter().enumerate().map(|(i, pos)| {
                let n_bit = (n >> i) & 1;
                n_bit << pos
            });
            let mut neigh_count = HashMap::new();
            for (i, &pos) in group.iter().enumerate() {
                if (n >> i) & 1 == 0 {
                    continue;
                }
                for (dlevel, npos) in &neigh_map[usize::from(pos)] {
                    let shift = npos * BITS_PER_NEIGHBOUR_COUNT;
                    let existing = *neigh_count.get(dlevel).unwrap_or(&0u64);
                    let ncount = (existing >> shift) & NEIGHBOUR_COUNT_MASK;
                    if ncount < NEIGHBOUR_COUNT_MASK {
                        neigh_count.insert(*dlevel, existing + (1 << shift));
                    }
                }
            }
            (bits.sum(), neigh_count.into_iter().collect())
        });
        (mask, neigh.collect())
    });

    groups.collect()
}

fn neigh_map(recursive: bool) -> Vec<SingleNeighbours> {
    let mid_coord = SIDE_LEN / 2;
    let mid = i16::from(mid_coord * SIDE_LEN + mid_coord);

    #[allow(clippy::type_complexity)]
    let dirs: [(i8, i8, Box<dyn Fn(u8) -> (u8, u8)>); 4] = [
        (-1, 0, Box::new(|nx: u8| (SIDE_LEN - 1, nx))),
        (1, 0, Box::new(|nx: u8| (0, nx))),
        (0, -1, Box::new(|ny: u8| (ny, SIDE_LEN - 1))),
        (0, 1, Box::new(|ny: u8| (ny, 0))),
    ];

    let step = |y: u8, x: u8, dy: i8, dx: i8| {
        let ny = i16::from(y) + i16::from(dy);
        let nx = i16::from(x) + i16::from(dx);

        match (u8::try_from(ny), u8::try_from(nx)) {
            (Ok(uny), Ok(unx)) => {
                if uny < SIDE_LEN && unx < SIDE_LEN {
                    Some(uny * SIDE_LEN + unx)
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let neigh_for = |pos: u8| {
        let (y, x) = (pos / SIDE_LEN, pos % SIDE_LEN);

        if recursive {
            let neighs = dirs.iter().flat_map(|(dy, dx, inner_border)| {
                if let Some(npos) = step(y, x, *dy, *dx) {
                    if i16::from(npos) == mid {
                        let ns = (0..SIDE_LEN).map(|n| {
                            let (iny, inx) = inner_border(n);
                            (DLevel::In, iny * SIDE_LEN + inx)
                        });
                        ns.collect()
                    } else {
                        vec![(DLevel::Same, npos)]
                    }
                } else {
                    let npos16 = mid + i16::from(*dy) * i16::from(SIDE_LEN) + i16::from(*dx);
                    vec![(DLevel::Out, u8::try_from(npos16).expect("bad out"))]
                }
            });
            neighs.collect()
        } else {
            dirs.iter()
                .filter_map(|&(dy, dx, _)| step(y, x, dy, dx).map(|x| (DLevel::Same, x)))
                .collect()
        }
    };

    (0..(SIDE_LEN * SIDE_LEN)).map(neigh_for).collect()
}

fn group_cache() -> Vec<u32> {
    let cache_for = |x: u64| {
        let mut ncs = (x >> GROUP_SIZE) & NEIGHBOUR_COUNT_GROUP_MASK;
        let mut current_alive = x & u64::from(ALIVE_GROUP_MASK);
        let mut pos = 0;
        let mut new_level = 0;

        while ncs > 0 {
            let nc = ncs & NEIGHBOUR_COUNT_MASK;
            let now_alive = nc == 1 || nc == 2 && current_alive & 1 == 0;
            if now_alive {
                new_level |= 1 << pos
            }
            ncs >>= BITS_PER_NEIGHBOUR_COUNT;
            current_alive >>= 1;
            pos += 1;
        }

        new_level
    };

    let max = 1 << (GROUP_SIZE * (BITS_PER_NEIGHBOUR_COUNT + 1));
    (0..max).map(cache_for).collect()
}

fn first_repeat<T: Copy + Eq + std::hash::Hash>(it: impl Iterator<Item = T>) -> T {
    let mut seen = std::collections::HashSet::new();
    for x in it {
        if !seen.insert(x) {
            return x;
        }
    }
    panic!("no repeat");
}

fn bio(s: &str) -> u32 {
    s.chars().rev().fold(0, |a, c| match c {
        '#' => a * 2 + 1,
        '.' => a * 2,
        '\n' => a,
        _ => panic!("bad char {}", c),
    })
}

fn main() {
    let s = adventofcode::read_input_file();
    let bio = bio(&s);
    let group_cache = group_cache();

    let neigh1 = neigh_map(false);
    let neigh1 = group_neigh_map(&neigh1);
    let repeat = first_repeat(std::iter::successors(Some(bio), |&x| {
        let grown = grow_bugs(&[x], &neigh1, &group_cache);
        if grown.len() != 1 {
            panic!("expanded beyond 1 level {:?}", grown);
        }
        Some(grown[0])
    }));
    println!("{}", repeat);

    let neigh2 = neigh_map(true);
    let neigh2 = group_neigh_map(&neigh2);
    let iters = if bio == 1_205_552 { 10 } else { 200 };
    let mut grids = vec![bio];
    for _ in 0..iters {
        grids = grow_bugs(&grids, &neigh2, &group_cache);
    }
    let popcount: u32 = grids.into_iter().map(u32::count_ones).sum();
    println!("{}", popcount);
}

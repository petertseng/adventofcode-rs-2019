type Segment = (i32, i32, i32, i32, i32);
type Wire = (Vec<Segment>, Vec<Segment>);

fn intersections(wire1: &Wire, wire2: &Wire) -> (Vec<i32>, Vec<i32>) {
    let (horiz1, vert1) = wire1;
    let (horiz2, vert2) = wire2;

    let mut dists_from_origin = Vec::new();
    let mut dists_on_wire = Vec::new();

    for (horizs, verts) in &[(horiz1, vert2), (horiz2, vert1)] {
        for (y1, x1min, x1max, l1min, l1d) in *horizs {
            for (x2, y2min, y2max, l2min, l2d) in *verts {
                if x2 < x1min || x2 > x1max || y1 < y2min || y1 > y2max || *y1 == 0 && *x2 == 0 {
                    continue;
                }

                dists_from_origin.push(y1.abs() + x2.abs());

                let l1 = l1min + l1d * (x2 - x1min);
                let l2 = l2min + l2d * (y1 - y2min);

                dists_on_wire.push(l1 + l2);
            }
        }
    }

    (dists_from_origin, dists_on_wire)
}

fn parse_wire(s: &str) -> Wire {
    let mut horiz = Vec::new();
    let mut vert = Vec::new();
    let mut y = 0;
    let mut x = 0;
    let mut total_length = 0;

    for seg in s.split(',') {
        let dir = seg.chars().next().unwrap();
        let length: i32 = seg[1..].parse().expect("can't parse integer");

        let old_y = y;
        let old_x = x;
        let old_length = total_length;

        total_length += length;

        match dir {
            'U' => {
                y -= length;
                vert.push((x, y, old_y, total_length, -1));
            }
            'D' => {
                y += length;
                vert.push((x, old_y, y, old_length, 1));
            }
            'L' => {
                x -= length;
                horiz.push((y, x, old_x, total_length, -1));
            }
            'R' => {
                x += length;
                horiz.push((y, old_x, x, old_length, 1));
            }
            _ => panic!("Unknown direction {} of {}", dir, seg),
        }
    }
    (horiz, vert)
}

fn main() {
    let wires = adventofcode::read_input_lines(parse_wire);

    if wires.len() != 2 {
        panic!("Expected two wires not {}", wires.len());
    }

    let (dists_from_origin, dists_on_wire) = intersections(&wires[0], &wires[1]);

    println!("{}", dists_from_origin.iter().min().unwrap());
    println!("{}", dists_on_wire.iter().min().unwrap());
}

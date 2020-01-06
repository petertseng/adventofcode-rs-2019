const WIDTH: usize = 25;
const LAYER: usize = WIDTH * 6;

fn pixel(img: &[char], mut i: usize) -> char {
    loop {
        if img[i] != '2' {
            return img[i];
        }
        i += LAYER;
    }
}

fn count<T: Eq>(xs: &[T], x: &T) -> usize {
    xs.iter().filter(|&c| c == x).count()
}

fn main() {
    let layers: Vec<_> = adventofcode::read_input_file().trim().chars().collect();

    let min_layer = layers
        .chunks(LAYER)
        .min_by_key(|layer| count(layer, &'0'))
        .unwrap_or(&[]);
    let ones = count(min_layer, &'1');
    let twos = count(min_layer, &'2');
    println!("{}", ones * twos);

    let img: Vec<_> = (0..LAYER).map(|i| pixel(&layers, i)).collect();

    for row in img.chunks(WIDTH) {
        let s = row.iter().map(|&c| match c {
            '1' => '#',
            '0' => ' ',
            _ => panic!("unknown char {}", c),
        });
        println!("{}", s.collect::<String>());
    }
}

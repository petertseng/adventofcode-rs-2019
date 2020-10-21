fn fft(digits: &mut [i16]) {
    let mut sum = 0;
    let sum_left = digits.iter().map(|d| {
        let new_sum = sum + d;
        std::mem::replace(&mut sum, new_sum)
    });
    let sum_left: Vec<_> = sum_left.collect();

    for i in 0..digits.len() {
        let n = i + 1;
        let mut sign = 1;
        let mut base = i;
        let mut total = 0;
        while base < digits.len() {
            total += (sum_left.get(base + n).unwrap_or(&sum) - sum_left[base]) * sign;
            base += n * 2;
            sign *= -1;
        }
        digits[i] = total.abs() % 10;
    }
}

fn binom_99_mod_10(i: usize) -> usize {
    let b2 = if (i + 99) & 99 == 99 { 5 } else { 0 };
    let b5 = match i % 125 {
        0 => 6,
        25 => 4,
        _ => 0,
    };
    (b2 + b5) % 10
}

fn main() {
    let s = adventofcode::read_input_file();

    let input = s.trim().bytes().map(|digit| {
        if !(b'0'..=b'9').contains(&digit) {
            panic!("bad digit {}", digit);
        }
        digit - b'0'
    });
    let input: Vec<_> = input.collect();

    let mut digits: Vec<_> = input.iter().cloned().map(i16::from).collect();
    for _ in 0..100 {
        fft(&mut digits);
    }

    for i in &digits[0..8] {
        print!("{}", i);
    }
    println!();

    let offset = input[0..7]
        .iter()
        .fold(0, |acc, d| acc * 10 + usize::from(*d));

    let l10k = input.len() * 10000;
    if offset * 2 < l10k {
        panic!("Can't offset {} w/ len {}", offset, l10k);
    }

    use std::cmp::min;

    let len = l10k - min(l10k, offset);
    let rsize = 8;
    let mut r = vec![0; rsize];

    for i in 0..len {
        let bin = binom_99_mod_10(i);
        if bin == 0 {
            continue;
        }

        let dist_from_end = len - i;
        for j in 0..(min(dist_from_end, rsize)) {
            r[j] += usize::from(input[(offset + i + j) % input.len()]) * bin;
        }
    }

    for i in &r[0..8] {
        print!("{}", i % 10);
    }
    println!();
}

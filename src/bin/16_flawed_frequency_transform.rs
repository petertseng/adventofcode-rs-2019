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

fn binom(mut n: usize, mut k: usize) -> usize {
    let mut num = 1;
    let mut denom = 1;
    while k > 0 {
        num *= n;
        denom *= k;
        n -= 1;
        k -= 1;
    }
    num / denom
}

fn binom_mod(mut n: usize, mut k: usize, m: usize) -> usize {
    let mut r = 1;
    while k > 0 && r > 0 {
        r *= binom(n % m, k % m) % m;
        n /= m;
        k /= m;
    }
    r
}

fn binom_mod_10(n: usize, k: usize) -> usize {
    //let b2 = usize::from(n & k == k);
    //let b5 = binom_mod(n, k, 5);
    //[[0, 6, 2, 8, 4], [5, 1, 7, 3, 9]][b2][b5]
    let b2 = if n & k == k { 5 } else { 0 };
    let b5 = 6 * binom_mod(n, k, 5);
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
        let bin = binom_mod_10(99 + i, i);
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

fn count(min: u32, max: u32) -> (usize, usize) {
    let max_digits = digits(max);
    let mut digits = digits(min);

    for i in 1..digits.len() {
        if digits[i - 1] > digits[i] {
            for j in i..digits.len() {
                digits[j] = digits[j - 1];
            }
            break;
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    while digits.iter().le(max_digits.iter()) {
        let mut count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for &i in digits.iter() {
            count[usize::from(i)] += 1;
        }
        if count.iter().any(|&x| x >= 2) {
            p1 += 1;
        }
        if count.contains(&2) {
            p2 += 1;
        }

        if !add_one(&mut digits) {
            break;
        }
    }

    (p1, p2)
}

fn digits(mut n: u32) -> Vec<u8> {
    let mut v = Vec::new();
    while n > 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }
    v.reverse();
    v
}

fn add_one(digits: &mut [u8]) -> bool {
    let mut non_nine = digits.len() - 1;
    while digits[non_nine] == 9 {
        if non_nine == 0 {
            return false;
        }
        non_nine -= 1;
    }

    let new_digit = digits[non_nine] + 1;
    for d in digits.iter_mut().skip(non_nine) {
        *d = new_digit;
    }

    true
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let s = if args.len() == 1 {
        adventofcode::read_input_file()
    } else if args.len() == 2 {
        if args[1].contains('-') {
            args[1].clone()
        } else {
            adventofcode::read_input_file()
        }
    } else {
        format!("{}-{}", args[1], args[2])
    };
    let nums: Vec<u32> = s
        .split('-')
        .map(|i| i.parse().expect("can't parse integer"))
        .collect();

    let (p1, p2) = count(nums[0], nums[1]);
    println!("{}", p1);
    println!("{}", p2);
}

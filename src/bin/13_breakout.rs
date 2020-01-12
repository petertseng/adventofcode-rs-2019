#[allow(clippy::many_single_char_names)]
fn breakout(mem: &[i64]) -> (usize, i64) {
    let (board, a, b, m) = score_consts(mem);

    let game_grid = &mem[board..(board + m)];
    let non_one = game_grid.iter().position(|&c| c != 1).expect("all walls");
    let width = non_one - 1;
    let height = m / width;
    let mut num_blocks = 0;
    let scores = &mem[(board + m)..(board + m * 2)];

    let score = game_grid.iter().enumerate().map(|(i, &c)| {
        if c == 2 {
            // could just do a count() on this,
            // but only want to iterate it once
            num_blocks += 1;
            let (y, x) = (i / width, i % width);
            let i = (a * (x * height + y) + b) % scores.len();
            scores[i]
        } else {
            0
        }
    });
    let score = score.sum();

    (num_blocks, score)
}

fn score_consts(mem: &[i64]) -> (usize, usize, usize, usize) {
    use std::convert::TryFrom;

    let funcs = adventofcode::intcode::functions(&mem);
    let last = funcs[funcs.len() - 1].clone();
    let board = last.end + 3;
    let nums = mem[last].windows(4).filter_map(|inst| {
        if inst[0] == 21101 {
            Some((inst[3], inst[1] + inst[2]))
        } else if inst[0] == 21102 {
            Some((inst[3], inst[1] * inst[2]))
        } else {
            None
        }
    });
    let nums: std::collections::HashMap<_, _> = nums.collect();

    let a = usize::try_from(nums[&2]).expect("bad a");
    let b = usize::try_from(nums[&3]).expect("bad b");
    let m = usize::try_from(nums[&4]).expect("bad m");

    (board, a, b, m)
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    let (blocks, score) = breakout(&mem);
    println!("{}", blocks);
    println!("{}", score);
}

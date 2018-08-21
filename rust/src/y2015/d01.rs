use crate::{Date, Day, Puzzle, Result};
use failure::bail;

const DATE: Date = Date::new(Day::D01, super::YEAR);
pub(super) const PUZZLE: Puzzle = Puzzle::new(DATE, solve);

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn solve(input: String) -> Result {
    let mut moves = 0;
    let mut floor = 0;
    let mut visited_basement = false;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => bail!("unexpected character '{}'", c),
        }
        if !visited_basement {
            moves += 1;
            if floor == -1 {
                visited_basement = true;
            }
        }
    }
    answer!(floor, moves);
}

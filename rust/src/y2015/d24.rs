use crate::{Date, Day, Puzzle, Result};

const DATE: Date = Date::new(Day::D24, super::YEAR);
pub(super) const PUZZLE: Puzzle = Puzzle::new(DATE, solve);

#[allow(clippy::missing_const_for_fn)]
#[allow(clippy::needless_pass_by_value)]
fn solve(_input: String) -> Result {
    answer!();
}

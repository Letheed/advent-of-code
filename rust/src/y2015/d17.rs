use std::cmp::Ordering;

use crate::{Date, Day, Puzzle, Result};

const DATE: Date = Date::new(Day::D17, super::YEAR);
pub(super) const PUZZLE: Puzzle = Puzzle::new(DATE, solve);

#[allow(clippy::needless_pass_by_value)]
fn solve(input: String) -> Result {
    let mut jugs = parse_jugs(&input)?;
    let wanted_volume = jugs.swap_remove(0);
    jugs.sort_unstable();
    let mut combinations = 0;
    let mut combinations_of_fewest_jugs = 0;
    let mut fewest_jugs = u8::max_value();
    let mut volumes = vec![Volume::default()];
    let mut new_volumes = Vec::new();
    for jug in jugs {
        for &volume in &volumes {
            let new_total = volume.total + jug;
            match new_total.cmp(&wanted_volume) {
                Ordering::Equal => {
                    combinations += 1;
                    let jugs = volume.jugs + 1;
                    match jugs.cmp(&fewest_jugs) {
                        Ordering::Equal => combinations_of_fewest_jugs += 1,
                        Ordering::Less => {
                            fewest_jugs = jugs;
                            combinations_of_fewest_jugs = 1;
                        }
                        Ordering::Greater => {}
                    }
                    new_volumes.push(volume);
                }
                Ordering::Less => {
                    let mut new_volume = volume;
                    new_volume.jugs += 1;
                    new_volume.total = new_total;
                    new_volumes.push(volume);
                    new_volumes.push(new_volume);
                }
                Ordering::Greater => {}
            }
        }
        std::mem::swap(&mut volumes, &mut new_volumes);
        new_volumes.clear();
    }
    answer!(combinations, combinations_of_fewest_jugs);
}

type Jug = u8;

#[derive(Copy, Clone, Default)]
struct Volume {
    jugs: u8,
    total: Jug,
}

fn parse_jugs(s: &str) -> Result<Vec<Jug>> {
    Ok(s.lines().map(str::parse).collect::<std::result::Result<_, _>>()?)
}

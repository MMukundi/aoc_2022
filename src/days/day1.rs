use std::str::FromStr;

use crate::solution::{AOCSolution, Labeled};

type Calorie = u32;
pub struct ElfCalories {
    calories: Vec<Calorie>
}
impl FromStr for ElfCalories{
    type Err=<Calorie as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories = s.split("\n\n").map(|elf_section|elf_section.lines().map(Calorie::from_str).try_fold(0 as Calorie, |total,new|{
            new.map(|new|new+total)
        })).collect::<Result<Vec<_>,_>>()?;
        calories.sort_by_cached_key(|c|std::cmp::Reverse(*c));
        Ok(Self {
            calories
        })
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=ElfCalories;
    type Part1=Labeled<Calorie>;
    type Part2=Labeled<Calorie>;
    type Err = ();
    fn solve(calories:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        Ok((
            (calories.calories[0],"calorie").into(),
            (calories.calories.iter().take(3).sum(),"calorie").into(),
        ))
    }
}
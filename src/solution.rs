use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    str::FromStr,
};

pub trait AOCSolution {
    type Input: FromStr;
    type Part1: Display;
    type Part2: Display;
    type Err;
    fn solve(input: Self::Input) -> Result<(Self::Part1, Self::Part2), Self::Err>;
}

#[derive(Debug)]
pub struct Unsolved;
impl Display for Unsolved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unsolved")
    }
}
// pub fn solve<S:AOCSolution>(input_str:&str)->Result<(S::Part1,S::Part2),<S::Input as FromStr>::Err> {
//     let input: S::Input = input_str.parse()?;
//     Ok(S::solve(input))
// }
pub fn get_path(day_index: usize) -> Option<PathBuf> {
    let mut root = PathBuf::from(file!()).parent()?.parent()?.to_owned();
    root.push("inputs");
    root.push(format!("day{day_index}.txt"));
    Some(root)
}
pub fn solution_string<S: AOCSolution>(day_index: usize) -> String
where
    <S::Input as FromStr>::Err: Debug,
    S::Err: Debug,
{
    let input_path = match get_path(day_index) {
        Some(p) => p,
        _ => return "Cannot find inputs folder".into(),
    };
    let input_str = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => return format!("Error reading input for day {day_index}: {e:?}"),
    };
    let input: S::Input = match input_str.parse() {
        Ok(i) => i,
        Err(e) => return format!("Error parsing input for day {day_index}: {e:?}"),
    };
    let (part_1, part_2) = match S::solve(input) {
        Ok(s) => s,
        Err(e) => return format!("Error solving day {day_index}: {e:?}"),
    };
    format!("Day {day_index}\n----------\n - Part 1: {part_1}\n - Part 2: {part_2}")
}
pub struct Labeled<T> {
    value: T,
    label: String,
}
impl<T, S: Display> From<(T, S)> for Labeled<T> {
    fn from((value, label): (T, S)) -> Self {
        Self {
            value,
            label: label.to_string(),
        }
    }
}
impl<T: Display> Display for Labeled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}(s)", self.value, self.label)
    }
}

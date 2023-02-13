use std::{str::FromStr, fmt::{Display, Debug}};

#[derive(Debug,Clone)]
pub struct SNAFU(Vec<i8>);
impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().rev().filter_map(|c|match c{
            2=>Some('2'),
            1=>Some('1'),
            0=>Some('0'),
            -1=>Some('-'),
            -2=>Some('='),
            _=>None
        }).collect::<String>())
    }
}
impl From<&SNAFU> for isize {
    fn from(value: &SNAFU) -> Self {
        value.0.iter().zip(std::iter::successors(Some(1_isize), |&prev|prev.checked_mul(5))).map(|(&digit,power)|(digit as isize)*power).sum()
    }
}
impl From<isize> for SNAFU {
    fn from(value: isize) -> Self {
        let mut digits = vec![];
        let mut remainder = value;

        while remainder != 0 {
            let mut digit = (remainder % 5) as i8;
            remainder = remainder/5;
            if digit > 2 {
                remainder += 1;
                digit -= 5;
            }
            digits.push((digit))
        }

        Self(digits)
    }
}



use crate::solution::{Unsolved, AOCSolution};

#[derive(Debug,Clone)]
pub struct InputStruct {
    numbers:Vec<SNAFU>
}
impl FromStr for InputStruct {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{numbers:s.lines().map(|line|{
            SNAFU(line.chars().rev().filter_map(|c|match c{
                '2'=>Some(2),
                '1'=>Some(1),
                '0'=>Some(0),
                '-'=>Some(-1),
                '='=>Some(-2),
                _=>None
            }).collect::<Vec<i8>>())
        }).collect()})
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=InputStruct;
    type Part1=SNAFU;
    type Part2=Unsolved;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let sum = input.numbers.iter().map(isize::from).sum::<isize>();
        let sum = sum.into();
        Ok((sum,Unsolved))
    }
}

use std::fmt::Debug;

use std::str::FromStr;


use crate::solution::{AOCSolution};


#[derive(Debug,Clone)]
pub struct Signal(Vec<char>);
impl FromStr for Signal {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Signal;
    type Part1=usize;
    type Part2=usize;
    type Err = ();
    fn solve(signal:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let a= signal.0.windows(4).position(|chars|{
            let mut iter = chars.iter();
            chars.iter().all(|c|{
                iter.next();
                !iter.clone().any(|i|i==c)
            })
        });
        let b = signal.0.windows(14).position(|chars|{
            let mut iter = chars.iter();
            chars.iter().all(|c|{
                iter.next();
                !iter.clone().any(|i|i==c)
            })
        });
        Ok((a.unwrap()+4,b.unwrap()+14))
    }
}
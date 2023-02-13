use std::convert::Infallible;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::str::FromStr;


use crate::solution::{AOCSolution, Labeled};


#[derive(Debug,Clone)]
pub struct ElfCleaningRanges(Vec<(RangeInclusive<u8>,RangeInclusive<u8>)>);
impl FromStr for ElfCleaningRanges {
    type Err=Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.lines().filter_map(|l|{
            l.split_once(',')
        }).filter_map(|(l,r)|{
            l.split_once('-').zip(r.split_once('-'))
        }).filter_map(|((a,b),(c,d))|{
            if let (Ok(a),Ok(b),Ok(c),Ok(d)) = (a.parse::<u8>(),b.parse::<u8>(),c.parse::<u8>(),d.parse::<u8>()) {
                Some((a..=b,c..=d))
            }else{
                None
            }
        });
        Ok(Self(parsed.collect()))
    }
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=ElfCleaningRanges;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(ranges:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let contained_ranges = ranges.0.iter().filter(|(a,b)|{
            (a.start() >= b.start() && a.end() <= b.end())||
            (b.start() >= a.start() && b.end() <= a.end()) 
        }).count();
        let overlapping_ranges = ranges.0.iter().filter(|(a,b)|{
            a.contains(b.start()) ||a.contains(b.end())||
            b.contains(a.start()) ||b.contains(a.end())
        }).count();
        Ok((
            (contained_ranges,"contained range").into(),
            (overlapping_ranges,"overlapping range").into(),
        ))
    }
}
//12156
#[cfg(test)]
mod test {
    #[test]
    fn test(){

    }
}
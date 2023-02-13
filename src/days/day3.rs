use std::fmt::Debug;
use std::str::FromStr;


use crate::solution::{AOCSolution, Labeled};
use crate::bitset::{BitSet, DefaultedBytes};

pub struct RSChar(u8);
impl RSChar {
    pub fn priority(&self)->u8 {
        if (b'a'..=b'z').contains(&self.0) {
            1+self.0-b'a'
        }else {
            27+self.0-b'A'
        }
    }
}
impl From<RSChar> for usize {
    fn from(rs_char:RSChar) -> usize {
        (rs_char.0 - b'A') as usize
    }
}
impl From<RSChar> for char {
    fn from(rs_char:RSChar) -> char {
        rs_char.0 as char
    }
}
impl Debug for RSChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RSChar").field(&(self.0 as char)).finish()
    }
}
impl TryFrom<usize> for RSChar {
    type Error=usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value<=(b'z'-b'A') as usize {
            Ok(Self(value as u8+b'A'))            
        } else{
            Err(value)
        }
    }
}
impl TryFrom<char> for RSChar {
    type Error=char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !('A'..='z').contains(&value) {
            Err(value)
        } else{
            Ok(Self(value as u8))            
        }
    }
}


type Chars = BitSet<RSChar,DefaultedBytes<10>>;

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Rucksack(Chars);

#[derive(Debug,Clone)]
pub struct Rucksacks(Vec<(Rucksack,Rucksack)>);
impl FromStr for Rucksacks {
    type Err=String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.lines().map(|l|{
            let len = l.len();
            l.split_at(len/2)
        }).map(|(l,r)|{
            let [mut lb,mut rb] = <[Chars;2]>::default();
            lb.extend(l.chars().map(|c|RSChar::try_from(c).unwrap()));
            rb.extend(r.chars().map(|c|RSChar::try_from(c).unwrap()));
            (Rucksack(lb),Rucksack(rb))
        });
        Ok(Self(parsed.collect()))
    }
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=Rucksacks;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(rucksacks:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let priority_sum = rucksacks.0.iter().map(|(l,r)|{
            (l.0.clone()&r.0.clone()).iter().map(|b|{
                b.unwrap().priority() as usize
            }).sum::<usize>()
        }).sum();
        // let mut trio_sum = 0;
        // let trios = ArrayWindows::<3,_>::new(rucksacks.0.into_iter()); 
        // for sacks in trios.step_by(3) {
        //     trio_sum +=sacks.into_iter()
        //     .map(|(a,b)| a.0|b.0)
        //     .reduce(|a,b|a&b)
        //     .and_then(|badge|badge.into_iter().next().and_then(|c|c.ok().as_ref().map(RSChar::priority))).unwrap_or_default() as usize;
        // }
        let trio_sum = rucksacks.0.into_iter().collect::<Vec<_>>().chunks_exact(3).map(|trio|{
            let mut trio = trio.iter().cloned();
            let Some((a,b)) = trio.next() else {return 0};
            let Some((c,d)) = trio.next() else {return 0};
            let Some((e,f)) = trio.next() else {return 0};
            if let Some(Ok(c)) = ((a.0|b.0)&(c.0|d.0)&(e.0|f.0)).into_iter().next() {
                c.priority() as usize
            }else{
                0
            } 
        }).sum::<usize>();

        Ok(((priority_sum,"priority point").into(),(trio_sum,"priority point").into()))
        // dbg!(&rucksacks.0[0].0.0);
        // dbg!(RSChar(b'P'));
        // dbg!(rucksacks.0[0].0.0.contains(RSChar(b'P')));
        // dbg!(rucksacks.0[0].0.0.contains(RSChar(b'D')));
        // dbg!(rucksacks.0[0].0.0.iter().map(|c|c.map(char::from)).collect::<Result<String,_>>());
        // dbg!(rucksacks.0[0].1.0.iter().map(|c|c.map(char::from)).collect::<Result<String,_>>());

        // let score_if_second_is_throws = instructions.0.iter().map(|(a,b)|
        //     b.as_throw().score_against(&a.as_throw())
        // ).sum();
        // let real_score = instructions.0.iter().map(|(throw,outcome)|{
        //     let throw=throw.as_throw();
        //     let outcome=outcome.as_outcome();
        //     throw.throw_for(&outcome.flip()).throw_score()+outcome.score()
        // }).sum();
        // Ok((
        //     (score_if_second_is_throws,"points").into(),
        //     (real_score,"points").into(),
        // ))
    }
}
//12156
#[cfg(test)]
mod test {
    use super::RSChar;

    #[test]
    fn prior(){
        // for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        //     dbg!(RSChar::try_from(c).as_ref().map(|c|{
        //         (c,c.priority())
        //     }));
        // }
    }
    #[test]
    fn test(){
        // let s = "ABCDEFGHIJKLMNOPQRSijklmnopqrstuvwxyz";
        let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        dbg!(b'A');
        dbg!(b'Z');
        dbg!(b'a');
        dbg!(b'z');
        let mut bs = super::Chars::default();
        bs.extend(s.chars().map(|c|RSChar::try_from(c).unwrap()));
        dbg!(&bs);
        assert!(bs.iter().map(|c|c.map(char::from)).collect::<Result<String,_>>().is_ok());
    }
}
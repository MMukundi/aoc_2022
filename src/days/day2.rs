use std::str::FromStr;

use crate::solution::{AOCSolution, Labeled};

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum Throw {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}
impl Outcome {
    pub fn flip(&self)->Self {
        match self {
            Self::Draw => Self::Draw,
            Self::Lose => Self::Win,
            Self::Win => Self::Lose
        }
    }

    pub fn score(&self)->usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6
        }
    }
}

impl Throw {
    pub fn throw_for(&self,outcome:&Outcome)->Self {
        match (self,outcome) {
            (Throw::Scissors, Outcome::Draw)|
            (Throw::Rock, Outcome::Win)|
            (Throw::Paper, Outcome::Lose) => Throw::Scissors,
          
            (Throw::Paper, Outcome::Draw) |
            (Throw::Scissors, Outcome::Win) |
            (Throw::Rock, Outcome::Lose) => Throw::Paper,
   
            (Throw::Rock, Outcome::Draw) |
            (Throw::Scissors, Outcome::Lose) |
            (Throw::Paper, Outcome::Win) => Throw::Rock,
        }
    }
    pub fn throw_score(&self)->usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn outcome_against(&self,other:&Self)->Outcome {
        match (self,other) {
            (Self::Rock,Self::Paper) |
            (Self::Paper,Self::Scissors) |
            (Self::Scissors,Self::Rock) => Outcome::Lose,

            (Self::Rock,Self::Rock) |
            (Self::Paper,Self::Paper) |
            (Self::Scissors,Self::Scissors) => Outcome::Draw,
                        
            (Self::Rock,Self::Scissors) |
            (Self::Paper,Self::Rock) |
            (Self::Scissors,Self::Paper) => Outcome::Win,
        }
    }
    pub fn score_against(&self, other:&Self)->usize{
        self.throw_score() + self.outcome_against(other).score()
    }
}

#[derive(Debug,Clone)]
pub enum First{A,B,C}
#[derive(Debug,Clone)]
pub enum Second{X,Y,Z}
impl First {
    pub fn as_throw(&self)->Throw {
        match self {
            Self::A => Throw::Rock,
            Self::B => Throw::Paper,
            Self::C => Throw::Scissors,
        }
    }
}
impl Second {
    pub fn as_throw(&self)->Throw {
        match self {
            Self::X => Throw::Rock,
            Self::Y => Throw::Paper,
            Self::Z => Throw::Scissors,
        }
    }

    pub fn as_outcome(&self)->Outcome {
        match self {
            Self::X => Outcome::Lose,
            Self::Y => Outcome::Draw,
            Self::Z => Outcome::Win,
        }
    }
}
impl FromStr for First {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A"=>Ok(Self::A),
            "B"=>Ok(Self::B),
            "C"=>Ok(Self::C),
            _ => Err(s.into())
        }
    }
}
impl FromStr for Second {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "X"=>Ok(Self::X),
            "Y"=>Ok(Self::Y),
            "Z"=>Ok(Self::Z),
            _ => Err(s.into())
        }
    }
}

#[derive(Debug,Clone)]
pub struct Instructions(Vec<(First,Second)>);
impl FromStr for Instructions {
    type Err=String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.lines().filter_map(|l|l.split_once(' '));
        let parsed = pairs.map(|(f,s)|First::from_str(f).and_then(|f|Second::from_str(s).map(|s|
            (f,s)
        )));
        let vec:Vec<_> = parsed.collect::<Result<_,_>>()?;
        Ok(Self(vec))
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Instructions;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(instructions:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let score_if_second_is_throws = instructions.0.iter().map(|(a,b)|
            b.as_throw().score_against(&a.as_throw())
        ).sum();
        let real_score = instructions.0.iter().map(|(throw,outcome)|{
            let throw=throw.as_throw();
            let outcome=outcome.as_outcome();
            throw.throw_for(&outcome.flip()).throw_score()+outcome.score()
        }).sum();
        Ok((
            (score_if_second_is_throws,"points").into(),
            (real_score,"points").into(),
        ))
    }
}
//12156
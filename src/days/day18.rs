use std::{str::FromStr, collections::HashSet};

use crate::{solution::{Unsolved, AOCSolution, Labeled}, matcher::{Delimeted, FromStrMatcher, Matcher, DelimetedArray}, vec2::Vec3, array::next_chunk};

pub type Pos = usize;


#[derive(Debug,Clone)]
pub struct InputStruct(Vec<Vec3<Pos>>);
impl FromStr for InputStruct {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = Delimeted::<Vec<[Pos;3]>,_,_>::new(DelimetedArray::<3,FromStrMatcher<Pos>,char>::new(Default::default(), ','), "\n");
        let Ok(positions) = positions.next_match(s) else {
            return Err(());
        };
        Ok(Self(positions.matched.into_iter().map(|[x,y,z]|Vec3::new(x,y,z)).collect()))
    }
}

#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub enum Axis {
    X,
    Y,
    Z
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=InputStruct;
    type Part1=Labeled<usize>;
    type Part2=Unsolved;
    type Err = ();
    fn solve(InputStruct(input):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut air_exposed_sides = HashSet::new();
        for Vec3 { x, y, z } in input {
            let sides = [
                (Vec3::new(x,y,z),Axis::X),
                (Vec3::new(x+1,y,z),Axis::X),
                (Vec3::new(x,y,z),Axis::Y),
                (Vec3::new(x,y+1,z),Axis::Y),
                (Vec3::new(x,y,z),Axis::Z),
                (Vec3::new(x,y,z+1),Axis::Z),
            ];
            
            for side in sides {
                if air_exposed_sides.contains(&side) {
                    air_exposed_sides.remove(&side);
                }else {
                    air_exposed_sides.insert(side);
                }
            }
        }
        Ok(((air_exposed_sides.len(),"sides").into(),Unsolved))
    }
}
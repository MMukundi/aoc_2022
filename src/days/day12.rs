use std::str::FromStr;

use crate::{solution::{AOCSolution, Labeled}, grid::Grid, astar::a_star};

#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum MapSpot {
    Start,
    Height(u8),
    End,
}
impl PartialOrd for MapSpot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u8::from(*self).partial_cmp(&u8::from(*other))
    }
}

impl Ord for MapSpot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}
impl MapSpot {
    fn can_climb_to(&self,dest:&Self)->bool {
        let height_diff=  (u8::from(*dest) as i16)-(u8::from(*self) as i16);
        height_diff < 2
    }
}
impl From<MapSpot> for u8 {
    fn from(value: MapSpot) -> Self {
        match value {
            MapSpot::Start => 0,
            MapSpot::Height(n) => n,
            MapSpot::End => 25,
        }
    }
}

#[derive(Debug,Clone)]
pub struct HeightMap(Grid<MapSpot>);
impl FromStr for HeightMap {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let side_len = s.lines().next().ok_or(())?.len();
        let heights = s.chars().filter(|c|!c.is_whitespace()).map(|c|{
            match c {
                'a'..='z'=>{
                    Some(MapSpot::Height((c as u8)-b'a'))
                }
                'S'=>Some(MapSpot::Start),
                'E'=>Some(MapSpot::End),
                _=>None
            }
        }).flatten().collect::<Vec<_>>();
        Ok(HeightMap(Grid::from_parts(heights,side_len).ok_or(())?))
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=HeightMap;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(HeightMap(input):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let start:(usize,usize) = input.indices().find_map(|(i,_)|(input[i] == MapSpot::Start).then_some(i)).ok_or(())?;
        let end:(usize,usize) = input.indices().find_map(|(i,_)|(input[i] == MapSpot::End).then_some(i)).ok_or(())?;

        let ref_input = &input;

        // dbg!(start,end);

        let path = a_star([start], |(x,y)|{
            let current_map_spot = ref_input.get((x.clone(),y.clone()));
            current_map_spot.map(|spot|[
                x.checked_sub(1).map(|x|(x,y)),
                y.checked_sub(1).map(|y|(x,y)),
                x.checked_add(1).map(|x|(x,y)),
                y.checked_add(1).map(|y|(x,y)),
            ].into_iter().flatten().filter(move |s|{
                if let Some(s) = ref_input.get(*s) {
                    spot.can_climb_to(s)
                }else{
                    false
                }
            }).map(|s|(s,1))).into_iter().flatten()
        }, |s|{
            s.0.abs_diff(end.0) + s.1.abs_diff(end.1)
        },  |s|s==&end);

        let scenic_path = a_star(input.indices().filter_map(|(s,_)|(matches!(ref_input[s],MapSpot::Start|MapSpot::Height(0)).then_some(s))), |(x,y)|{
            let current_map_spot = ref_input.get((x.clone(),y.clone()));
            current_map_spot.map(|spot|[
                x.checked_sub(1).map(|x|(x,y)),
                y.checked_sub(1).map(|y|(x,y)),
                x.checked_add(1).map(|x|(x,y)),
                y.checked_add(1).map(|y|(x,y)),
            ].into_iter().flatten().filter(move |s|{
                if let Some(s) = ref_input.get(*s) {
                    spot.can_climb_to(s)
                }else{
                    false
                }
            }).map(|s|(s,1))).into_iter().flatten()
        }, |s|{
            s.0.abs_diff(end.0) + s.1.abs_diff(end.1)
        },  |s|s==&end);

        dbg!(path == scenic_path);

        Ok((
            (path.ok_or(())?.len()-1,"steps").into(),
            (scenic_path.ok_or(())?.len()-1,"steps").into(),
        ))
    }
}
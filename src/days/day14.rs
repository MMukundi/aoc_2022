use std::{str::FromStr, collections::HashSet, ops::ControlFlow};

use crate::solution::{AOCSolution, Labeled};

type Pos = u16;

#[derive(Debug,Clone)]
pub struct RockMap {
    filled_positions:HashSet<(Pos,Pos)>,
    lowest_rock_y: Pos
}
impl FromStr for RockMap {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut filled_positions:HashSet<(Pos,Pos)> = Default::default();
        for l in s.lines() {
            let mut segments = l.split(" -> ").flat_map(|pair|{
                pair.split_once(',').and_then(|(x,y)|x.parse().and_then(|x|y.parse().map(|y|(x,y))).ok())
            });
            if let Some(mut prev) = segments.next(){
                segments.for_each(|p|{
                    if prev.0 == p.0 {
                        filled_positions.extend(if prev.1<p.1 {
                            prev.1..=p.1
                        }else {
                            p.1..=prev.1
                        }.map(|y|(prev.0, y)))
                    }else{ 
                        filled_positions.extend(if prev.0<p.0 {
                            prev.0..=p.0
                        }else {
                            p.0..=prev.0
                        }.map(|x|(x,prev.1)))
                    };
                    prev = p;
                });
            }
        };
        let lowest_rock_y = filled_positions.iter().map(|p|p.1).max().ok_or(())?;
        Ok(Self {
            lowest_rock_y,
            filled_positions
        })
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=RockMap;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut initial_map = input.filled_positions.clone();

        let res = (0usize..).find_map(|number_of_sand_units|{
            let next_positon = (0..=input.lowest_rock_y).try_fold(500,|x,y|{
                let next_x = [x,x-1,x+1].into_iter()
                    .find(|&test_x|{
                        !initial_map.contains(&(test_x,y))
                    });

                if let Some(next_x) = next_x{
                    ControlFlow::Continue(next_x)
                }else{
                    ControlFlow::Break((x,y-1))
                }
            });
            match next_positon {
                ControlFlow::Continue(_)=>Some(number_of_sand_units),
                ControlFlow::Break(pos)=>{
                    initial_map.insert(pos);
                    None
                }
            }
        });
        let mut map_with_floor = input.filled_positions;
        let floor_y = input.lowest_rock_y+2;
        let start = (500,0);
        let additional_units = (0usize..).find_map(|number_of_sand_units|{
            let next_positon = (0..=floor_y).try_fold(500,|x,y|{

                if map_with_floor.contains(&start) {
                    ControlFlow::Break(None)
                } else {                
                    let next_x = [x,x-1,x+1].into_iter()
                        .find(|&test_x|{
                            !map_with_floor.contains(&(test_x,y))
                        });
                    if let Some(next_x) = next_x{
                        ControlFlow::Continue(next_x)
                    } else {
                        ControlFlow::Break(Some((x,y-1)))
                    }
                }
            });
            match next_positon {
                ControlFlow::Continue(x)=>{
                    map_with_floor.insert((x,floor_y-1));
                    None
                },
                ControlFlow::Break(None)=>{
                    Some(number_of_sand_units)
                },
                ControlFlow::Break(Some(pos))=>{
                    map_with_floor.insert(pos);
                    None
                }
            }
        });
        if let Some((p1,p2)) = res.zip(additional_units){
            Ok(((p1,"sand unit").into(),(p2,"sand unit").into()))
        }else{
            Err(())
        }
    }
}
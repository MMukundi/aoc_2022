use std::{str::FromStr, collections::HashSet, ops::ControlFlow};

use crate::{solution::{Unsolved, AOCSolution, Labeled}, vec2::Vec2, array::next_chunk};

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum JetMove {
    Left,
    Right
}
// #[derive(Debug,Clone,Copy,Eq,PartialEq)]
// pub enum RockShape {
//     FlatHorizontal=0,
//     Plus=1,
//     L=2,
//     FlatVertical=3,
//     Square=4
// }
type Pos = Vec2<usize>;
const ROCK_SHAPES:[([Pos;4],Option<Pos>);5] = [
    ([Pos::new(0,0),Pos::new(1,0),Pos::new(2,0),Pos::new(3,0)],None),
    ([Pos::new(1,0),Pos::new(0,1),Pos::new(1,2),Pos::new(2,1)],Some(Pos::new(1,1))),
    ([Pos::new(0,0),Pos::new(1,0),Pos::new(2,0),Pos::new(2,1)],Some(Pos::new(2,2))),
    ([Pos::new(0,0),Pos::new(0,1),Pos::new(0,2),Pos::new(0,3)],None),
    ([Pos::new(0,0),Pos::new(1,0),Pos::new(0,1),Pos::new(1,1)],None),
];


#[derive(Debug,Clone)]
pub struct JetMoves(Vec<JetMove>);
impl FromStr for JetMoves {
    type Err=char;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().map(|c|match c {
            '<' => Ok(JetMove::Left),
            '>' => Ok(JetMove::Right),
            c => Err(c)
        }).collect::<Result<_,_>>().map(Self)
    }
}

pub struct Counted<'a,I>(I,&'a mut usize);
impl <'a,I:Iterator> Iterator for Counted<'a, I>{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        *self.1+=1;
        self.0.next()
    }
}

const CAVE_WIDTH:usize = 7;

#[derive(Debug,Clone)]
pub struct SaveState {
    block_count:usize,
    repeat_height:usize,
}

fn part_1(moves:&Vec<JetMove>)->usize{
    let mut stationary_rocks = HashSet::new();
    let mut max_y_part_1 = 0;
    let mut count = 0;
    let mut jet_moves = moves.iter().cycle();

    for rock in ROCK_SHAPES.into_iter().cycle().take(2022){
        let rock = rock.0.into_iter().chain(rock.1);
        let mut pos = Pos::new(2,max_y_part_1+3);
        loop {
            let Some(jet_move) = jet_moves.next() else {
                break
            };
            match jet_move {
                JetMove::Left => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x>0 && !stationary_rocks.contains(&Pos::new(p.x-1,p.y))){
                        pos.x-=1;
                    }
                },
                JetMove::Right => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x!=(CAVE_WIDTH-1) && !stationary_rocks.contains(&Pos::new(p.x+1,p.y))){
                        pos.x+=1;
                    }
                },
            }
            if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.y>0 && !stationary_rocks.contains(&Pos::new(p.x,p.y-1))){
                pos.y-=1;
            }else {
                break;
            }
        };
        stationary_rocks.extend(rock.clone().map(|rock_offset|pos+rock_offset));
        for rock_pos_y in rock.map(|rock_offset|pos.y+rock_offset.y+1) {
            max_y_part_1 = max_y_part_1.max(rock_pos_y);
        }
    }
    max_y_part_1
}

fn part_2(moves:&Vec<JetMove>)->Result<usize,()>{
    let mut stationary_rocks = HashSet::new();
    let mut max_y=0;
    let mut save_state = Option::<(usize,usize)>::None;
    let mut jet_moves = moves.iter().cycle();
    let mut rocks = ROCK_SHAPES.into_iter().cycle();

    let (
        (start_repeat_rock_count,repeat_start_height),
        (end_repeat_rock_count ,repeat_end_height),
    ) = loop{
        let Some(rock) = rocks.next() else {
            return Err(())
        };
        let rock = rock.0.into_iter().chain(rock.1);
        let mut pos = Pos::new(2,max_y+3);
        loop {
            let Some(jet_move) = jet_moves.next() else {
                return Err(());
            };
            match jet_move {
                JetMove::Left => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x>0 && !stationary_rocks.contains(&Pos::new(p.x-1,p.y))){
                        pos.x-=1;
                    }
                },
                JetMove::Right => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x!=(CAVE_WIDTH-1) && !stationary_rocks.contains(&Pos::new(p.x+1,p.y))){
                        pos.x+=1;
                    }
                },
            }
            if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.y>0 && !stationary_rocks.contains(&Pos::new(p.x,p.y-1))){
                pos.y-=1;
            }else {
                break;
            }
        };
        stationary_rocks.extend(rock.clone().map(|rock_offset|pos+rock_offset));
        for rock_pos_y in rock.map(|rock_offset|pos.y+rock_offset.y+1) {
            max_y = max_y.max(rock_pos_y);
        }
        let prev_y=max_y-1;
        if (0..CAVE_WIDTH).all(|x|stationary_rocks.contains(&Pos::new(x,prev_y)))
        {
            if let Some(save_state) = save_state {
                break (
                    save_state,
                    (stationary_rocks.len(),prev_y)
                );
            }else{
                save_state = Some((
                    stationary_rocks.len(),
                    prev_y
                ));
            };
        }
    };

    let current_max_y = max_y;

    const A_BILLY:usize = 10usize.pow(12);
    let repeated_rocks = end_repeat_rock_count-start_repeat_rock_count;

    let rocks_remaining = A_BILLY-start_repeat_rock_count;
    let rocks_after_repeat = rocks_remaining%repeated_rocks;

    let repeated_height = (repeat_end_height-repeat_start_height)*(rocks_remaining/repeated_rocks);
    let height_after_repeat = repeated_height+repeat_start_height;

    for rock in rocks.take(rocks_after_repeat) {
        let rock = rock.0.into_iter().chain(rock.1);
        let mut pos = Pos::new(2,max_y+3);
        for jet_move in &mut jet_moves {
            match jet_move {
                JetMove::Left => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x>0 && !stationary_rocks.contains(&Pos::new(p.x-1,p.y))){
                        pos.x-=1;
                    }
                },
                JetMove::Right => {
                    if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.x!=(CAVE_WIDTH-1) && !stationary_rocks.contains(&Pos::new(p.x+1,p.y))){
                        pos.x+=1;
                    }
                },
            }
            if rock.clone().map(|rock_offset|pos+rock_offset).all(|p|p.y>0 && !stationary_rocks.contains(&Pos::new(p.x,p.y-1))){
                pos.y-=1;
            }else {
                break;
            }
        };
        stationary_rocks.extend(rock.clone().map(|rock_offset|pos+rock_offset));
        for rock_pos_y in rock.map(|rock_offset|pos.y+rock_offset.y+1) {
            max_y = max_y.max(rock_pos_y);
        }
    }
    dbg!(stationary_rocks.iter().max_by_key(|p|p.y),max_y); 

    // 349959777046
    // 349959777039

    let final_height =(max_y-current_max_y)+height_after_repeat;
    Ok(final_height)
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=JetMoves;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(JetMoves(moves):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        Ok(((part_1(&moves),"blocks").into(),(part_2(&moves)?,"blocks").into()))
    }
}
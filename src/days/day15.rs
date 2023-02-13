use std::{str::FromStr, ops::Range, marker::PhantomData, fmt::Debug};

use crate::solution::{AOCSolution, Labeled};

type PosCoord = isize;
type Pos = (PosCoord,PosCoord);

#[derive(Debug,Clone)]
pub struct Scanner {
    position:Pos,
    manhattan_to_closest:usize,
}
impl Scanner {
    // pub fn rules_out_location(&self,&(x,y):&Pos)->bool{
    //     // manhattan(_, _)
    // }
}

#[derive(Debug,Clone)]
pub struct ScannersAndBeacons {
    scanners: Vec<Scanner>,
    beacon_positions: Vec<Pos> 
}
impl FromStr for ScannersAndBeacons {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scanners = Vec::new();
        let mut beacon_positions = Vec::new();
        let scanners_and_beacons = s.lines().filter_map(|line|{
            line.split_once(": ")
        }).filter_map(|(scanner,beacon)|{
            scanner.split_once(", ").zip(beacon.split_once(", "))
        }).filter_map(|(scanner,beacon)|{
            fn split_to_point((almost_x,almost_y):(&str,&str))->Option<Pos> {
                let x = almost_x.split_once("=").and_then(|(_,x)|x.parse().ok())?;
                let y = almost_y.split_once("=").and_then(|(_,y)|y.parse().ok())?;
                Some((x, y))
            }
            split_to_point(scanner).zip(split_to_point(beacon))
        });
        for (scanner,beacon) in scanners_and_beacons {
            // if let Some(existing_index) = beacon_positions.iter().position(|b|b==&beacon){
            //     existing_index
            // }else{
            //     let index = beacon_positions.len();
            //     beacon_positions.push(beacon);
            //     index
            // };
            beacon_positions.push(beacon);

            
            scanners.push(Scanner {
                manhattan_to_closest:manhattan(&scanner,&beacon),
                position: scanner,
            });
        }
        Ok(Self {
            scanners,
            beacon_positions,
        })
    }
}

#[inline]
fn manhattan(&(a_x,a_y):&Pos,&(b_x,b_y):&Pos)->usize{
    a_x.abs_diff(b_x) + a_y.abs_diff(b_y)
}
// #[inline]
// fn min_max<O:Ord+Clone,I:Iterator<Item=O>>(mut iter:I)->Option<(O,O)>{
//     let first = iter.next()?;
//     Some(iter.fold((first.clone(),first), |(min,max),next|{
//         if next < min {
//             (next,max)
//         } else if max<next {
//             (min,next)
//         }else {
//             (min,max)
//         }
//     }))
// }

#[derive(Debug,Default)]
pub struct RangeSet<T> {
    ranges: Vec<Range<T>>,
    phantom_data:PhantomData<T>
}
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum RangeIndex {
    Before(usize),
    Within(usize),
    // After(usize)
}
impl <T:PartialEq+PartialOrd+Clone+Debug> RangeSet<T>{
    // pub fn contains(&self,item:&T)->bool {
    //     self.ranges.iter().any(|r|r.contains(item))
    // }
    pub fn min(&self)->Option<&T> {
        self.ranges.first().map(|r|&r.start)
    }
    pub fn max(&self)->Option<&T> {
        self.ranges.last().map(|r|&r.end)
    }
    // pub fn is_empty(&self) ->bool {
    //     self.ranges.is_empty()
    // }
    pub fn clear(&mut self){
        self.ranges.clear()
    }
    pub fn ranges(&self)->&Vec<Range<T>>{
        &self.ranges
    }
    pub fn insert(&mut self, new_range:Range<T>) {
        if new_range.end < new_range.start {
            return;
        }
        let start_index = self.ranges.iter().enumerate().find_map(|(i,existing_ranges)|{
            if new_range.start < existing_ranges.start {
                Some(RangeIndex::Before(i))
            }else if new_range.start < existing_ranges.end {
                Some(RangeIndex::Within(i))
            }else{
                None
            }
        });
        let end_index = self.ranges.iter().enumerate().rev().find_map(|(i,existing_ranges)|{
            if new_range.end > existing_ranges.end {
                Some(RangeIndex::Before(i))
            }else if new_range.end > existing_ranges.start {
                Some(RangeIndex::Within(i))
            }else{
                None
            }
        });
        // println!("{:?}; {self:?}",(start_index,end_index));
        let (start,end) = match (start_index,end_index){
            (Some(s), Some(e)) => (s,e),
            (Some(_), None) => {
                self.ranges.insert(0, new_range);
                return;
            },
            _ => {
                self.ranges.push( new_range);
                return;
            }
        };
        match (start,end) {
            (RangeIndex::Before(after_start_of_range), RangeIndex::Before(before_end_of_range)) => {
                self.ranges.drain(after_start_of_range..=before_end_of_range); // 
                self.ranges.insert(after_start_of_range, new_range);
            },
            (RangeIndex::Before(after_start_of_range), RangeIndex::Within(containing_end_of_range)) => {
                self.ranges.drain(after_start_of_range..containing_end_of_range); // don't drain the containing-end range
                self.ranges[after_start_of_range].start = new_range.start; // alter the range containing the end to include the start
            },
            (RangeIndex::Within(containing_start_of_range), RangeIndex::Before(before_end_of_range)) => {
                let after_start_of_range = containing_start_of_range+1;
                if after_start_of_range < self.ranges.len() {
                    self.ranges.drain(after_start_of_range..=before_end_of_range); // don't drain the containing-start range
                }
                self.ranges[containing_start_of_range].end = new_range.end; // alter the range containing the end to include the new end
            },
            (RangeIndex::Within(containing_start_of_range), RangeIndex::Within(containing_end_of_range)) => {
                if containing_start_of_range != containing_end_of_range {
                    let after_start_of_range = containing_start_of_range+1;
                    // Drain the end-endpoint,
                    // And then retrieve it's end
                    let end_range = self.ranges.drain(after_start_of_range..=containing_end_of_range).next_back();
                    if let Some (end_range) = end_range{
                        self.ranges[containing_start_of_range].end = end_range.end; // alter the range containing the end to include the new end
                    }; 
                }
            },
        }
    }
    pub fn remove(&mut self, range_to_remove: Range<T>) {
        let start_index = self.ranges.iter().enumerate().find_map(|(i,existing_ranges)|{
            if range_to_remove.start < existing_ranges.start {
                Some(RangeIndex::Before(i))
            }else if range_to_remove.start < existing_ranges.end {
                Some(RangeIndex::Within(i))
            }else{
                None
            }
        });
        let end_index = self.ranges.iter().enumerate().rev().find_map(|(i,existing_ranges)|{
            if range_to_remove.end > existing_ranges.end {
                Some(RangeIndex::Before(i))
            }else if range_to_remove.end > existing_ranges.start {
                Some(RangeIndex::Within(i))
            }else{
                None
            }
        });
        let Some((start,end)) = start_index.zip(end_index) else {
            return;
        };
        match (start,end) {
            (RangeIndex::Before(after_start_of_range), RangeIndex::Before(before_end_of_range)) => {
                self.ranges.drain(after_start_of_range..=before_end_of_range); // 
            },
            (RangeIndex::Before(after_start_of_range), RangeIndex::Within(containing_end_of_range)) => {
                self.ranges.drain(after_start_of_range..containing_end_of_range); // don't drain the containing-end range
                self.ranges[after_start_of_range].start = range_to_remove.end; // alter the range containing the end to include the start
            },
            (RangeIndex::Within(containing_start_of_range), RangeIndex::Before(before_end_of_range)) => {
                let after_start_of_range = containing_start_of_range+1;
                if after_start_of_range < self.ranges.len() {
                    self.ranges.drain(after_start_of_range..=before_end_of_range); // don't drain the containing-start range
                }
                self.ranges[after_start_of_range].end = range_to_remove.start; // alter the range containing the end to include the new end
            },
            (RangeIndex::Within(containing_start_of_range), RangeIndex::Within(containing_end_of_range)) => {
                let after_start_of_range = containing_start_of_range+1;
                if after_start_of_range < self.ranges.len() && containing_start_of_range!=containing_end_of_range{
                    self.ranges.drain(after_start_of_range..containing_end_of_range);// don't drain either endpoint range
                    // The two ranges should now be adjacent
                    self.ranges[containing_start_of_range].end = range_to_remove.start; 
                    self.ranges[after_start_of_range].start = range_to_remove.end; 
                }else {
                    let old_end = std::mem::replace(
                        &mut self.ranges[containing_start_of_range].end,
                        range_to_remove.start
                    );
                    self.ranges.push(range_to_remove.end..old_end)
                }
            },
        }
    }
}

impl <T:Ord+Clone+Debug> Extend<Range<T>> for RangeSet<T> {
    fn extend<I: IntoIterator<Item = Range<T>>>(&mut self, iter: I) {
        for new_range in iter {
            self.insert(new_range)
        }
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=ScannersAndBeacons;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        // dbg!(&input);
        let ScannersAndBeacons { beacon_positions, scanners } = input;

        // const Y:isize = 10;//5181556
        const Y:isize = 2_000_000;//5181556
        // const Y:isize = 10;
        let mut covered_ranges:RangeSet<_> = Default::default(); 
        covered_ranges.extend(scanners.iter().filter_map(|s|{
            let (x,y) = s.position;
            let remaining = s.manhattan_to_closest.checked_sub(y.abs_diff(Y))? as isize;
            Some(x-remaining..x+remaining+1)
        }));
        for beacon_x in beacon_positions.iter().filter_map(|&(b_x,b_y)|(b_y==Y).then_some(b_x)) {
            covered_ranges.remove(beacon_x..beacon_x+1);
        }
        let maybe_count = covered_ranges.ranges().iter().map(|r|r.len()).sum();

        const MAX:isize = 2*Y;
        let x_y = (0isize..=MAX).find_map(|target_y|{
            covered_ranges.clear();
            covered_ranges.extend(scanners.iter().filter_map(|s|{
                // dbg!(target_y,s.position);
                let (x,y) = s.position;
                let remaining = s.manhattan_to_closest.checked_sub(y.abs_diff(target_y))? as isize;
                Some(((x-remaining).max(0))..((x+remaining+1).max(0)))
            }));
            // dbg!(&covered_ranges);
            // println!("{target_y}/{MAX} = {:.2}%", 100.*(target_y as f64)/(MAX as f64));
            if covered_ranges.ranges().len() > 1 {
                covered_ranges.ranges().windows(2).find_map(|w|{
                    (w[0].end < w[1].start).then_some((w[0].end,target_y))
                })
            }else {
                if let Some(&max) = covered_ranges.max(){
                    if max <= MAX {
                        Some((max,target_y))
                    }else{None}
                }else if let Some(&min) = covered_ranges.min(){
                    if 0 <= min {
                        Some((min,target_y))
                    }else{None}
                }else{None}
            }
        });
        
        Ok((
            (maybe_count,"position").into(),
            (x_y.map(|(x,y)|x*4_000_000+y).ok_or(())? as usize,"hertz").into(),            
        ))
        // // 4641885
        // todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day15::RangeSet;

    #[test]
    pub fn test(){
        let mut r = RangeSet::<usize>::default();
        r.extend([10..30,100..130]);
        dbg!(&r);
        r.extend([20..120]);
        dbg!(&r);
        r.remove(20..120);
        dbg!(&r);
    }
}
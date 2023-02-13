use core::num;
use std::{str::FromStr, collections::{HashMap, HashSet}, ops::Range, iter::{Flatten, Cloned}};

use rayon::{option::Iter, range};

use crate::solution::{Unsolved, AOCSolution};

type Num = i64;

#[derive(Debug,Clone)]
pub struct Numbers(Vec<Num>);
impl FromStr for Numbers {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines().map(Num::from_str).collect::<Result<Vec<_>,_>>().map_err(|_|{}).map(Numbers)
    }
}
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct ShuffledList<T>{
    values: Vec<T>,
    indices: Vec<Range<usize>>
}
impl <T> ShuffledList<T> {
    pub fn new(values:Vec<T>)->Self {
        Self {
            indices: vec![0..values.len()],
            values,
        }
    }
    pub fn is_empty(&self)->bool {
        self.values.is_empty()
    }
    pub fn len(&self)->usize {
        self.values.len()
    }
    #[inline]
    fn range_containing(&self, index:usize) -> (usize, usize){
        let mut items_passed = 0;
        let containing_range_index = self.indices.iter().position(move |range|{
            let len = range.len();
            if index < items_passed+len {
                true
            } else {
                items_passed+=len;
                false
            }
        });
        (containing_range_index.unwrap(),items_passed)
    }
    #[inline]
    fn range_containing_original(&self, original_index:usize)->Option<usize>{
        self.indices.iter().position(move |range|{
            range.contains(&original_index)
        })
    }
    #[inline]
    pub fn nth_original_after(&self, original_index:usize,offset:usize)->Option<usize>{
        let count = self.len();
        let mut remaining = offset%count;
        let mut range_index = self.range_containing_original(original_index)?;
        let mut remaining_in_range = self.indices[range_index].end-original_index;
        // dbg!(original_index,offset,remaining_in_range,remaining,range_index);
        while let Some(new_remaining) = remaining.checked_sub(remaining_in_range) {
            remaining = new_remaining;
            if range_index == self.indices.len()-1 {
                range_index = 0;
            }else{
                range_index+=1;
            };
            remaining_in_range = self.indices[range_index].len();
        }
        Some(self.indices[range_index].end-remaining_in_range+remaining)
    }

    pub fn get_by_original_index(&self, original_index:usize)->Option<&T> {
        self.values.get(original_index)
    }
    pub fn get_mut_by_original_index(&mut self, original_index:usize)->Option<&mut T> {
        self.values.get_mut(original_index)
    }
    pub fn move_forward(&mut self, original_index:usize,mut offset:usize) {
        if offset == 0 {
            return;
        }
        if let Some(mut range_index) = self.range_containing_original(original_index) {
            let original_range = &self.indices[range_index];
            self.indices.splice(range_index..=range_index, [
                original_range.start..original_index,
                original_index+1..original_range.end,
            ]);
            range_index = range_index+1;

            let mut remaining = offset;
            while let Some(new_remaining) = remaining.checked_sub(self.indices[range_index].len()) {
                remaining = new_remaining;
                if range_index == self.indices.len()-1 {
                    range_index = 0;
                }else{
                    range_index+=1;
                }
            }

            let range = &self.indices[range_index];
            let offset_into_range = range.start+remaining;

            self.indices.splice(range_index..=range_index, [
                range.start..offset_into_range,
                original_index..original_index+1,
                offset_into_range..range.end,
            ]);

            self.indices.retain(|r|!r.is_empty())
        }
    }
    pub fn move_backward(&mut self, original_index:usize,mut offset:usize) {
        if offset == 0 {
            return;
        }
        if let Some(mut range_index) = self.range_containing_original(original_index) {
            let original_range = &self.indices[range_index];
            self.indices.splice(range_index..=range_index, [
                original_range.start..original_index,
                original_index+1..original_range.end,
            ]);

            let mut remaining = offset;
            while let Some(new_remaining) = remaining.checked_sub(self.indices[range_index].len()) {
                remaining = new_remaining;
                if range_index == 0 {
                    range_index = self.indices.len()-1;
                }else{
                    range_index-=1;
                }
            }

            let range = &self.indices[range_index];
            let offset_into_range = range.end-remaining;

            self.indices.splice(range_index..=range_index, [
                range.start..offset_into_range,
                original_index..original_index+1,
                offset_into_range..range.end,
            ]);

            self.indices.retain(|r|!r.is_empty())
        }
    }
    fn iter(&self)->ShuffledListIter<T>{
        (&self).into_iter()
    }
}
impl <'a,T> IntoIterator for &'a ShuffledList<T> {
    type Item=&'a T;

    type IntoIter=ShuffledListIter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            indices: self.indices.iter().cloned().flatten(),
            values: &self.values
        }
    }
}
#[derive(Debug,Clone)]
pub struct ShuffledListIter<'a,T>{
    values: &'a Vec<T>,
    indices: Flatten<Cloned<std::slice::Iter<'a,Range<usize>>>>
}
impl <'a,T> Iterator for ShuffledListIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().and_then(|i|self.values.get(i))
    }
}
#[cfg(test)]
mod test {
    use super::ShuffledList;

    #[test]
    fn te() {
        let data = (0..30).collect::<Vec<_>>();
        let mut shuff = ShuffledList::new(data);
        shuff.move_backward(15, 1);
        dbg!(&shuff);
        shuff.move_forward(0, 10);
        dbg!(&shuff);
        dbg!(shuff.iter().cloned().collect::<Vec<_>>());
    }
}

fn mix(numbers:Vec<Num>,mixes:Num, key:Num)->Num {
    let count = numbers.len();
    let mut numbers = ShuffledList::new(numbers);
    for n in &mut numbers.values {
        *n*=key;
    }
    for _ in 0..mixes{
        for i in 0..count {
            let Some(&num) = numbers.get_by_original_index(i) else {
                continue;
            };
            let num = num%(count-1) as Num;
            if num >= 0 {
                numbers.move_forward(i, num as _)
            }else {
                numbers.move_backward(i, (-num) as _)
            }
        }
    }
    let mut sum = 0;
    let mut search_index = numbers.values.iter().position(|&n|n==0).unwrap();
    for _ in 0..3 {
        if let Some(new_i) = numbers.nth_original_after(search_index, 1000) {
            search_index = new_i;
            if let Some(val) = numbers.get_by_original_index(new_i) {
                sum+=val;
            }
        }
    }
    sum
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=Numbers;
    type Part1=Num;
    type Part2=Num;
    type Err = ();
    fn solve(Numbers(numbers):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        Ok((
            mix(numbers.clone(),1,1),
            mix(numbers,10,811589153)
        ))
    }
}
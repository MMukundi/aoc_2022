use std::{str::FromStr, borrow::{Borrow, BorrowMut}, fmt::Debug};

use crate::{solution::{AOCSolution, Labeled}, grid::Grid};

#[derive(Debug,Clone)]
pub struct TreeGrid {
    heights:Grid<u8>,
}
impl FromStr for TreeGrid {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let column_count = lines.peek().cloned().map(str::len).unwrap_or(0);
        let mut heights = Grid::new(column_count);
        for line in lines {
            let line_heights = line.chars().map(|c|match c {
                '0'=>0,
                '1'=>1,
                '2'=>2,
                '3'=>3,
                '4'=>4,
                '5'=>5,
                '6'=>6,
                '7'=>7,
                '8'=>8,
                '9'=>9,
                _=>panic!("ah!")
            });
            drop(heights.insert_row(line_heights));
        };
        Ok(Self{
            heights,
        })
    }
}

fn tree_visibility_easy<H:IntoIterator, V:IntoIterator>(tree_heights: H, tree_visiblities: V)
where 
    H::Item:Borrow<u8>,
    V::Item:BorrowMut<bool>{
        let mut height_and_visibility = tree_heights.into_iter().zip(tree_visiblities.into_iter());
        let Some((first_height,mut first_vis)) = height_and_visibility.next() else {return};
        *first_vis.borrow_mut() = true;
        height_and_visibility.fold(*first_height.borrow(),|current_max,(new_height,mut new_tree_is_vis)|{
            let new_height = *new_height.borrow();
            let new_tree_is_vis = new_tree_is_vis.borrow_mut();
            if new_height > current_max {
                *new_tree_is_vis = true;
                new_height
            }else {
                current_max
            }
        });
}
/// Every sequence of trees can be split into strictly ascending subsequences
///                       T 
///                    T  T        T 
///                    T  T        T
///     T  T        T  T  T        T 
///  T  T  T  T  T  T  T  T  T  T  T  T  T  T  T  T  
/// |____||_||_||__________||_||____||_||_||_||_||_|
/// 
/// Each of these subsequences should record the number of trees above the tallest tree in the previous subsequenceses
/// 
///                       T 
///                    T  T        T 
///                    T  T        T
///     T  T        T  T  T        T 
///  T  T  T  T  T  T  T  T  T  T  T  T  T  T  T  T  
/// |____||_||_||__________||_||____||_||_||_||_||_|
/// 0      0  0  2,2,3
fn tree_visibility_count<H:IntoIterator, V:IntoIterator>(tree_heights: H, tree_visiblity_count: V)
where 
    H::Item:Borrow<u8>+Debug,
    V::Item:BorrowMut<usize>+Debug{
    let mut waiting_to_enter = [();10].map(|_|Option::<V::Item>::default());
    let mut waiting_for = [();10].map(|_|Vec::<V::Item>::default());
    let height_and_visibility = tree_heights.into_iter().zip(tree_visiblity_count.into_iter());
    for (new_height, new_tree_vis_count) in height_and_visibility {
        let new_height = *new_height.borrow();
        let (waiting_to_enter_for_this_one,still_waiting) = waiting_to_enter.split_at_mut(new_height as usize);
        {
            let (waiting_for_this_one,rest) = waiting_for.split_at_mut((new_height+1) as usize);
            let trees_that_can_see_this_one = waiting_for_this_one.iter_mut().flat_map(|hs|{
                hs.drain(..)
            })
            .chain(waiting_to_enter_for_this_one.iter_mut().flat_map(Option::take))
            .map(|mut h|{
                *h.borrow_mut() += 1;
                h
            });
            if let Some(next) = rest.first_mut() {
                next.extend(trees_that_can_see_this_one);
            }else{ 
                trees_that_can_see_this_one.for_each(drop);
            }
        }
        let mut still_waiting = still_waiting.iter_mut();
        if let Some(Some(this_height)) = still_waiting.next() {
            *this_height.borrow_mut() += 1;
        }
        for i in still_waiting.flatten() {
            *i.borrow_mut() += 1;
        }
        waiting_to_enter[new_height as usize]  = Some(new_tree_vis_count);
    };
}

fn visibility_from_edge<H:IntoIterator, V:IntoIterator>(tree_heights: H, tree_visiblities: V)
where 
    H::Item:Borrow<u8>,
    V::Item:BorrowMut<bool>
{
    tree_visibility_easy(tree_heights,tree_visiblities)
    // tree_visibility(tree_heights,tree_visiblities,|&h,current_max|h>current_max,|_,_|{})
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=TreeGrid;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(grid:Self::Input)->Result<(Self::Part1,Self::Part2),()> {        
        let mut is_visible = grid.heights.map_ref(|_|false);
        for (row,row_is_vis) in grid.heights.rows().into_iter().zip(is_visible.rows_mut()) {
            visibility_from_edge(row,row_is_vis);
        }
        for (row,row_is_vis) in grid.heights.rows().into_iter().zip(is_visible.rows_mut()) {
            visibility_from_edge(row.iter().rev(),row_is_vis.iter_mut().rev());
        }
        for (col,col_is_vis) in grid.heights.cols().into_iter().zip(is_visible.cols_mut()) {
            visibility_from_edge(col,col_is_vis);
        }
        for (col,col_is_vis) in grid.heights.cols().into_iter().zip(is_visible.cols_mut()) {
            visibility_from_edge(col.into_iter().rev(),col_is_vis.into_iter().rev());
        }

        let mut row_counts = grid.heights.map_ref(|_|0);
        for (row,row_counts) in grid.heights.rows().into_iter().zip(row_counts.rows_mut()) {
            tree_visibility_count(row,row_counts);
        }
        let mut rev_row_counts = grid.heights.map_ref(|_|0);
        for (row,row_counts) in grid.heights.rows().into_iter().zip(rev_row_counts.rows_mut()) {
            tree_visibility_count(row.iter().rev(),row_counts.iter_mut().rev());
        }
        let mut col_counts = grid.heights.map_ref(|_|0);
        for (col,col_counts) in grid.heights.cols().into_iter().zip(col_counts.cols_mut()) {
            tree_visibility_count(col,col_counts);
        }
        let mut rev_col_counts = grid.heights.map_ref(|_|0);
        for (col,col_counts) in grid.heights.cols().into_iter().zip(rev_col_counts.cols_mut()) {
            tree_visibility_count(col.into_iter().rev(),col_counts.into_iter().rev());
        }
        let scores = rev_col_counts.zip(rev_row_counts).zip(col_counts).zip(row_counts)
            .map(|(((a,b),c),d)|a*b*c*d);
        
        Ok(((is_visible.data().iter().filter(|&&a|a).count(),"tree").into(),(scores.into_data().into_iter().max().unwrap(),"scenic points").into()))
    }
}
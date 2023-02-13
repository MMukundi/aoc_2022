use core::slice;
use std::{str::FromStr, collections::{VecDeque, HashSet}, ops::Add, time::Instant, borrow::Borrow};

use rayon::prelude::*;

use crate::solution::{Unsolved, AOCSolution, Labeled};

type Cost = u16;

#[repr(u16)]
#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug,Clone,Copy,Hash,PartialEq, Eq)]
pub struct State {
    robots:[Cost;4],
    resources:[Cost;4],
    minutes_remaining: u16
}
impl State {
    pub fn try_build_robot<const RESOURCE:u16>(
        mut self,
        blueprint:&BlueprintCosts
    )-> Option<Self>
    {
        let robot_count = &mut self.robots[RESOURCE as usize];
        let max_robot_count = blueprint.max_robots[RESOURCE as usize];
        if *robot_count < max_robot_count {
            let robot_cost = blueprint.robot_costs[RESOURCE as usize];
            let new_resources = [
                self.resources[Resource::Ore as usize].checked_sub(robot_cost[Resource::Ore as usize])?,
                self.resources[Resource::Clay as usize].checked_sub(robot_cost[Resource::Clay as usize])?,
                self.resources[Resource::Obsidian as usize].checked_sub(robot_cost[Resource::Obsidian as usize])?,
                self.resources[Resource::Geode as usize]
            ];
            self.resources = new_resources;
            *robot_count+=1;
            Some(self)
        }else{
            None
        }
    }
}


#[derive(Debug,Clone,Copy)]
pub struct BlueprintCosts{
    robot_costs:[[Cost;3];4],
    max_robots:[Cost;4],
}
impl FromStr for BlueprintCosts {
    type Err= Option<<Cost as FromStr>::Err>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let ore_robot_ore = words.nth(6).ok_or(None)?;
        let clay_robot_ore = words.nth(5).ok_or(None)?;
        let obisidian_robot_ore = words.nth(5).ok_or(None)?;
        let obisidian_robot_clay = words.nth(2).ok_or(None)?;
        let geode_robot_ore = words.nth(5).ok_or(None)?;
        let geode_robot_obsidian = words.nth(2).ok_or(None)?;


        let ore_robot_ore = ore_robot_ore.parse().map_err(Some)?;
        let clay_robot_ore = clay_robot_ore.parse().map_err(Some)?;
        let obisidian_robot_ore_and_clay= 
        (
            obisidian_robot_ore.parse().map_err(Some)?,
            obisidian_robot_clay.parse().map_err(Some)?,
        );
        let geode_robot_ore_and_obsidian= 
        (
            geode_robot_ore.parse().map_err(Some)?,
            geode_robot_obsidian.parse().map_err(Some)?,
        );

        Ok(Self {
            robot_costs: [
                [ore_robot_ore,0,0],
                [clay_robot_ore,0,0],
                [obisidian_robot_ore_and_clay.0,obisidian_robot_ore_and_clay.1,0],
                [geode_robot_ore_and_obsidian.0,0,geode_robot_ore_and_obsidian.1],
            ],
            max_robots: [
                ore_robot_ore.max(clay_robot_ore).max(obisidian_robot_ore_and_clay.0).max(geode_robot_ore_and_obsidian.0),
                obisidian_robot_ore_and_clay.1,
                geode_robot_ore_and_obsidian.1,
                Cost::MAX
            ]
        })
    }
}
#[derive(Debug,Clone)]
pub struct Blueprints(Vec<BlueprintCosts>);
impl FromStr for Blueprints {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Blueprints(s.lines().map(BlueprintCosts::from_str).collect::<Result<Vec<_>,_>>().map_err(|_|{})?))
    }
}

fn sim_robots<B:Borrow<BlueprintCosts>>(steps:u16,iter: impl IndexedParallelIterator<Item=B>)->impl IndexedParallelIterator<Item = Cost>{
    iter.map(move |costs|{
        let costs = costs.borrow();
        let mut queue = VecDeque::<State>::new();
        queue.push_back(State{
            robots: [1,0,0,0],
            resources: Default::default(),
            minutes_remaining: steps,
        });
        let mut max = State{
            robots: [1,0,0,0],
            resources: Default::default(),
            minutes_remaining: 0,
        };
        let mut visited=HashSet::<State>::default();
        let mut min_minutes = steps;
        while let Some(mut state) = queue.pop_front() {
            // println!("-----\n{state:?}");
            if let Some(minutes_remaining) = state.minutes_remaining.checked_sub(1) {
                if minutes_remaining < min_minutes {
                    min_minutes = minutes_remaining;
                    dbg!(min_minutes);
                }
                let deltas = state.robots;
                let states = [
                    state.try_build_robot::<0>(&costs),
                    state.try_build_robot::<1>(&costs),
                    state.try_build_robot::<2>(&costs),
                    state.try_build_robot::<3>(&costs),
                ].into_iter().flatten();

                for mut new_state in states {
                    for (resource,delta) in new_state.resources.iter_mut().zip(deltas) {
                        *resource+=delta;
                    }
                    new_state.minutes_remaining=minutes_remaining;
                    if !visited.contains(&new_state) {
                        visited.insert(new_state);
                        queue.push_back(new_state);
                    }
                }
                for (resource,delta) in state.resources.iter_mut().zip(deltas) {
                    *resource+=delta;
                }
                state.minutes_remaining=minutes_remaining;
                if !visited.contains(&state) {
                    visited.insert(state);
                    queue.push_back(state);
                }
            }else{
                if max.resources[Resource::Geode as usize] <= state.resources[Resource::Geode as usize] {
                    max = state
                }
            }
        };
        max.resources[Resource::Geode as usize]
    })
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Blueprints;
    type Part1=usize;
    type Part2=usize;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let sum = sim_robots(24, input.0.par_iter()).enumerate().map(|(i,geodes)|(i+1)*geodes as usize).sum();   
        dbg!(sum);
        let trio = sim_robots(32, input.0.par_iter().take(3)).map(|a|a as usize).product();   
        dbg!(trio);

        Ok((
            sum,
            trio,
        ))
    }
}

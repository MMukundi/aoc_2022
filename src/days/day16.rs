use std::{str::FromStr, collections::{HashMap, HashSet, VecDeque}, ops::{Deref, DerefMut}, num::NonZeroU8, hash::Hash};

use crate::{solution::{Unsolved, AOCSolution}, iter::ArrayProduct, array::next_chunk, grid::Grid, bitset::BitSet, or::Or};
use itertools::*;

#[derive(Debug,Clone,Copy,PartialEq, Eq,Hash)]
pub struct RoomName([char;2]);
impl FromStr for RoomName {
    type Err=usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if let Some((a,b)) = chars.next().zip(chars.next()) {
            Ok(Self([a,b]))
        }else{
            Err(s.len())
        }
    }
}

pub type Rate = usize;
// pub type BranchId(usize);

#[derive(Debug,Clone)]
pub struct VolcanoMap{
    start_room:usize,
    distances: Grid<Option<NonZeroU8>>,
    rates: Vec<Rate>,
}
impl VolcanoMap {
    pub fn contract(&mut self, valve_index:usize) {
        match self.start_room.cmp(&valve_index) {
            std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Greater => {
                self.start_room-=1;
            },
            std::cmp::Ordering::Less => {},
        }
        self.distances.remove_column(valve_index);
        self.distances.remove_row(valve_index);
    }
}

#[derive(Debug,Clone)]
pub struct InputStruct(VolcanoMap);
impl FromStr for InputStruct {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve_map,(rates,adjacenies)) = s.lines().enumerate().filter_map(|(valve_index,line)|{
            let (valve_info,tunnels_str) = line.split_once("; ")?;
            let value_name = valve_info[6..8].parse::<RoomName>().ok()?; // "Valve ..."
            let rate = valve_info[23..].parse::<Rate>().ok()?; // "Valve .. has flow rate="
            let tunnels = tunnels_str[23..].split(", "); // "Valve .. has flow rate="
            let valve_names:HashMap<_,_> = tunnels.map(|t|{
                t.parse::<RoomName>().map(|v|(v,1u8))
            }).collect::<Result<_,_>>().ok().or_else(||{
                tunnels_str[22..].split(", ").map(|t|{
                    t.parse::<RoomName>().map(|v|(v,1))
                }).collect::<Result<_,_>>().ok()
            })?;

            Some(((value_name,valve_index),(rate as Rate,valve_names)))
        }).unzip::<_,_,HashMap<_,_>,(Vec<_>,Vec<_>)>();

        let n = valve_map.len();
        let Some(mut dist_matrix) = Grid::from_parts(std::iter::repeat(None).take(n*n).collect(),n)else{
            return Err(());
        };
        for (room_index, adjacent_rooms) in adjacenies.into_iter().enumerate() {
            for (other, dist) in adjacent_rooms {
                let Some(&other_index) = valve_map.get(&other)else {
                    continue;
                };
                let dist = NonZeroU8::new(dist);
                dist_matrix[(other_index,room_index)]=dist;
                dist_matrix[(room_index,other_index)]=dist;
            }
        }
        for [k,i] in ArrayProduct::new([(0..n),(0..n)]) {
            for j in 1..i {
                let Some((i_to_k,k_to_j)) = dist_matrix.get((i,k)).zip(dist_matrix.get((k,j))).and_then(|(a,b)|a.zip(*b)) else {
                    continue;
                };
                let Some(new_dist) = i_to_k.checked_add(k_to_j.get()) else {
                    continue;
                };
                let Some(old_dist) = dist_matrix.get_mut((i,j)) else {
                    continue;
                };

                if let Some(old_dist) = old_dist {
                    if new_dist<*old_dist {
                        *old_dist = new_dist;
                    }
                }else{
                    *old_dist = Some(new_dist);
                }
            }
        }

        let Some(&start_room)=valve_map.get(&START_STATE)else{return Err(())};

        Ok(Self(VolcanoMap{
            rates,
            start_room,
            distances:dist_matrix
        }))
    }
}
const START_STATE:RoomName=RoomName(['A';2]);

pub struct Solution;
impl AOCSolution for Solution {
    type Input=InputStruct;
    type Part1=Unsolved;
    type Part2=Unsolved;
    type Err = ();
    fn solve(InputStruct(mut valve_map):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        // let mut valves_to_contract = (valve_map.rates.iter().enumerate()).filter_map(|(i,&r)|(r!=0 || i == valve_map.start_room).then_some(i));
        // for contracted_name in valves_to_contract {
        //     valve_map.contract(contracted_name);
        // }
        valve_rush::<1>(
            &valve_map
        );
        // let start_valve = valve_map.get(&START_STATE).cloned().ok_or(())?;

        // #[derive(Debug,Clone)]
        // pub struct State{ 
        //     current_valve_name: ValveName, 
        //     current_valve: Valve,
        //     current_valve_map: ValveMap,
        //     minutes_remaining:u8,
        //     current_flow_rate:usize,
        //     total_pressure_released:usize
        // }

        // println!("{:?}",&valve_map);

        // let mut queue:VecDeque<_> = [State{ 
        //     current_valve_name: START_STATE, 
        //     current_valve:start_valve, 
        //     current_valve_map: valve_map,
        //     minutes_remaining: 30, current_flow_rate: 0, total_pressure_released: 0
        // }].into_iter().collect();
        // let mut finished_states:Vec<_> = Vec::new();


        // while let Some(State {
        //     current_valve_name,
        //     current_valve,
        //     mut current_valve_map,
        //     minutes_remaining,
        //     current_flow_rate,
        //     total_pressure_released
        // }) = queue.pop_front() {
        //     // Add the 'Stay here until the end' state 
        //     let stationary_state = State {
        //         current_valve_name,
        //         current_valve: current_valve.clone(),
        //         current_valve_map:current_valve_map.clone(),
        //         minutes_remaining: 0,
        //         current_flow_rate,
        //         total_pressure_released:total_pressure_released+current_flow_rate*(minutes_remaining as usize)
        //     };
        //     println!("{:?}",&stationary_state);
        //     finished_states.push(stationary_state);

        //     // Contract the current valve away
        //     current_valve_map.contract(&current_valve_name); 
        //     let next_valves = current_valve.next_valves.iter()
        //         .filter_map(|(name,w)|{
        //             current_valve_map.get(&name).map(|n|(name,n.clone(),w))
        //         });
        
        //     for (&next_valve_name,next_valve,&travel_minutes) in next_valves {
        //         let time_to_travel_and_open = travel_minutes+1;
        //         if let Some(remaining_after_opening_next) = minutes_remaining.checked_sub(time_to_travel_and_open){
        //             queue.push_back(State {
        //                 current_flow_rate:current_flow_rate+next_valve.flow_rate,
        //                 current_valve:next_valve,
        //                 current_valve_name:next_valve_name,
        //                 current_valve_map:current_valve_map.clone(),
        //                 minutes_remaining:remaining_after_opening_next,
        //                 total_pressure_released: 
        //                 (time_to_travel_and_open as usize)*current_flow_rate+total_pressure_released
        //             });

        //         }else {
        //             finished_states.push(State {
        //                 current_valve:next_valve,
        //                 current_valve_name:next_valve_name,
        //                 current_valve_map:current_valve_map.clone(),
        //                 minutes_remaining:0,
        //                 current_flow_rate,
        //                 total_pressure_released: 
        //                 (minutes_remaining as usize)*current_flow_rate+total_pressure_released
        //             });
        //         }
        //     }
        //     println!("{:?}",finished_states.len())
        // };

        // dbg!(finished_states.iter().max_by_key(|s|{
        //     s.total_pressure_released
        // }));
        // println!("{:?}",&finished_states.len());
        // dbg!(valve_map.len());

        todo!()
    }
}

#[derive(Debug,Clone,Hash,PartialEq, Eq,PartialOrd, Ord)]
pub struct AgentState {
    valve_to_open: usize, 
    minutes_until_opened:u8
}
impl AgentState {
    // pub fn advance_action(&mut self, action:&Action,time:u8,valve_map:& VolcanoMap,closed_valves:&mut HashSet<RoomName>,flow_rate:&mut Rate) {
    //     match action {
    //         &Action::StartTowards(new_valve, distance) => {
    //             self.destination_valve_name=new_valve;
    //             self.minutes_to_reach= distance-time;
    //         },
    //         Action::ContinueToDestination => {
    //             self.minutes_to_reach-=time;
    //         },
    //         Action::OpenCurrentValve => {
    //             if closed_valves.remove(&self.destination_valve_name) {
    //                 if let Some(this_flow) = valve_map.get(&self.destination_valve_name) {
    //                     *flow_rate += this_flow.valve_flow_rate as Rate;
    //                 }
    //             }
    //         }
    //         // Action::Stay => {
                
    //         // },
    //     }
    // }
}

#[derive(Debug,Clone,Copy,PartialEq, Eq,PartialOrd, Ord,Hash)]
pub struct FinishedState {
    pressure_released:usize,
    flow_rate:usize,
}
#[derive(Debug,Clone, Eq,PartialEq)]
pub struct State<const N:usize> {
    agents: [AgentState;N],
    closed_valves: BitSet,
    minutes_remaining:u8,
    current_flow_rate:usize,
    total_pressure_released:usize,
}
impl <const N:usize> Hash for State<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.agents.hash(state);
        self.closed_valves.hash(state);
        // self.minutes_remaining.hash(state);
        // self.current_flow_rate.hash(state);
        self.total_pressure_released.hash(state);
    }
}

// fn non_false_bools<const N:usize>()->ArrayProduct<N, std::array::IntoIter<bool,2>>{
//     let mut values=  ArrayProduct::new([[false,true];N].map(IntoIterator::into_iter));
//     values.next(); // remove the all false, if it is present
//     values
// }


fn valve_rush<const N:usize>(volcano_map:&VolcanoMap)->Option<usize>{
    if N <= 0{
       return None;
    }
    let starting_minutes :u8 = 30-((N-1)*4) as u8;
    let mut closed_valves = BitSet::with_capacity(volcano_map.rates.len());
    closed_valves.extend(0..volcano_map.rates.len());
    closed_valves.remove(volcano_map.start_room);
    let start_row = volcano_map.distances.row(volcano_map.start_room)?;
    let mut queue:VecDeque<_> = ArrayProduct::new([();N].map(|_|start_row.iter().enumerate().filter_map(|(valve,dist)|dist.map(|d|(valve,d))))).filter_map(|a|{
        let is_ok = if N < 2 {
            true
        }else {
            let iters = a.iter().map(|state|state.0).collect::<HashSet<_>>();
            iters.len() == N
        };
        let mut agents = a.map(|(destination_valve,minutes_to_reach)|AgentState { valve_to_open: destination_valve, minutes_until_opened: minutes_to_reach.get() + 1 });
        agents.sort();
        is_ok.then_some(State::<N>{
            agents,
            minutes_remaining: starting_minutes,
            current_flow_rate: 0,
            total_pressure_released: 0,
            closed_valves:closed_valves.clone(),
        })
    }).collect();
    let mut max_pressure:Option<FinishedState> = None;

    let mut visited_states = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if !visited_states.insert(state.clone()) {
            continue;
        }
        let State { mut agents, mut closed_valves, mut minutes_remaining, mut current_flow_rate, mut total_pressure_released } = state;

        dbg!(minutes_remaining,queue.len());
        
        let Some(time_to_next_opened_valve) = agents.iter().map(|state|state.minutes_until_opened).min() else {
            continue;
        };
        if let Some(remaining_after_next_open) = minutes_remaining.checked_sub(time_to_next_opened_valve){
            for agent in &mut agents {
                agent.minutes_until_opened-=time_to_next_opened_valve;
            }
        
            let mut flow_rate_after_next_open = current_flow_rate;
            // Close all valves with an agent at them now
            for agent in agents.iter(){
                if agent.minutes_until_opened == 0 { // If an agent just opened their valve
                    closed_valves.remove(agent.valve_to_open);
                    flow_rate_after_next_open+=volcano_map.rates[agent.valve_to_open];
                }
            }

            let new_total_pressure_released=
                    (time_to_next_opened_valve as usize)*current_flow_rate+total_pressure_released;

            if closed_valves.is_empty() {
                let finished = FinishedState{
                    flow_rate:flow_rate_after_next_open,
                    pressure_released:(remaining_after_next_open as usize)*flow_rate_after_next_open+new_total_pressure_released
                };
                if let Some(max) = &mut max_pressure {
                    if max.pressure_released < finished.pressure_released {
                        *max = finished;
                    }
                }else{
                    max_pressure = Some(finished)
                }
                continue;
            }
            ;
            minutes_remaining = remaining_after_next_open;
            current_flow_rate = flow_rate_after_next_open;
            total_pressure_released = new_total_pressure_released;
        }else {
            let finished = FinishedState {flow_rate: current_flow_rate, pressure_released: total_pressure_released+(minutes_remaining as usize)*current_flow_rate };
            if let Some(max) = &mut max_pressure {
                if max.pressure_released < finished.pressure_released {
                    *max = finished;
                }
            }else{
                max_pressure = Some(finished)
            }
            continue;
        }

        let adjacents = agents.map(|a|{
            if a.minutes_until_opened == 0 {
                Or::Left(volcano_map.distances.row(a.valve_to_open)
                    .into_iter()
                    .flat_map(|r|r.iter().enumerate().filter_map(|(valve,distance)|{
                        if let Some(distance)=distance{
                            closed_valves.contains(valve).then_some(AgentState {
                                valve_to_open: valve,
                                minutes_until_opened: distance.get() +1
                            })
                        }else{
                            None
                        }
                    })))
            }else{
                Or::Right(Some(a))
            }.iter_collapsed()
        });
        // queue.push_back(State {
        //     agents:agents_after_actions,
        //     current_flow_rate:flow_rate_after_next_open,
        //     minutes_remaining:remaining_after_actions,
        //     closed_valves: valves_closed_after_actions,
        //     total_pressure_released
        // });

        let valid_combinations = ArrayProduct::new(adjacents).map(|mut n|{
            n.sort();
            n
        }).unique().filter(|a|{
            if N < 2 {
                true
            }else {
                let iters = a.iter().map(|state|state.minutes_until_opened).collect::<HashSet<_>>();
                iters.len() == N
            }
        });        
        queue.extend(valid_combinations.map(|agents|
            State {
                agents,
                current_flow_rate,
                minutes_remaining,
                closed_valves:closed_valves.clone(),
                total_pressure_released
            }
        ));
    };
    dbg!(max_pressure);
    // dbg!(valve_map.len());
    todo!()
}
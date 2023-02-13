use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
    iter::Sum,
};

#[inline]
fn zero<T: Sum>() -> T {
    std::iter::empty().sum()
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighNoneOption<T>(pub Option<T>);
impl<T: PartialOrd> PartialOrd for HighNoneOption<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.0, &other.0) {
            (Some(s), Some(o)) => s.partial_cmp(o),
            (Some(_), None) => Some(std::cmp::Ordering::Less),
            (None, Some(_)) => Some(std::cmp::Ordering::Greater),
            (None, None) => Some(std::cmp::Ordering::Equal),
        }
    }
}
impl<T: Ord> Ord for HighNoneOption<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.0, &other.0) {
            (Some(s), Some(o)) => s.cmp(o),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AStarDisance<W> {
    // discovered_distance: HighNoneOption<W>,
    discovered_distance: W,
    heuristic: W,
}
impl<W: Sum + Clone> AStarDisance<W> {
    // #[inline]
    // pub fn total_distance(&self)->HighNoneOption<W> {
    //     HighNoneOption(match &self.discovered_distance.0 {
    //         Some(s)=>Some([s.clone(),self.heuristic.clone()].into_iter().sum()),
    //         None=>None
    //     })
    // }
    #[inline]
    pub fn total_distance(&self) -> W {
        [self.discovered_distance.clone(), self.heuristic.clone()]
            .into_iter()
            .sum()
    }
}
impl<W: Sum + Clone + PartialOrd> PartialOrd for AStarDisance<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_distance().partial_cmp(&other.total_distance())
    }
}
impl<W: Sum + Clone + Ord> Ord for AStarDisance<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_distance().cmp(&other.total_distance())
    }
}

pub struct AStarState<S, W> {
    distance: AStarDisance<W>,
    state: S,
}
impl<S, W> PartialEq for AStarState<S, W>
where
    AStarDisance<W>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}
impl<S, W> Eq for AStarState<S, W> where AStarDisance<W>: Eq {}
impl<S, W> PartialOrd for AStarState<S, W>
where
    AStarDisance<W>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}
impl<S, W> Ord for AStarState<S, W>
where
    AStarDisance<W>: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub fn a_star<
    S: Hash + Eq + Clone + Debug,
    Ss: IntoIterator<Item = S>,
    W: Sum + Clone + Ord + Debug,
    N: Fn(S) -> Ns,
    Ns: IntoIterator<Item = (S, W)>,
    H: Fn(&S) -> W,
    C: Fn(&S) -> bool,
>(
    initial_states: Ss,
    get_neighbors: N,
    heuristic: H,
    is_complete: C,
) -> Option<Vec<S>> {
    let mut prev_state = HashMap::<S, S>::default();
    let mut discovered_distance = HashMap::<S, W>::default();
    let mut states_to_visit = BinaryHeap::<Reverse<AStarState<S, W>>>::default();
    for initial_state in initial_states {
        states_to_visit.push(Reverse(AStarState {
            distance: AStarDisance {
                discovered_distance: zero(),
                heuristic: heuristic(&initial_state),
            },
            state: initial_state.clone(),
        }));
        discovered_distance.insert(initial_state, zero());
    }
    while let Some(current_state) = states_to_visit.pop() {
        if is_complete(&current_state.0.state) {
            let mut path = vec![current_state.0.state.clone()];
            let mut path_curr = current_state.0.state;
            while let Some(prev) = prev_state.get(&path_curr) {
                // dbg!(prev);
                path.push(prev.clone());
                path_curr = prev.clone()
            }
            path.reverse();
            return Some(path);
        }
        for (neighbor, distance) in get_neighbors(current_state.0.state.clone()) {
            // println!("The distance from {:?} to neighbor {neighbor:?} would be {distance:?}",current_state.0.state);
            let tentative_discovered_distance: W = [
                current_state.0.distance.discovered_distance.clone(),
                distance,
            ]
            .into_iter()
            .sum();
            let new_shortest = match discovered_distance.entry(neighbor.clone()) {
                Entry::Occupied(mut o) => {
                    if &tentative_discovered_distance < o.get() {
                        o.insert(tentative_discovered_distance.clone());
                        true
                    } else {
                        false
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(tentative_discovered_distance.clone());
                    true
                }
            };
            if new_shortest {
                let mut neighbor_in_heap = false;
                // Alter the discovered distances in the heap
                states_to_visit = states_to_visit
                    .into_vec()
                    .into_iter()
                    .map(|mut s| {
                        if s.0.state == neighbor {
                            neighbor_in_heap = true;
                            s.0.distance.discovered_distance =
                                tentative_discovered_distance.clone();
                        }
                        s
                    })
                    .collect();
                if !neighbor_in_heap {
                    states_to_visit.push(Reverse(AStarState {
                        distance: AStarDisance {
                            discovered_distance: tentative_discovered_distance,
                            heuristic: heuristic(&neighbor),
                        },
                        state: neighbor.clone(),
                    }));
                }
                prev_state.insert(neighbor, current_state.0.state.clone());
            }
        }
    }
    None
}

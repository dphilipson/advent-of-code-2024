use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

// Copy/pasted from src/util/search/dijkstra.rs, but modified to track multiple
// previous indices.

pub type Distance = usize;

pub fn search<S, FNext, I, FGoal>(
    initial_state: S,
    get_next_states: FNext,
    is_goal: FGoal,
) -> SearchResult<S>
where
    S: Clone + Eq + Hash,
    FNext: Fn(&S) -> I,
    I: IntoIterator<Item = (S, Distance)>,
    FGoal: Fn(&S) -> bool,
{
    let (seen_states, reached_goal) = {
        let mut tracked_states = Vec::<TrackedState<S>>::new();
        let mut tracked_state_indices = HashMap::<S, usize>::new();
        let mut pending_states = BinaryHeap::<PendingState>::new();
        let mut seen_states = Vec::<SeenState<S>>::new();
        tracked_states.push(TrackedState {
            state: initial_state.clone(),
            distance: 0,
            prev_indices: HashSet::new(),
            seen_index: None,
        });
        tracked_state_indices.insert(initial_state, 0);
        pending_states.push(PendingState {
            distance: 0,
            index: 0,
        });
        let mut reached_goal = false;
        while let Some(PendingState { distance, index }) = pending_states.pop() {
            let TrackedState {
                state,
                distance: tracked_distance,
                prev_indices,
                ..
            } = tracked_states[index].clone();
            if distance > tracked_distance {
                continue;
            }
            seen_states.push(SeenState {
                state: state.clone(),
                distance,
                prev_indices: prev_indices
                    .into_iter()
                    .map(|i| tracked_states[i].seen_index.unwrap())
                    .collect(),
            });
            tracked_states[index].seen_index = Some(seen_states.len() - 1);
            if is_goal(&state) {
                reached_goal = true;
                break;
            }
            for (next_state, added_distance) in get_next_states(&state) {
                let next_distance = distance + added_distance;
                if let Some(&known_index) = tracked_state_indices.get(&next_state) {
                    let known_state = &mut tracked_states[known_index];
                    if next_distance < known_state.distance {
                        pending_states.push(PendingState {
                            distance: next_distance,
                            index: known_index,
                        });
                        known_state.distance = next_distance;
                        known_state.prev_indices = HashSet::from_iter([index]);
                    } else if next_distance == known_state.distance {
                        known_state.prev_indices.insert(index);
                    }
                } else {
                    tracked_states.push(TrackedState {
                        state: next_state.clone(),
                        distance: next_distance,
                        prev_indices: HashSet::from_iter([index]),
                        seen_index: None,
                    });
                    let next_index = tracked_states.len() - 1;
                    tracked_state_indices.insert(next_state, next_index);
                    pending_states.push(PendingState {
                        distance: next_distance,
                        index: next_index,
                    });
                }
            }
        }
        (seen_states, reached_goal)
    };
    SearchResult {
        seen_states,
        reached_goal,
    }
}

#[derive(Debug)]
pub struct SearchResult<S> {
    pub seen_states: Vec<SeenState<S>>,
    reached_goal: bool,
}

#[derive(Debug)]
pub struct SeenState<S> {
    pub state: S,
    pub distance: Distance,
    pub prev_indices: HashSet<usize>,
}

#[derive(Clone, Debug)]
struct TrackedState<S> {
    state: S,
    distance: Distance,
    prev_indices: HashSet<usize>,
    seen_index: Option<usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct PendingState {
    index: usize,
    distance: Distance,
}

impl Ord for PendingState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl PartialOrd for PendingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

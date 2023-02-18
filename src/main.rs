use std::collections::{HashMap, HashSet};

const LOC_1: &str = "1";
const LOC_2: &str = "2";
const LOC_3: &str = "3";
const LOC_4: &str = "4";
const LOC_5: &str = "5";
const LOC_6: &str = "6";
const LOC_M: &str = "M";
const MAX_DEPTH: usize = 10;

#[derive(Clone, Debug)]
struct GraphNode {
    location: String,       // TODO: should be &str
    data: Option<String>,   // If none it's an empty node
    connected: Vec<String>, // locations
}

impl GraphNode {
    fn is_placed(&self) -> bool {
        if let Some(inner) = self.data.as_deref() {
            return inner == self.location;
        } else if self.location == LOC_M {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    from: String,
    to: String,
}

#[derive(Debug, Clone, Default)]
struct State {
    map: HashMap<&'static str, GraphNode>,
    moves: Vec<Move>,
}

impl State {
    fn is_solved(&self) -> bool {
        self.map.values().all(|node| node.is_placed())
    }

    fn print(&self) {
        // TODO: Draw it nicely :)
        println!("{{");
        self.map
            .values()
            .for_each(|node| println!("\t{:?}: {:?}", node.location, node.data));
        println!("}}");
    }

    fn move_possibilities(&self) -> Vec<Move> {
        let empty = self.map.values().find(|v| v.data.is_none()).unwrap();

        empty
            .connected
            .iter()
            .map(|from| Move {
                from: from.clone(),
                to: empty.location.clone(),
            })
            .collect()
    }

    fn move_data(&self, m: Move) -> Result<State, &'static str> {
        let mut new_state = self.clone();

        let from_node = new_state.map.get_mut(m.from.as_str()).unwrap(); // TODO: convert to err?

        let mut moveable: String = "".to_string();

        if let Some(data) = from_node.data.as_ref() {
            moveable = data.clone();
            from_node.data = None;
        } else {
            Err("no data in from")?
        }

        let to_node = new_state.map.get_mut(m.to.as_str()).unwrap(); // TODO: convert to err?

        to_node.data = Some(moveable);
        new_state.moves.push(m);

        Ok(new_state)
    }

    fn hash(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            data_hash(self.map[LOC_1].data.as_ref()),
            data_hash(self.map[LOC_2].data.as_ref()),
            data_hash(self.map[LOC_3].data.as_ref()),
            data_hash(self.map[LOC_4].data.as_ref()),
            data_hash(self.map[LOC_5].data.as_ref()),
            data_hash(self.map[LOC_6].data.as_ref()),
            data_hash(self.map[LOC_M].data.as_ref())
        )
    }
}

fn data_hash(data: Option<&String>) -> String {
    if let Some(inner) = data {
        return inner.clone();
    }
    "0".to_string()
}

fn load_map(map: HashMap<&'static str, &'static str>) -> State {
    let mut initial_state = HashMap::new();

    // Create Nodes
    let middle_node = GraphNode {
        location: LOC_M.to_string(),
        data: None,
        connected: vec![LOC_1, LOC_2, LOC_3, LOC_4, LOC_5, LOC_6]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    initial_state.insert(LOC_M, middle_node);

    for (k, v) in map.into_iter() {
        let node = GraphNode {
            location: k.to_string(),
            data: Some(v.to_string()),
            connected: vec![],
        };
        initial_state.insert(k, node);
    }

    // Connect Nodes
    for i in 1..7 {
        let node = initial_state.get_mut(i.to_string().as_str()).unwrap();

        let up_neighbor = i / 6 + (i + 1) % 7;
        let mut down_neighbor = (i - 1) % 6;
        if down_neighbor == 0 {
            // This could have been math but I'm too tired to figure it out
            down_neighbor = 6;
        }

        node.connected.extend_from_slice(&[
            up_neighbor.to_string(),
            down_neighbor.to_string(),
            LOC_M.to_string(),
        ])
    }

    State {
        map: initial_state,
        moves: vec![],
    }
}

fn solve(state: &State, past_states: HashSet<String>) -> Result<State, String> {
    // println!("solve depth: {}, moves: {:?}", moves.len(), moves);
    // print_state(state);
    // println!("{:?}", past_states);
    // println!("---------");

    if state.is_solved() {
        println!("found a solution in {} moves!", state.moves.len());
        return Ok(state.clone());
    }

    if state.moves.len() >= MAX_DEPTH {
        return Err("max depth".to_string());
    }

    let possibilitties = state.move_possibilities();
    // println!("possible moves: {:?}", possibilitties);

    let solutions: Vec<State> = possibilitties
        .iter()
        .filter_map(|m| {
            let result = state.move_data(m.clone());
            if let Ok(new_state) = result {
                let hash = state.hash();
                if past_states.contains(&hash) {
                    return None;
                }

                let mut new_past_states = past_states.clone();
                new_past_states.insert(hash);
                solve(&new_state, new_past_states).ok()
            } else {
                None
            }
        })
        .collect();

    if solutions.is_empty() {
        Err("no solutions found :(")?
    }

    let mut min = usize::MAX;
    let mut curr_solution: &State = &State::default();

    for solution in solutions.iter() {
        if solution.moves.len() < min {
            min = solution.moves.len();
            curr_solution = solution;
        }
    }
    Ok(curr_solution.clone())
}

fn main() {
    let mut starting_position = HashMap::new();
    // TODO: read from json
    starting_position.insert(LOC_1, LOC_2);
    starting_position.insert(LOC_2, LOC_1);
    starting_position.insert(LOC_3, LOC_3);
    starting_position.insert(LOC_4, LOC_5);
    starting_position.insert(LOC_5, LOC_4);
    starting_position.insert(LOC_6, LOC_6);

    let state = load_map(starting_position);

    // print_state(&state);
    // println!("is solved: {}", is_solved(&state));

    match solve(&state, HashSet::new()) {
        // TODO: print moves & intermediate states?
        Ok(solved_state) => {
            println!("solved in {} moves!", solved_state.moves.len());
            solved_state.print();
            println!("Moves: {:?}", solved_state.moves);
        }
        Err(e) => println!("failed! {:?}", e),
    }
}

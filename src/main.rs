use std::collections::{HashMap, HashSet};

const LOC_1: &str = "1";
const LOC_2: &str = "2";
const LOC_3: &str = "3";
const LOC_4: &str = "4";
const LOC_5: &str = "5";
const LOC_6: &str = "6";
const LOC_M: &str = "M";
const MAX_DEPTH: usize = 10;

#[derive(Clone)]
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

impl Move {
    fn backwards_move(&self) -> Move {
        Move {
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }
}

fn data_hash(data: Option<&String>) -> String {
    if let Some(inner) = data {
        return inner.clone();
    }
    "0".to_string()
}

fn load_map<'a>(map: HashMap<&'a str, &'a str>) -> HashMap<&'a str, GraphNode> {
    let mut result = HashMap::new();

    // Create Nodes
    let middle_node = GraphNode {
        location: LOC_M.to_string(),
        data: None,
        connected: vec![LOC_1, LOC_2, LOC_3, LOC_4, LOC_5, LOC_6]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    result.insert(LOC_M, middle_node);

    for (k, v) in map.into_iter() {
        let node = GraphNode {
            location: k.to_string(),
            data: Some(v.to_string()),
            connected: vec![],
        };
        result.insert(k, node);
    }

    // Connect Nodes
    for i in 1..7 {
        let node = result.get_mut(i.to_string().as_str()).unwrap();

        let up_neighbor = i / 6 + (i + 1) % 7;
        let mut down_neighbor = (i - 1) % 6;
        if down_neighbor == 0 {
            down_neighbor = 6;
        }

        node.connected.extend_from_slice(&[
            up_neighbor.to_string(),
            down_neighbor.to_string(),
            LOC_M.to_string(),
        ])
    }

    result
}

fn is_solved(state: &HashMap<&str, GraphNode>) -> bool {
    state.values().all(|node| node.is_placed())
}

fn state_hash(state: &HashMap<&str, GraphNode>) -> String {
    format!(
        "{}{}{}{}{}{}{}",
        data_hash(state[LOC_1].data.as_ref()),
        data_hash(state[LOC_2].data.as_ref()),
        data_hash(state[LOC_3].data.as_ref()),
        data_hash(state[LOC_4].data.as_ref()),
        data_hash(state[LOC_5].data.as_ref()),
        data_hash(state[LOC_6].data.as_ref()),
        data_hash(state[LOC_M].data.as_ref())
    )
}

fn move_data(
    state: &HashMap<&'static str, GraphNode>,
    m: Move,
) -> Result<HashMap<&'static str, GraphNode>, &'static str> {
    let mut new_state = (*state).clone();

    let from_node = new_state.get_mut(m.from.as_str()).unwrap();

    let mut moveable: String = "".to_string();

    if let Some(data) = from_node.data.as_ref() {
        moveable = data.clone();
        from_node.data = None;
    } else {
        Err("no data in from")?
    }

    let to_node = new_state.get_mut(m.to.as_str()).unwrap();

    to_node.data = Some(moveable);

    Ok(new_state)
}

fn move_possibilities(
    state: &HashMap<&'static str, GraphNode>,
    last_move: Option<Move>,
) -> Vec<Move> {
    let empty = state.values().find(|v| v.data.is_none()).unwrap();

    let forbidden_move = last_move.map(|m| m.backwards_move());

    empty
        .connected
        .iter()
        .map(|from| Move {
            from: from.clone(),
            to: empty.location.clone(),
        })
        .filter(|m| Some(m) != forbidden_move.as_ref())
        .collect()
}

fn print_state(state: &HashMap<&str, GraphNode>) {
    // TODO: Draw it nicely :)
    println!("{{");
    state
        .values()
        .for_each(|node| println!("\t{:?}: {:?}", node.location, node.data));
    println!("}}");
}

fn solve(
    state: &HashMap<&'static str, GraphNode>,
    moves: Vec<Move>,
    past_states: HashSet<String>,
) -> Result<(HashMap<&'static str, GraphNode>, Vec<Move>), String> {
    // println!("solve depth: {}, moves: {:?}", moves.len(), moves);
    // print_state(state);
    // println!("{:?}", past_states);
    // println!("---------");

    if is_solved(state) {
        println!("found a solution in {} moves!", moves.len());
        return Ok((state.clone(), moves));
    }

    if moves.len() >= MAX_DEPTH {
        return Err("max depth".to_string());
    }

    // TODO: need to remove back-and-forths
    let last_move = if moves.is_empty() {
        None
    } else {
        Some(moves[moves.len() - 1].clone())
    };

    let possibilitties = move_possibilities(state, last_move);
    // println!("possible moves: {:?}", possibilitties);

    let solutions: Vec<(HashMap<&str, GraphNode>, Vec<Move>)> = possibilitties
        .iter()
        .filter_map(|m| {
            let result = move_data(state, m.clone());
            if let Ok(new_state) = result {
                let hash = state_hash(state);
                if past_states.contains(&hash) {
                    return None;
                }
                let mut new_moves = moves.clone();
                new_moves.push(m.clone());
                let mut new_past_states = past_states.clone();
                new_past_states.insert(hash);
                solve(&new_state, new_moves, new_past_states).ok()
            } else {
                None
            }
        })
        .collect();

    // for (new_state, mv, new_hashes) in possibilitties.iter().filter_map(|m| {
    //     if let Ok(new_state) = move_data(state, m.clone()) {
    //         let hash = state_hash(state);
    //         if past_states.contains(&hash) {
    //             return None;
    //         }
    //         let mut new_past_states = past_states.clone();
    //         new_past_states.insert(hash);
    //         Some((new_state, m.clone(), new_past_states))
    //     } else {
    //         None
    //     }
    // }) {
    //     let mut new_moves = moves.clone();
    //     new_moves.push(mv);

    //     if let Ok((solution, good_moves)) = solve(&new_state, new_moves, new_hashes) {
    //         return Ok((solution, good_moves));
    //     }
    // }

    if solutions.is_empty() {
        Err("no solutions found :(")?
    }

    let mut min = usize::MAX;
    let mut curr_solution: (HashMap<&str, GraphNode>, Vec<Move>) = (HashMap::new(), vec![]);

    for (sol, moves) in solutions.iter() {
        if moves.len() < min {
            min = moves.len();
            curr_solution = (sol.clone(), moves.clone());
        }
    }
    Ok(curr_solution)
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

    match solve(&state, vec![], HashSet::new()) {
        // TODO: print moves & intermediate states?
        Ok((solved_state, moves)) => {
            println!("solved in {} moves!", moves.len());
            print_state(&solved_state);
            println!("Moves: {:?}", moves);
        }
        Err(e) => println!("failed! {:?}", e),
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

const LOC_1: &str = "1";
const LOC_2: &str = "2";
const LOC_3: &str = "3";
const LOC_4: &str = "4";
const LOC_5: &str = "5";
const LOC_6: &str = "6";
const LOC_M: &str = "M";
const MAX_DEPTH: usize = 64;

#[derive(Clone, Debug)]
struct GraphNode {
    location: &'static str,
    data: Option<&'static str>,   // If none it's an empty node
    connected: Vec<&'static str>, // what locations this node is connected to
}

impl GraphNode {
    fn is_placed(&self) -> bool {
        if let Some(inner) = self.data {
            return inner == self.location;
        } else if self.location == LOC_M {
            return true;
        }
        false
    }

    fn data_hash(&self) -> &'static str {
        self.data.unwrap_or("0")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    from: &'static str,
    to: &'static str,
}

#[derive(Debug, Clone, Default)]
struct State {
    map: HashMap<&'static str, GraphNode>,
    moves: Vec<Move>,
    past_states: HashSet<String>,
}

impl State {
    fn is_solved(&self) -> bool {
        self.map.values().all(|node| node.is_placed())
    }

    fn is_max_depth(&self) -> bool {
        self.moves.len() >= MAX_DEPTH
    }

    fn print(&self) {
        // TODO: Draw it nicely :)
        println!("{{");
        self.map
            .values()
            .for_each(|node| println!("\t{:?}: {:?}", node.location, node.data));
        println!("}}");
    }

    fn available_moves(&self) -> Vec<Move> {
        let empty = self.map.values().find(|v| v.data.is_none()).unwrap();

        empty
            .connected
            .iter()
            .map(|from| Move {
                from,
                to: empty.location,
            })
            .collect()
    }

    fn move_data(&self, m: &Move) -> Result<State, &'static str> {
        let mut new_state = self.clone();

        let from_node = new_state.map.get_mut(m.from).unwrap(); // TODO: convert to err?

        let mut moveable: &'static str = "";

        if let Some(data) = from_node.data.as_ref() {
            moveable = data;
            from_node.data = None;
        } else {
            Err("no data in from")?
        }

        let to_node = new_state.map.get_mut(m.to).unwrap(); // TODO: convert to err?

        to_node.data = Some(moveable);
        new_state.moves.push(m.clone());

        let state_hash = new_state.hash();
        if new_state.past_states.contains(&state_hash) {
            Err("circular, path, aborting")?
        }

        new_state.past_states.insert(state_hash);
        Ok(new_state)
    }

    fn hash(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            self.map[LOC_1].data_hash(),
            self.map[LOC_2].data_hash(),
            self.map[LOC_3].data_hash(),
            self.map[LOC_4].data_hash(),
            self.map[LOC_5].data_hash(),
            self.map[LOC_6].data_hash(),
            self.map[LOC_M].data_hash()
        )
    }
}

// TODO: consider load M form map, to accommodate a non-initial state.
fn load_map(map: HashMap<&'static str, &'static str>) -> State {
    let mut initial_state = HashMap::new();

    // Create Nodes
    let middle_node = GraphNode {
        location: LOC_M,
        data: None,
        connected: vec![LOC_1, LOC_2, LOC_3, LOC_4, LOC_5, LOC_6],
    };
    initial_state.insert(LOC_M, middle_node);

    for (k, v) in map.into_iter() {
        let node = GraphNode {
            location: k,
            data: Some(v),
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
            // This could have been math but I'm too tired to figure it out :(
            down_neighbor = 6;
        }

        node.connected.extend_from_slice(&[
            to_predefined_str(up_neighbor).unwrap(),
            to_predefined_str(down_neighbor).unwrap(),
            LOC_M,
        ])
    }

    State {
        map: initial_state,
        moves: vec![],
        past_states: HashSet::new(),
    }
}

// Does this suck as much as I think it does??
fn to_predefined_str(x: i32) -> Result<&'static str, ()> {
    match x {
        1 => Ok(LOC_1),
        2 => Ok(LOC_2),
        3 => Ok(LOC_3),
        4 => Ok(LOC_4),
        5 => Ok(LOC_5),
        6 => Ok(LOC_6),
        _ => Err(()),
    }
}

fn solve_bfs(initial_state: State) -> Result<State, &'static str> {
    let mut states_pool = VecDeque::from([initial_state]);

    while let Some(current_state) = states_pool.pop_front() {
        if current_state.is_solved() {
            return Ok(current_state);
        }

        if current_state.is_max_depth() {
            continue;
        }

        let new_states: Vec<State> = current_state
            .available_moves()
            .iter()
            .filter_map(|m| current_state.move_data(m).ok())
            .collect();
        states_pool.extend(new_states.into_iter());
    }

    Err("failed finding a solution")
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

    match solve_bfs(state) {
        Ok(solved_state) => {
            println!("solved in {} moves!", solved_state.moves.len());
            solved_state.print();
            println!("Moves: {:?}", solved_state.moves);
        }
        Err(e) => println!("failed! {:?}", e),
    }
}

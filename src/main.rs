use std::collections::{HashMap, VecDeque};
use std::io::{stdin, BufRead, BufReader};
use std::str::SplitWhitespace;
use std::usize;

struct Input<B> {
    inner: B,
    buffer: String,
}

impl<B: BufRead> Input<B> {
    pub fn new(inner: B) -> Input<B> {
        Self {
            inner,
            buffer: String::new(),
        }
    }

    pub fn line(&mut self) -> Line {
        self.buffer.clear();
        self.inner.read_line(&mut self.buffer).unwrap();
        Line {
            split: self.buffer.split_whitespace(),
        }
    }

    pub fn string(&mut self) -> String {
        self.buffer.clear();
        self.inner.read_line(&mut self.buffer).unwrap();
        self.buffer.trim().to_string()
    }

    pub fn skip(&mut self) {
        let _ = self.inner.read_line(&mut self.buffer);
    }

    fn parse<T: std::str::FromStr>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.string().trim().parse::<T>().unwrap()
    }
}

struct Line<'a> {
    split: SplitWhitespace<'a>,
}

impl<'a> Line<'a> {
    fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.split.next().unwrap().parse::<T>().unwrap()
    }

    fn pair<T: std::str::FromStr>(&mut self) -> (T, T)
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (self.next(), self.next())
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct Vertex{
    current_turn: usize,
    actor_name: usize,
    score: i32,
}

struct LookupTable{
    table: Vec<Vec<Vertex>>
}

impl LookupTable{
    fn add_edge(&mut self, u: usize, v: &Vertex) {
        // Check if the table has enough capacity for index `u`
        if self.table.len() <= u {
            self.table.resize(u + 1, Vec::new());
        }

        // Check for the matching vertex and update its score
        let mut found = false;
        for g in &mut self.table[u] {
            if g.current_turn == v.current_turn && g.actor_name == v.actor_name {
                g.score += 1;
                found = true;
                break;
            }
        }

        // If no match was found, insert a new vertex
        if !found {
            let new_input = Vertex {
                current_turn: v.current_turn.clone(),
                actor_name: v.actor_name.clone(),
                score: 1,
            };
            self.table[u].push(new_input);
        }
    }
}

//Looks if a possibility was already said in the game
fn is_available(vertex: &Vertex, list: &Vec<usize>) -> bool {
    return !list.contains(&vertex.actor_name);
}

fn print_choice(list: &HashMap<String, usize>, search_value: usize){
    let choiche = list.iter().find(|(_, &value)| value == search_value).map(|(key, _)| key);
    if let Some(choiche) = choiche {
        println!("{}", choiche);
    } else {
        println!("No key found with value {}", search_value);
    }
}

//Calculate the maximum score if you win in the first turn.
fn maximum_score(lookup_table: LookupTable, n: usize) -> i32{
    let mut g = 0;
    let mut max_score = 0;
    while g <= 2 * n{
        for h in &lookup_table.table[g]{
            if h.score > max_score{
                max_score = h.score;
            }
        }
        g += 1;
    }
    return max_score;
}

//Returns our next turn based on the opponents turn.
fn our_next_move(lookup: &LookupTable, already_said: &Vec<usize>, list: &HashMap<String, usize>, opponents_next_move: &String) -> Vertex{
    let mut opponent_move: Vertex = Vertex { current_turn: 0, actor_name: 0, score: 0 };
    let j = list.get(opponents_next_move);
    if let Some(&j) = j{
        for h in &lookup.table[j]{
            if h.score > opponent_move.score && is_available(&h, &already_said){
                opponent_move.actor_name = h.actor_name;
                opponent_move.score = h.score;
                opponent_move.current_turn = j;
            }
        }
    }
    return opponent_move;
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (n, m) = input.line().pair::<usize>();

    let actresses = (1..=n)
        .map(|i| (input.string(), i))
        .collect::<HashMap<String, usize>>();

    let actors = (n + 1..=n + n)
        .map(|i| (input.string(), i))
        .collect::<HashMap<String, usize>>();

    let mut lookup = LookupTable { table: Vec::new() };

    for _ in 0..m {
        input.skip();
        let cast_size: usize = input.parse();
        let cast: Vec<String> = (0..cast_size)
            .map(|_| input.string())
            .collect();

        let mut actress_indices = vec![];
        let mut actor_indices = vec![];

        for cast_member in &cast {
            if let Some(&index) = actresses.get(cast_member) {
                actress_indices.push(index);
            } else if let Some(&index) = actors.get(cast_member) {
                actor_indices.push(index);
            }
        }

        for &actress_index in &actress_indices {
            for &actor_index in &actor_indices {
                let new_edge1: Vertex = Vertex { current_turn: actress_index, actor_name: actor_index, score: 1 };
                let new_edge2: Vertex = Vertex { current_turn: actor_index, actor_name: actress_index, score: 1 };
                lookup.add_edge(actress_index, &new_edge1);
                lookup.add_edge(actor_index, &new_edge2);

            }
        }
    }

    let player = input.string();
    let mut your_score = 0;
    let mut already_said : Vec<usize> =vec![];
    let mut turn = 0;
    let mut winning = true;

    if player == "Mark" {
        let mut opponents_next_move = input.string();
        
        while opponents_next_move != "IGiveUp" && winning{
            let our_next_move: Vertex = our_next_move(&lookup, &already_said, &actresses, &opponents_next_move);
            if our_next_move.actor_name == 0{
                println!("IGiveUp");
                winning = false;
            }else{
                print_choice(&actors, our_next_move.actor_name);
                already_said.push(our_next_move.actor_name);
                turn += 1;
                your_score = your_score + our_next_move.score;
                opponents_next_move = input.string();
            }
        }
    }else {
        let mut g = 0;
        let mut first_move: Vertex = Vertex { current_turn: 0, actor_name: 0, score: 9999999 };
        
        while g < n{
            for h in &lookup.table[g]{
                if h.score < first_move.score{
                    first_move.actor_name = h.actor_name;
                    first_move.score = h.score;
                    first_move.current_turn = g;
                }
            }
            g += 1;
        }
        print_choice(&actresses, first_move.current_turn);
        already_said.push(first_move.current_turn);
        let mut opponents_next_move = input.string();
        
        while opponents_next_move != "IGiveUp" && winning{
            let our_next_move: Vertex = our_next_move(&lookup, &already_said, &actors, &opponents_next_move);
            if our_next_move.actor_name == 0{
                println!("IGiveUp");
                winning = false;
            }else{
                print_choice(&actresses, our_next_move.actor_name);
                already_said.push(our_next_move.actor_name);
                turn += 1;
                your_score = your_score + our_next_move.score;
                opponents_next_move = input.string();
            }
        } 
    }

    if winning {
        if turn == 0 {
            println!("Final score is {}", maximum_score(lookup, n));
        }else{
            println!("Final score is {}", your_score / turn);
        }
    }else{
        println!("Final score is 0");
    }
}
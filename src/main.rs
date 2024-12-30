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

const NIL: usize = 0; // Using 0 as NIL to represent unmatched nodes
const INF: usize = usize::MAX;

struct BipartiteGraph {
    adj_list: Vec<Vec<usize>>,
}

impl BipartiteGraph {
    fn with_capacity(n: usize) -> BipartiteGraph {
        BipartiteGraph {
            adj_list: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
    }
}

struct HopcroftKarp {
    pair_u: Vec<usize>,
    pair_v: Vec<usize>,
    dist: Vec<usize>,
}


impl HopcroftKarp {
    fn with_capacity(n: usize) -> Self {
        HopcroftKarp {
            // + 1 for later appended the sink
            pair_u: vec![NIL; n + 1],
            pair_v: vec![NIL; n + 1],
            dist: vec![INF; n + 1],
        }
    }

    fn bfs(&mut self, graph: &BipartiteGraph) -> bool {
        let mut queue: VecDeque<usize> = VecDeque::new();

        //Since we can assume left and right are identical otherwise use std:cmp::min(self.pair_u.len(), self.pair_v.len())
        for u in 1..self.pair_u.len() {
            if self.pair_u[u] == NIL {
                self.dist[u] = 0;
                queue.push_back(u);
            } else {
                self.dist[u] = INF;
            }
        }

        self.dist[NIL] = INF;

        while let Some(u) = queue.pop_front() {
            if self.dist[u] < self.dist[NIL] {
                for &v in &graph.adj_list[u] {
                    let pair_v = self.pair_v[v];
                    if self.dist[pair_v] == INF {
                        self.dist[pair_v] = self.dist[u] + 1;
                        queue.push_back(pair_v);
                    }
                }
            }
        }

        self.dist[NIL] != INF
    }


    fn dfs(&mut self, u: usize, graph: &BipartiteGraph) -> bool {
        if u != NIL {
            for v in &graph.adj_list[u] {
                let pair_v = self.pair_v[*v];
                if self.dist[pair_v] == self.dist[u] + 1 {
                    if self.dfs(pair_v, graph) {
                        self.pair_v[*v] = u;
                        self.pair_u[u] = *v;
                        return true;
                    }
                }
            }
            self.dist[u] = INF;
            return false;
        }
        true
    }

    pub fn maximum_matching(&mut self, graph: &BipartiteGraph) -> usize {
        let mut result = 0;

        while self.bfs(graph) {
            for u in 1..self.pair_u.len() {
                if self.pair_u[u] == NIL && self.dfs(u, graph) {
                    result += 1;
                }
            }
        }
        result
    }
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

    let mut graph = BipartiteGraph::with_capacity(n + n + 1); // Ensuring the graph can hold all nodes
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
                graph.add_edge(actress_index, actor_index);
                let new_edge1: Vertex = Vertex { current_turn: actress_index, actor_name: actor_index, score: 1 };
                let new_edge2: Vertex = Vertex { current_turn: actor_index, actor_name: actress_index, score: 1 };
                lookup.add_edge(actress_index, &new_edge1);
                lookup.add_edge(actor_index, &new_edge2);

            }
        }
    }
    let mut hopcroft_karp = HopcroftKarp::with_capacity(n + n);
    if hopcroft_karp.maximum_matching(&graph) == n {
        //println!("Mark");
    } else {
       // println!("Veronique");
    }

    let player = input.string();
    let mut your_score = 0;
    let mut oponent_score = 0;
    println!("{}", player);
    if player == "Mark" {
        let mut next_move = input.string();
        let mut already_said : Vec<usize> =vec![];
        let mut turn = 0;
        let mut winning = true;
        while next_move != "IGiveUp" {
            let mut opponent_move: Vertex = Vertex { current_turn: 0, actor_name: 0, score: 0 };
            let j = actresses.get(&next_move);
            println!("{:?}", j);
            if let Some(&j) = j{
                for h in &lookup.table[j]{
                    if h.score > opponent_move.score && is_available(&h, &already_said){
                        opponent_move.actor_name = h.actor_name;
                        opponent_move.score = h.score;
                        opponent_move.current_turn = j;
                    }
                }
            }
            if opponent_move.actor_name == 0{
                println!("IGiveUp");
                winning = false;
            }else{
                let choiche = actors.iter().find(|(_, &value)| value == opponent_move.actor_name).map(|(key, _)| key);
                if let Some(choiche) = choiche {
                    println!("{}", choiche);
                } else {
                    println!("No key found with value {}", opponent_move.actor_name);
                }
                already_said.push(opponent_move.actor_name);
                turn += 1;
                your_score = your_score + opponent_move.score;
                next_move = input.string();
            }
        } 
        if winning {
            println!("Final score is {}", your_score / turn);
        }
    }else {
        let mut g = 0;
        let mut max_score: Vertex = Vertex { current_turn: 0, actor_name: 0, score: 9999999 };
        while g < n{
            for h in &lookup.table[g]{
                if h.score < max_score.score{
                    max_score.actor_name = h.actor_name;
                    max_score.score = h.score;
                    max_score.current_turn = g;
                }
            }
            g += 1;
        }


        let key = actresses.iter().find(|(_, &value)| value == max_score.current_turn).map(|(key, _)| key);

        if let Some(key) = key {
            println!("{}", key);
        } else {
            println!("No key found with value {}", max_score.current_turn);
        }
        let mut already_said : Vec<usize> =vec![];
        already_said.push(max_score.current_turn);

        let mut turn = 0;
        let mut next_move = input.string();
        let mut winning = true;
        while next_move != "IGiveUp" {
            let mut opponent_move: Vertex = Vertex { current_turn: 0, actor_name: 0, score: 0 };
            let j = actors.get(&next_move);
            println!("{:?}", j);
            if let Some(&j) = j{
                for h in &lookup.table[j]{
                    if h.score > opponent_move.score && is_available(&h, &already_said){
                        opponent_move.actor_name = h.actor_name;
                        opponent_move.score = h.score;
                        opponent_move.current_turn = j;
                    }
                }
            }
            if opponent_move.actor_name == 0{
                println!("IGiveUp");
                winning = false;
            }else{
                let choiche = actresses.iter().find(|(_, &value)| value == opponent_move.actor_name).map(|(key, _)| key);
                if let Some(choiche) = choiche {
                    println!("{}", choiche);
                } else {
                    println!("No key found with value {}", opponent_move.actor_name);
                }
                already_said.push(opponent_move.actor_name);
                turn += 1;
                your_score = your_score + opponent_move.score;
                next_move = input.string();
            }
        } 
        if winning {
            println!("Final score is {}", your_score / turn);
        }
    }
    
}

fn is_available(vertex: &Vertex, list: &Vec<usize>) -> bool {
    return !list.contains(&vertex.actor_name);
}

fn next_move(matrix: LookupTable, your_score: i32, opponent_score: i32, current_move: usize){
    matrix;
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



fn test() {
    println!("test");
}
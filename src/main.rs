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
    const NIL: usize = 0;
    const INF: usize = usize::MAX;

    fn with_capacity(n: usize) -> Self {
        HopcroftKarp {
            // + 1 for later appended the sink
            pair_u: vec![Self::NIL; n + 1],
            pair_v: vec![Self::NIL; n + 1],
            dist: vec![Self::INF; n + 1],
        }
    }

    fn bfs(&mut self, graph: &BipartiteGraph) -> bool {
        let mut queue: VecDeque<usize> = VecDeque::new();

        //Since we can assume left and right are identical otherwise use std:cmp::min(self.pair_u.len(), self.pair_v.len())
        for u in 1..self.pair_u.len() {
            if self.pair_u[u] == Self::NIL {
                self.dist[u] = 0;
                queue.push_back(u);
            } else {
                self.dist[u] = Self::INF;
            }
        }

        self.dist[Self::NIL] = Self::INF;

        while let Some(u) = queue.pop_front() {
            if self.dist[u] < self.dist[Self::NIL] {
                for &v in &graph.adj_list[u] {
                    let pair_v = self.pair_v[v];
                    if self.dist[pair_v] == Self::INF {
                        self.dist[pair_v] = self.dist[u] + 1;
                        queue.push_back(pair_v);
                    }
                }
            }
        }

        self.dist[Self::NIL] != Self::INF
    }

    fn dfs(&mut self, u: usize, graph: &BipartiteGraph) -> bool {
        if u != Self::NIL {
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
            self.dist[u] = Self::INF;
            return false;
        }
        true
    }

    pub fn maximum_matching(&mut self, graph: &BipartiteGraph) -> usize {
        let mut result = 0;

        while self.bfs(graph) {
            for u in 1..self.pair_u.len() {
                if self.pair_u[u] == Self::NIL && self.dfs(u, graph) {
                    result += 1;
                }
            }
        }
        result
    }

    fn perfect_match(&mut self, graph: &BipartiteGraph) -> bool {
        self.maximum_matching(graph) == self.pair_u.len() - 1
    }
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (n, m) = input.line().pair::<usize>();

    let actresses = (1..=n)
        .map(|i| (input.string(), i))
        .collect::<HashMap<String, usize>>();

    let actors = (1..=n)
        .map(|i| (input.string(), i))
        .collect::<HashMap<String, usize>>();

    let mut graph = BipartiteGraph::with_capacity(n + n);

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
            }
        }
    }

    let mut hopcroft_karp = HopcroftKarp::with_capacity(n);
    if hopcroft_karp.perfect_match(&graph) {
        println!("Mark");
    } else {
        println!("Veronique");
    }
}

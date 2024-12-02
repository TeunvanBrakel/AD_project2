use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::{stdin, BufRead, BufReader};
use std::str::{FromStr, SplitWhitespace};

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
        self.buffer.to_string()
    }

    fn parse<T: FromStr>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.string().trim().parse::<T>().unwrap()
    }
}

struct Line<'a> {
    split: SplitWhitespace<'a>,
}

impl<'a> Line<'a> {
    fn next<T: FromStr>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self
            .split
            .next()
            .unwrap()
            .parse::<T>()
            .unwrap()
    }
    fn pair<T: FromStr>(&mut self) -> (T, T)
    where
        T: FromStr,
        T::Err: Debug,
    {
        (self.next(), self.next())
    }
    fn collect<T: FromStr>(self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self
            .split
            .map(|d| d.parse::<T>().unwrap())
            .collect::<Vec<T>>()
    }
}
#[derive(Clone)]
struct Actors {
    name: String,
    movies: HashSet<String>,
}

#[derive(Clone)]
struct Movies {
    title: String,
    cast: HashSet<String>,
}

#[derive(Eq, PartialEq)]
enum Player {
    Veronique,
    Mark,
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (n, m) = input.line().pair::<usize>();

    let mut actresses = (0..n)
        .map(|_| input.string())
        .collect::<Vec<String>>();

    let mut actors = (0..n)
        .map(|_| input.string())
        .collect::<Vec<String>>();

    let mut movie_casts: Vec<Movies> = Vec::with_capacity(m);

    for _ in 0..m {
        let movie_title: String = input.string();
        let cast_size: usize = input.parse::<usize>();
        let cast: HashSet<String> = (0..cast_size).map(|_| input.string()).collect::<HashSet<String>>();

        movie_casts.push(
            Movies {
                title: movie_title,
                cast,
            });
    }

    game(&movie_casts, &actresses, &actors, Player::Veronique);
}

fn game(movies: &Vec<Movies>, actresses: &Vec<String>, actors: &Vec<String>, player: Player, turn: &str) -> Option<Player> {
    if actresses.len() == 0 && player == Player::Veronique {
        Some(Player::Mark)
    } else if actors.len() == 0 && player == Player::Mark {
        Some(Player::Veronique)
    } else if player == Player::Mark {
        new_new_move(movies, actors, actresses, player, turn)
    } else {
        new_new_move(movies, actors, actresses, player, turn)
    }
}

fn new_new_move(movies: &Vec<Movies>, possible_actors: &Vec<String>, possible_actresses: &Vec<String>, player: Player, turn: &str) -> Option<Player> {
    if player == Player::Veronique {
        for y in possible_actresses {
            let c = movies
                .iter()
                .filter_map(|m| {
                    if m.cast.contains(y) && m.cast.contains(turn) {
                        let n =  possible_actresses.clone();
                        n.iter().enumerate().find(|(c, acc)| )
                        game(movies,)
                    }
                })
        }
    } else {

    }
    None
}

fn new_movie(movies: &Vec<Movies>, possible_actors: &Vec<String>, possible_actresses: &Vec<String>, turn: Player, all_movies: &Vec<Movies>) {
    let mut _index: usize = 0;
    let mut result: Vec<Movies> = vec![];
    for movie in movies {
        for actor in movie.cast {
            if turn == Player::Mark {
                for possible in possible_actors.iter() {
                    if &actor == possible {
                        possible_actors.clone().remove(_index);
                        for m in all_movies.iter() {
                            if m.cast.contains(&actor) {
                                result.push(m.clone());
                            }
                        }
                        game(result, possible_actresses.clone(), possible_actors.clone(), "Veronique", all_movies.clone());
                        break;
                    }
                    _index = _index + 1;
                }
            } else if turn == "Veronique" {
                for possible in possible_actresses.iter() {
                    if &actor == possible {
                        possible_actresses.clone().remove(_index);
                        for m in all_movies.iter() {
                            if m.cast.contains(&actor) {
                                result.push(m.clone());
                            }
                        }
                        game(result.clone(), possible_actresses.clone(), possible_actors.clone(), "Mark", all_movies.clone());
                        break;
                    }
                    _index = _index + 1;
                }
            }
        }
    }
}
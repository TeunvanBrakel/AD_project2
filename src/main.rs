use crate::Player::{Mark, Veronique};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
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
}

#[derive(Debug)]
struct Casts {
    co_stars: HashMap<String, Vec<String>>,
}

impl Display for Casts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (actor, co_stars) in &self.co_stars {
            write!(f, "Actor: {}\nCo-Stars:\n", actor)?;
            for co_star in co_stars {
                writeln!(f, " - {}", co_star)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Player {
    Mark,
    Veronique,
}

impl Player {
    fn opposite(self) -> Player {
        match self {
            Mark => Veronique,
            Veronique => Mark,
        }
    }
}

fn winning_strategy(actresses: &Vec<String>, actors: &Vec<String>, casts: &Casts) -> Player {
    for actress in actresses {
        let mut remaining_actresses = actresses.clone();
        remaining_actresses.retain(|actor| actor != actress);

        if rec_winning_strategy(&remaining_actresses, actors, casts, &actress, Mark) == Veronique {
            return Veronique
        }
    }
    Mark
}

fn rec_winning_strategy(actresses: &Vec<String>, actors: &Vec<String>, casts: &Casts, current_move: &String, current_player: Player) -> Player {
    if actors.is_empty() && current_player == Mark {
        return Veronique;
    }

    if actresses.is_empty() && current_player == Veronique {
        return Mark
    }

    if let Some(co_stars) = casts.co_stars.get(current_move) {
        let options = if current_player == Mark {
            co_stars.iter().filter(|co_star| actors.contains(co_star)).collect::<Vec<&String>>()
        } else {
            co_stars.iter().filter(|co_star| actresses.contains(co_star)).collect::<Vec<&String>>()
        };

        if options.is_empty() {
            return current_player.opposite();
        }

        for option in options {
            let mut new_actors = actors.clone();
            let mut new_actresses = actresses.clone();

            if current_player == Mark {
                new_actors.retain(|actor| actor != option);
            } else {
                new_actresses.retain(|actress| actress != option);
            }

            if rec_winning_strategy(&new_actresses, &new_actors, casts, option, current_player.opposite()) == current_player {
                return current_player
            }
        }
        return current_player.opposite();
    }

    panic!("invalid move: {}", current_move)
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (n, m) = input.line().pair::<usize>();

    let actresses = (0..n)
        .map(|_| input.string().trim().to_string())
        .collect::<Vec<String>>();

    let actors = (0..n)
        .map(|_| input.string().trim().to_string())
        .collect::<Vec<String>>();

    let mut movie_casts: Casts = Casts { co_stars: HashMap::with_capacity(n) };

    actresses
        .iter()
        .zip(actors.iter())
        .for_each(|(actor, actress)| {
            movie_casts.co_stars.insert(actor.clone(), vec![]);
            movie_casts.co_stars.insert(actress.clone(), vec![]);
        });

    for _ in 0..m {
        let _: String = input.string();
        let cast_size: usize = input.parse::<usize>();
        let cast: Vec<String> = (0..cast_size).map(|_| input.string().trim().to_string()).collect::<Vec<String>>();

        for cast_member in &cast {
            if let Some(co_stars) = movie_casts.co_stars.get_mut(cast_member) {
                co_stars.extend(cast.iter().filter(|&c| c != cast_member).cloned());
            }
        }
    }

    //println!("{:?}", movie_casts);
    println!("{:?}", winning_strategy(&actresses, &actors, &movie_casts));
}

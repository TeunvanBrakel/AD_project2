fn main() {
    let mut actor1 = ActorActress{ name: String::from("BradPitt"), movies: vec![]};
    let mut actor2 = ActorActress{ name: String::from("NormanReedus"), movies: vec![]};
    let mut actress1 = ActorActress{ name: String::from("DianaKruger"), movies: vec![]};
    let mut actress2 = ActorActress{ name: String::from("MelanieLaurent"), movies: vec![]};
    let women = vec![actress1.name.clone(), actress2.name.clone()];
    let men = vec![actor1.name.clone(), actor2.name.clone()];
    let mov1: Movies = Movies{ name: String::from("ingloriousBasterd"), actor_actress: vec![actress1.name.clone(), actress2.name.clone(), actor1.name.clone()]};
    let mov2: Movies = Movies{ name: String::from("Sky"), actor_actress: vec![actress1.name.clone(), actor2.name.clone()]};
    actor1.movies.push(mov1.name.clone());
    actor2.movies.push(mov2.name.clone());
    actress1.movies.push(mov1.name.clone());
    actress1.movies.push(mov2.name.clone());
    actress2.movies.push(mov1.name.clone());
    let allmovies: Vec<Movies> = vec![mov1, mov2];

    game(allmovies.clone(), women, men,"Veronique", allmovies);
}

#[derive(Clone)]
struct ActorActress{
    name: String,
    movies: Vec<String>
}

#[derive(Clone)]
struct Movies{
    name: String,
    actor_actress: Vec<String>
}

fn game(movies: Vec<Movies>, actrices: Vec<String>, actors: Vec<String>, turn: &str, all_movies: Vec<Movies>){
    if actrices.len() == 0 && turn == "Veronique" {
        println!("{}", "Mark");
    }else if actors.len() == 0 && turn == "Mark" {
        println!("{}", "Veronique");
    }else{
        if turn == "Mark" {
            new_movie(movies, actors, actrices, turn, all_movies);
        }else{
            new_movie(movies, actors, actrices, turn, all_movies);
        }
    }
}

fn new_movie(movies: Vec<Movies>, possible_actors: Vec<String>, possible_actrices: Vec<String>, turn: &str, all_movies: Vec<Movies>){
    let mut _index: usize = 0;
    let mut result: Vec<Movies> = vec![];
    for movie in movies {
        for actor in movie.actor_actress{
            if turn == "Mark"{
                for possible in possible_actors.iter(){
                    if &actor == possible {
                        possible_actors.clone().remove(_index);
                        for m in all_movies.iter(){
                            if m.actor_actress.contains(&actor) {
                                result.push(m.clone());
                            }
                        }
                        game(result.clone(), possible_actrices.clone(), possible_actors.clone(), "Veronique", all_movies.clone());
                        break;
                    }
                    _index = _index + 1;
                }
            }
            else if turn == "Veronique"{
                for possible in possible_actrices.iter(){
                    if &actor == possible {
                        possible_actrices.clone().remove(_index);
                        for m in all_movies.iter(){
                            if m.actor_actress.contains(&actor) {
                                result.push(m.clone());
                            }
                        }
                        game(result.clone(), possible_actrices.clone(), possible_actors.clone(), "Mark", all_movies.clone());
                        break;
                    }
                    _index = _index + 1;
                }
            }
        }
    }
}
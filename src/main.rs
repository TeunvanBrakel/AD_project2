fn main() {
    let women = vec!["tes".to_string(), "test".to_string()];
    let men = vec!["asdfdasf".to_string(), "dasf".to_string(), "fdas".to_string()];
    let mut result = "";
    game(women, men,"Veronique");
}

fn game(mut x: Vec<String>, mut y: Vec<String>, turn: &str){
    if(x.len() == 0 && turn == "Veronique"){
        println!("{}", "Mark");
    }else if(y.len() == 0 && turn == "Mark"){
        println!("{}", "Veronique");
    }else{
        if(turn == "Mark"){
            y.remove(0);
            game(x, y, "Veronique");
        }else{
            x.remove(0);
            game(x, y, "Mark");
        }
    }
}
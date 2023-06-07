use std::collections::HashMap;

pub fn tournament_winner(matches: Vec<Vec<&str>>, records: Vec<i8>) -> String {
    let mut winners = HashMap::new();
    let mut iter = matches.iter()
    while let Some(value) = iter.next() {
        println!("{}", matches[value])
    }
    return "thing".to_string()
}

fn main() {
    println!("{}", tournament_winner(vec![vec!["HTML", "C"], vec!["C", "Python"], vec!["Python", "HTML"]], vec![0,1,2]));
}
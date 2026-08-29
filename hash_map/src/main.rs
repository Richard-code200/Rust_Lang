use std::{collections::HashMap, vec};
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("The score for {team_name} is {score:?}");

    for (key, value) in &scores {
        println!("{key} : {value}")
    }

    scores.insert(String::from("Blue"), 25);
    let score = scores.get(&team_name);
    println!("The score for {team_name} is {score:?}");
    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world mandom world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite character");
    let field_value = String::from("kayoko");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("Hello, world!");
}

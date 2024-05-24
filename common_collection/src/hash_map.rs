use std::collections::HashMap;

pub fn hash_map () {
    let team1 : String = String::from("Futbal Club Barcelona");
    let team2 : String = String::from("Real Madrid");

    let mut score = HashMap::new();
    //ownership of team1 and team2 is sent to the hashmap
    score.insert(team1, 2);
    score.insert(team2, 0);
    println!("{:?}", score);

    //how to access the hashmap
    //redefining because team1's ownership is with the hash map
    let team1 = String::from("Futbal Club Barcelona");
    let score1 = score.get(&team1);
    match score1 {
        Some(value) => println!("the score is: {:?}", value),
        None => println!("it has no value")
    }

    //iterating
    for (key, value) in &score{
        println!("key: {} value: {}", key, value);
    }

    //overwrite
    score.insert(String::from("Sevilla"), 2);
    score.insert(String::from("Sevilla"), 4);

    //for no overwrite
    score.entry(String::from("Athletico Madrid")).or_insert(3); //if there is no entry of the variable we execute this
    score.entry(String::from("Athletico Madrid")).or_insert(4); //here already there is entry of Athletico Madrid so we dont execute this
    
    for (key, value) in &score {
        println!("keys: {} value: {}", key, value);
    }

    //word count in a string
    let test_string : String = String::from("Hello Pranil Hello mate Hello");
    let array = test_string.split_whitespace();

    let mut map = HashMap::new();
    for word in array {
        //already present key, only increases the count 
        let count = map.entry(word).or_insert(0);
        *count = *count + 1;
    }
    println!("{:?}", map);
}
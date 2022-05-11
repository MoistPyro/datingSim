use rand::Rng;
use std::io;

pub struct NPC {
    pub ego: i8,
    pub muscle: i8,
    pub smarts: i8,
    pub name: String,
    pub relationship: i8,
}

impl NPC {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            ego: rng.gen_range(1..11),
            muscle: rng.gen_range(1..11),
            smarts: rng.gen_range(1..11),
            name: name.to_string(),
            relationship: 0,
        }
    }

    pub fn react_to_score(&mut self, score: i8) -> String {
        self.relationship += score - 6;
        
        if score == 15 {
            "You know me perfectly.".to_string()
        } else if score >= 9 {
            "I like that".to_string()
        } else if score >= 3 {
            "You tried..".to_string()
        } else {
            "You have to try harder than that.".to_string()
        }
    }
}

pub struct Player {
    pub ego: i8,
    pub muscle: i8,
    pub smarts: i8,
    pub name: String,
}

impl Player {
    pub fn new() -> Self {
        println!("how big is your ego?");
        let ego:i8 = input_to_i8();

        println!("how strong are you?");
        let muscle: i8 = input_to_i8();
        
        println!("how smart are you?");
        let smarts: i8 = input_to_i8();
        
        println!("What is your name?");
        let name: String = input_to_string();

        Self {
            ego: ego,
            muscle: muscle,
            smarts: smarts,
            name: name.to_string()
        }
    }
}

fn input_to_i8() -> i8 {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().parse::<i8>().unwrap()
}

fn input_to_string() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().to_string()
}

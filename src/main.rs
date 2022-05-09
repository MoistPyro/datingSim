use rand::{self, Rng};
use std::io;

struct NPC {
    ego: i8,
    muscle: i8,
    smarts: i8,
    name: String,
    relationship: i8,
}

impl NPC {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            ego: rng.gen_range(0..10),
            muscle: rng.gen_range(0..10),
            smarts: rng.gen_range(0..10),
            name: name.to_string(),
            relationship: 0,
        }
    }

    fn answer(&mut self, score: i8) -> String {
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

struct Player {
    ego: i8,
    muscle: i8,
    smarts: i8,
    name: String,
}

impl Player {
    pub fn new() -> Self {
        let ego:i8 = input_to_i8("how big is your ego?");
        let muscle: i8 = input_to_i8("how strong are you?");
        let smarts: i8 = input_to_i8("how smart are you?");
        let name: String = input_to_string("What is your name?");

        Self {
            ego: ego,
            muscle: muscle,
            smarts: smarts,
            name: name.to_string()
        }
    }
}

struct Compliment {
    ego: i8,
    muscle: i8,
    smarts: i8,
    text: String,
}

impl Compliment {
    pub fn give(&mut self, giver: &Player, target: &NPC) -> i8 {
        println!("You tell {}: {}", target.name, self.text);

        let ego = giver.ego + self.ego - target.ego;
        let muscle = giver.muscle + self.muscle - target.muscle;
        let smarts = giver.smarts + self.smarts - target.smarts;

        15 - ego.abs() - muscle.abs() - smarts.abs()
    }
}

fn main() {
    let mut marianne = NPC::new("Marianne");

    println!("name: {}, ego: {}, muscle: {}, smarts: {}", marianne.name, marianne.ego, marianne.muscle, marianne.smarts);

    let player = Player::new();

    let mut compliment1 = Compliment {
        ego: 5,
        muscle: 0,
        smarts: 0,
        text: "I'm better than you".to_string()
    };

    let mut compliment2 = Compliment {
        ego: 0,
        muscle: 2,
        smarts: 0,
        text: "Let's fight.".to_string()
    };

    let answer1: String = marianne.answer(compliment1.give(&player, &marianne));
    println!("{answer1}");
    println!("current relation: {}", marianne.relationship);

    let answer2: String = marianne.answer(compliment2.give(&player, &marianne));
    println!("{answer2}");
    println!("current relation: {}", marianne.relationship);
}

fn input_to_i8(question: &str) -> i8 {
    println!("{question}");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().parse::<i8>().unwrap()
}

fn input_to_string(question: &str) -> String {
    println!("{question}");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().to_string()
}
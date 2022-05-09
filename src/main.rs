mod characters;
use crate::characters::characters::{Player, NPC};

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

    let answer2: String = marianne.answer(compliment2.give(&player, &marianne));
    println!("{answer2}");
}
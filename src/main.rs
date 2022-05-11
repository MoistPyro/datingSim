mod characters;
use crate::characters::{Player, NPC};
mod interactions;
use interactions::*;

fn main() {
    let mut marianne = NPC::new("Marianne");

    println!("name: {}, ego: {}, muscle: {}, smarts: {}", marianne.name, marianne.ego, marianne.muscle, marianne.smarts);

    let player = Player::new();

    let mut interaction1 = Impress {
        ego_boost: 1,
        flex: 0,
        smarts_multiplier: 2,
        text: "I'm better than you".to_string()
    };

    let mut interaction2 = Impress {
        ego_boost: 0,
        flex: 2,
        smarts_multiplier: 1,
        text: "Let's fight.".to_string()
    };

    let answer1: String = marianne.react_to_score(interaction1.give(&player, &marianne));
    println!("{answer1}");

    let answer2: String = marianne.react_to_score(interaction2.give(&player, &marianne));
    println!("{answer2}");

    let mut interaction3 = Request {
        relation_threshold: 10,
        text: "Kiss me tenderly".to_string()
    };

    let answer3: String = marianne.react_to_score(interaction3.give(&player, &marianne));
    println!("{answer3}");
}
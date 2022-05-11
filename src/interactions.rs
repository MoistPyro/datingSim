use crate::characters::{Player, NPC};

pub trait Interact {
    fn give(&mut self, giver: &Player, target: &NPC) -> i8;
}

pub struct Impress {
    pub ego_boost: i8,
    pub flex: i8,
    pub smarts_multiplier: i8,
    pub text: String,
}

impl Interact for Impress {
    fn give(&mut self, giver: &Player, target: &NPC) -> i8 {
        println!("You tell {}: {}", target.name, self.text);

        let ego = giver.ego + self.ego_boost - target.ego;
        let muscle = giver.muscle + self.flex - target.muscle;
        let smarts = (giver.smarts - target.smarts) * self.smarts_multiplier;

        15 - ego.abs() + muscle - smarts.abs()
    }
}

pub struct Request {
    pub relation_threshold: i8,
    pub text: String,
}

impl Interact for Request {
    fn give(&mut self, giver: &Player, target: &NPC) -> i8 {
        println!("You make a simple request: {}", self.text);
        if target.relationship < self.relation_threshold {0} else {9}
    }
}
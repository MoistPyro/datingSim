pub mod characters {
    use rand::Rng;
    use std::io;
    pub struct NPC {
        pub ego: i8,
        pub muscle: i8,
        pub smarts: i8,
        pub name: String,
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

        pub fn answer(&mut self, score: i8) -> String {
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
}
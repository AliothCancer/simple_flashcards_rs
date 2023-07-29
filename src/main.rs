use std::io::{self, Write};

const MENU_STRING: &str = "
    Main Menu

    0. Exit
    1. New subject
"; 



fn main() {
    let card1 = Card{
        keyword: "Potenziale d'azione".to_string(),
        description: "Il potenziale blablabla...".to_string(),
        priority_index: 0,
        side: CardSide::Front
    };

    let arg1 = Argument{
        name: "Sist. nervoso".to_string(),
        flashcards: vec![card1]
    };

    let sub1 = Subject{
        name: "Fisiologia".to_string(),
        arguments: vec![arg1]
    };

    let mut cmd = String::new();
    loop {
        
        println!("{}", MENU_STRING);
        
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cmd).expect("Errore leggendo");
        


        match cmd.trim() {
            "0" => {
                println!("Exit");
                break
            },
            _ => {
                println!("Other case is called: {}", cmd);
                cmd.clear()
            }
        }
    }
}

struct Subject{
    name: String,
    arguments: Vec<Argument>
}

struct Argument {
    name: String,
    flashcards: Vec<Card>
}

struct Card {
    keyword: String,
    description: String,
    priority_index: i32,
    side: CardSide
}

enum CardSide{
    Front,
    Back
}

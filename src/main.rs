
use std::{io::{self, Write}, collections::HashMap};

const MENU_STRING: &str = "
    Main Menu

    0. Exit
    1. New subject
"; 


struct MenuOptions{
    menu_name: String,
    option_list: Vec<String>
}

impl MenuOptions {
    fn new<T: Into<String>>(menu_name: &str, option_list: Vec<T>) -> Self {
        let converted_option_list: Vec<String> = option_list.into_iter().map(|item| item.into()).collect();
        MenuOptions {
            menu_name: menu_name.to_string(),
            option_list: converted_option_list,
        }
    }
    fn option_call(menu: MenuOptions){
        menu.run()
    }
    fn run(self) {

        let mut cmd = String::new();
        loop {

            self.print_title();
            self.print_options();

            print!("\n>>> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).expect("Errore leggendo");

            // Parse the input command to get the selected index
            match cmd.trim().parse::<usize>() {
                Ok(option_index) => {
                    if option_index < self.option_list.len() {
                        println!("Selected: {}\n\n", self.option_list[option_index]);
                        //call menu.option_call()
                        if option_index == 0{
                            break
                        }
                    } else {
                        println!("Invalid option index: {}\n\n", option_index);
                    }
                }
                Err(_) => {
                    println!("Invalid input: {}", cmd);
                }
            }
            cmd.clear();
        }
    }

    fn print_title(&self){
        println!("\n\n\t  {}\n", self.menu_name)
    }

    fn print_options(&self) {
        for (index, choice) in self.option_list.iter().enumerate() {
            println!("\t{}. {}", index, choice)
        }
    }
}

fn main() {

    let mut hash_menu: HashMap<&str, MenuOptions> = HashMap::new();

    let main_menu = MenuOptions::new(
        "Main",
        vec![
            "Exit",
            "New subject",
            "Another option",
        ]);

    let new_subj_menu = MenuOptions::new(
            "New Subject menu",
            vec![
                "Opt1",
                "Opt2",
                "Opt3",
                ]);
    
    hash_menu.insert("Main menu", main_menu);
    hash_menu.insert("Subject menu", new_subj_menu);

    //let sub = Subject::default();

    // I'd want to call the run function of new_subj_menu from
    // inside of the run function of main_menu

    
}

struct Subject{
    name: String,
    arguments: Vec<Argument>
}


impl Default for Subject{
    fn default() -> Self { // only for example purpose
        let card = Card{
            keyword: "Potenziale d'azione".to_string(),
            description: "Il potenziale blablabla...".to_string(),
            priority_index: 0,
            side: CardSide::Front
        };
    
        let arg = Argument{
            name: "Sist. nervoso".to_string(),
            flashcards: vec![card]
        };
    
        Subject{
            name: "Fisiologia".to_string(),
            arguments: vec![arg]
        }
    }
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


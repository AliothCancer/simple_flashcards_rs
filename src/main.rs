mod deck;

use deck::*;

enum RepetitionMode{
    Linear,
    NonLinear,
    Other
}

struct Quiz{
    subject: Subject,
    mode: RepetitionMode,
    arguments: Argument
}

fn main() {
    let mut quiz = Quiz::new();
    loop {
        quiz.select_subject();
        quiz.select_mode();
        quiz.select_arguments();

        quiz.start_quiz();
    }
}

impl Quiz{
    fn new() -> Self {
        todo!()
    }
    fn select_subject(&mut self){
        todo!()
    }
    
    fn select_mode(&mut self){
        todo!()
    }
    
    fn select_arguments(&mut self){
        todo!()
    }
        
    fn start_quiz(&mut self){
        todo!()
    }
}
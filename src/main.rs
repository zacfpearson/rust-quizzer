use std::io::{self};

struct QuizCard {
    question: String,
    answer: String,
    times_quized: u32,
    times_correct: u32
}

fn add_question( ) -> Result<QuizCard, std::io::Error> {
    let mut question = String::new();
    let mut answer = String::new();

    println!("question: ");
    io::stdin().read_line(&mut question)?;
    println!("answer: ");
    io::stdin().read_line(&mut answer)?;
    let card: QuizCard = QuizCard{ 
        question: question,
        answer: answer, 
        times_quized: 0, 
        times_correct: 0 };
    println!("question: {}", card.question );
    println!("answer: {}", card.answer );
    Ok(card)
}

fn ask_question( ) {
    println!("not implemented yet.");
}

fn main() {

    let mut mode = String::from("");

    let mut quiz_questions: Vec<QuizCard> = Vec::new();

    loop {
        println!("(q)uit | (a)dd question | (s)tart quiz");
        mode = String::from("");

        match io::stdin().read_line(&mut mode) {
            Ok(n) => {
                match mode.trim() {
                    "q" => {
                        println!("Good Bye!");
                        break;
                    },
                    "a" => {
                        let card = match add_question() {
                            Ok(card) => card,
                            Err(_) => {
                                println!("Couldn't create Quiz Card.");
                                continue;
                            }
                        };
                        quiz_questions.push(card);
                    },
                    "s" => ask_question(),
                    _ => println!("Enter A Valid Field"),
                }
                
            }
            Err(error) => println!("error: {}", error),
        }
    }

    
}

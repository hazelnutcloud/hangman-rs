use rand::Rng;
use std::{error::Error, io, process};

mod words {
    pub fn list() -> String {
        String::from(
            ":animals
        horse
        rabbit
        elephant
        giraffe
        hippopotamus
    :sports
        badminton
        football
        rugby
        archery
        pingpong
    :food
        pizza
        hamburger
        banana
        rice
        hotdog
    :countries
        japan
        lithuania
        russia
        malaysia
        mexico
        peru
        indonesia
    :vehicles
        airplane
        car
        van
        bicycle
        motorcycle
        boat",
        )
    }
}

struct Config {
    category: String,
    secret_word: String,
}

struct Score {
    points: i32,
    tries: usize,
}

impl Score {
    fn add_twenty(&mut self) {
        self.points += 20;
    }

    fn subtract_twenty(&mut self) {
        self.points -= 20;
    }

    fn add_try(&mut self) {
        self.tries += 1;
    }
}

fn display_welcome_message() {
    const HANGMAN: &str = "\
    \n\n
██╗░░██╗░█████╗░███╗░░██╗░██████╗░███╗░░░███╗░█████╗░███╗░░██╗
██║░░██║██╔══██╗████╗░██║██╔════╝░████╗░████║██╔══██╗████╗░██║
███████║███████║██╔██╗██║██║░░██╗░██╔████╔██║███████║██╔██╗██║
██╔══██║██╔══██║██║╚████║██║░░╚██╗██║╚██╔╝██║██╔══██║██║╚████║
██║░░██║██║░░██║██║░╚███║╚██████╔╝██║░╚═╝░██║██║░░██║██║░╚███║
╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝░╚═════╝░╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝

                    Made with <3 by @hsnmkls

--------------------------------------------------------------

                 press ENTER to start the game!

--------------------------------------------------------------";

    println!("{}", HANGMAN);
}

fn get_and_display_categories_list() -> Result<Vec<String>, Box<dyn Error>> {
    let categories = words::list();

    println!("Choose a category:\n");

    let mut categories_list = vec![];
    for category in categories.as_str().split(':') {
        if let Some(name) = category.lines().next() {
            println!("-- {}", name);
            categories_list.push(name.trim().to_string());
        }
    }

    Ok(categories_list)
}

fn parse_config(categories_list: Vec<String>) -> Result<Config, Box<dyn Error>> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if !categories_list.iter().any(|e| e == input) {
            println!("invalid category. try again")
        } else {
            let secret_word = secret_word(input);
            return Ok(Config {
                category: input.to_string(),
                secret_word,
            });
        }
    }
}

fn secret_word(category_chosen: &str) -> String {
    let categories = words::list();
    let mut result = String::new();

    for category_and_words in categories.as_str().split(':') {
        let category = match category_and_words.lines().next() {
            Some(name) => name.trim(),
            _ => "",
        };
        if category == category_chosen {
            let words: Vec<&str> = category_and_words.lines().collect();
            let secret_index = rand::thread_rng().gen_range(1..words.len());
            result = words[secret_index].trim().to_string();
        }
    }

    result
}

fn display_hangman(tries: usize) {
    let hangman = [
        "\
\n
\n       
\n     
\n           
_____",
        "\
  _
 |      
 |      
 |      
 |       
 |      
 |
_|___",
        "\
 ________
 |/      
 |      
 |      
 |       
 |      
 |
_|___",
        "\
 ________
 |/     |
 |      
 |      
 |       
 |      
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |      
 |       
 |      
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |      |
 |      |
 |      
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |     \\|
 |      |
 |      
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |     \\|/
 |      |
 |      
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |     \\|/
 |      |
 |     / 
 |
_|___",
        "\
 ________
 |/     |
 |     (_)
 |     \\|/
 |      |
 |     / \\
 |
_|___",
    ];

    println!("{}\n", hangman[tries]);
}

enum Guess {
    Correct {
        letter: String,
        correct_indices: Vec<usize>,
    },
    Incorrect,
    Pass,
}

fn parse_guess(secret_word: &str, letters_guessed: &mut Vec<String>) -> Guess {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("invalid guess");
    let guess = guess.trim();
    if guess.len() != 1 {
        return Guess::Pass;
    };
    if letters_guessed.contains(&guess.to_string()) {
        return Guess::Pass;
    } else {
        letters_guessed.push(guess.to_string());
    }

    let mut correct_guess = false;
    let mut correct_index = vec![];
    for (index, char) in secret_word.chars().enumerate() {
        if let Some(guess_char) = guess.chars().next() {
            if char == guess_char {
                correct_index.push(index);
                correct_guess = true;
            }
        }
    }
    if correct_guess {
        Guess::Correct {
            letter: guess.to_string(),
            correct_indices: correct_index,
        }
    } else {
        Guess::Incorrect
    }
}

fn loop_game(config: Config) {
    let mut score = Score {
        points: 0,
        tries: 0,
    };

    let mut display_letters = vec!["_".to_string(); config.secret_word.len()];
    let mut letters_guessed: Vec<String> = vec![];

    loop {
        print!("\x1B[2J\x1B[1;1H");
        display_hangman(score.tries);
        println!("{}\n", display_letters.join(" "));
        println!("points: {}", score.points);
        println!("tries remaining: {}", 9 - score.tries);
        println!("letters guessed: {}\n", letters_guessed.join(" "));

        if score.tries == 9 {
            println!("GAME OVER!");
            println!("the correct answer was {}!", config.secret_word);
            process::exit(0);
        }
        if display_letters.join("") == config.secret_word {
            println!("YOU WON!");
            process::exit(0);
        }

        println!("enter your guess: ");

        match parse_guess(&config.secret_word, &mut letters_guessed) {
            Guess::Correct {
                letter,
                correct_indices,
            } => {
                for index in correct_indices {
                    display_letters[index] = letter.to_string();
                }
                score.add_twenty();
            }
            Guess::Incorrect => {
                score.subtract_twenty();
                score.add_try();
            }
            Guess::Pass => (),
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");
    display_welcome_message();

    let mut skip = String::new();
    io::stdin().read_line(&mut skip).unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let categories_list = get_and_display_categories_list()?;
    print!("\x1B[2J\x1B[1;1H");

    let config = parse_config(categories_list)?;

    println!(
        "\nyou have chosen category {}. good luck!\n",
        config.category
    );

    loop_game(config);
    Ok(())
}

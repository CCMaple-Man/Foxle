use rand::Rng;
use colored::Colorize;
#[derive(PartialEq)]
#[derive(Debug)]
enum LetterStatus {
    None,
    Wrong,
    Right,
    Correct
}

#[derive(Debug)]
struct GuessWord {
    guess: String,
    guess_length: u8,
}

impl GuessWord{

    fn check_guess_length(&self, guess: &GuessWord) -> bool {
        self.guess_length == guess.guess_length
    }
}

#[derive(Debug)]
struct GuessResult {
    guess_letters: Vec<Letter>,
}

impl GuessResult {
   fn print_guess_result(&self) {
    for letter in &self.guess_letters {
        match letter.letter_status {
            LetterStatus::None => print!("{}", "_".blue()),
            LetterStatus::Wrong => print!("{}", letter.letter_character.to_string().red()),
            LetterStatus::Right => print!("{}", letter.letter_character.to_string().yellow()),
            LetterStatus::Correct => print!("{}", letter.letter_character.to_string().green()),
            }
        }
    }

    fn check_guess_result_with_game_word(&mut self, game_word: &GuessWord) {
        for letter in &mut self.guess_letters {

            if game_word.guess.to_lowercase().chars().nth(letter.letter_position) == Some(letter.letter_character) {
                letter.letter_status = LetterStatus::Correct;
            }

           else if game_word.guess.to_lowercase().contains(letter.letter_character) {
               letter.letter_status = LetterStatus::Right;
           }  
           else {
               letter.letter_status = LetterStatus::Wrong;
           }
        }
    }

    fn check_if_all_characters_are_correct(&self) -> bool {
        for letter in &self.guess_letters {
            if letter.letter_status != LetterStatus::Correct {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Letter {
    letter_character: char,
    letter_position: usize,
    letter_status: LetterStatus,
}

#[derive(Debug)]
struct Game {
    word: GuessWord,
    guess: GuessWord,
    guess_result: GuessResult,
    num_of_guesses: u8,
    max_guesses: u8,
}

impl Game{
    fn is_game_over(&self) -> bool {
        self.num_of_guesses == self.max_guesses
    }

    fn is_game_won(&self) -> bool {
        self.guess_result.check_if_all_characters_are_correct()
    }

    fn is_guess_right_length(&self) -> bool {
        self.word.check_guess_length(&self.guess)
    }
    
    fn set_game_guess_word(&mut self, game_word: &str) {
       self.word = GuessWord {
            guess: String::from(game_word),
            guess_length: game_word.len() as u8,
        }
    }
    fn set_game_guess(&mut self, game_guess: &str) {
        self.guess = GuessWord {
            guess: String::from(game_guess),
            guess_length: game_guess.len() as u8,
        }
    }

    fn set_guess_from_user_input(&mut self) {
        self.set_game_guess(&get_user_input());
    }

    fn set_game_guess_results(&mut self) {
        self.guess_result = GuessResult { guess_letters: get_letters(&self.guess.guess)};

        self.guess_result.check_guess_result_with_game_word(&self.word);
    }
 
    fn print_guess_result(&self) {
        self.guess_result.print_guess_result();
        println!("");
    }

    fn play_game(&mut self)
    {
        println!("Welcome to Foxle, your word is {} letters long", self.word.guess_length);
        
        while self.is_game_over() == false
        {
            println!("Enter your guess: ");
            self.set_guess_from_user_input();
            
            while self.is_guess_right_length() == false {
                println!("Your guess is not the right length, try again:");
                self.set_guess_from_user_input();
            }

            self.set_game_guess_results();
            self.print_guess_result();

            if self.is_game_won() {
                println!("You won the game!");
                break;
            }
            self.num_of_guesses += 1;
        }
    }
}

fn get_letters(word: &str) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();
    let mut position: usize = 0;
    for letter in word.chars() {
        letters.push(Letter {
            letter_character: letter,
            letter_position: position,
            letter_status: LetterStatus::None,
        });
        position += 1;
    }
    letters
}

fn load_words_from_file() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let file = std::fs::read_to_string("src/files/words.txt").expect("Unable to read file");
    for line in file.lines() {
        words.push(String::from(line));
    }
    words
}

fn get_random_word(words: &Vec<String>) -> String {
    let random_number = rand::thread_rng().gen_range(0..= words.len());
    words[random_number].clone()
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input_trimmed = user_input.trim();
    user_input_trimmed.to_string()
}

fn main() {
  
    let mut the_game = Game {

        guess: GuessWord {
            guess: String::from(""),
            guess_length: 0,
        },

        word: GuessWord {
            guess: String::from(""),
            guess_length: 0,
        },

        guess_result: GuessResult {
            guess_letters: Vec::new(),
        },
        num_of_guesses: 0,
        max_guesses: 5,
    };
    
    the_game.set_game_guess_word(&get_random_word(&load_words_from_file()));

    the_game.play_game();
}
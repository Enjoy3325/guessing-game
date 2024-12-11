use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() {
    println!("Guess the number!");

    let mut player1_name = String::new();
    let mut player2_name = String::new();

    println!("Enter the name of Player 1:");
    io::stdin()
        .read_line(&mut player1_name)
        .expect("Failed to read line");
    player1_name = player1_name.trim().to_string();

    println!("Enter the name of Player 2:");
    io::stdin()
        .read_line(&mut player2_name)
        .expect("Failed to read line");
    player2_name = player2_name.trim().to_string();

    // Initializing player accounts
    let mut player1_wins = 0;
    let mut player2_wins = 0;

    // Player results table
    let mut results = String::new();

    loop {
        // Random number generation is different for each player
        let secret_number_player1 = rand::thread_rng().gen_range(1..=100);
        let secret_number_player2 = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is hidden for both players.");
        // Player 1 starts the game
        println!("\n{}'s turn!", player1_name);
        // println!("{secret_number_player1}");
        if play_game(secret_number_player1) {
            println!("{} wins this round!", player1_name);
            player1_wins += 1;
        } else {
            println!("{} did not guess the number.", player1_name);
        }

        // Player 2 starts the game
        println!("\n{}'s turn!", player2_name);
        // println!("{secret_number_player2}");
        if play_game(secret_number_player2) {
            println!("{} wins this round!", player2_name);
            player2_wins += 1;
        } else {
            println!("{} did not guess the number.", player2_name);
        }

        // Displaying current results
        println!("\nCurrent Score:");
        println!("{}: {}", player1_name, player1_wins);
        println!("{}: {}", player2_name, player2_wins);

        // Save the results to a file
        results.push_str(&format!(
            "{}: {}, {}: {}\n",
            player1_name, player1_wins, player2_name, player2_wins
        ));
        save_results(&results);

        // We ask if they want to continue the game
        println!("\nDo you want to play again? (y/n)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Game Over! Final Scores:");
    println!("{}: {}", player1_name, player1_wins);
    println!("{}: {}", player2_name, player2_wins);
}

// A game feature where the player tries to guess a number
fn play_game(secret_number: u8) -> bool {
        let victory_art = r#"
                    ***********************
               *********************************
           *******   *     *       *    *    ******
        *******   ***      **     **     ***   ******
      ******   *****       *********      *****    *****
    ******  ********       *********       ******    *****
   ****   **********       *********       *********   *****
  ****  **************    ***********     ************   ****
 ****  *************************************************  ****
****  ***************************************************  ****
****  ****************************************************  ****
****  ****************************************************  ****
 ****  ***************************************************  ****
  ****  *******     ****  ***********  ****     *********  ****
   ****   *****      *      *******      *      ********  ****
    *****   ****             *****             ******   *****
      *****   **              ***              **    ******
       ******   *              *              *   *******
         *******                                *******
            ********                         *******
               *********************************
                    ***********************
    "#;
    let mut attempts = 0;
    let max_attempts = 5;

    loop {
        attempts += 1;
        println!("Attempt {}/{}: Please input your guess.", attempts, max_attempts);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

    // Options for comparing the entered number with a secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉");
                println!("{}", victory_art);
                return true;
            }
        }

        if attempts >= max_attempts {
            println!("You've used all your attempts. Better luck next time!😞\nThe secret number was: {}", secret_number);
            break;
        }
    }

    false
}

// Function for saving results to file
fn save_results(results: &str) {
    let file_path = "game_results.txt";

    // Opening a file for appending (if the file does not exist, it will be created)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    // Writing results to a file
    if let Err(e) = writeln!(file, "{}", results) {
        eprintln!("Failed to write to file: {}", e);
    }
}

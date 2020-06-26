extern crate rand;

mod game;

use game::Game;
use std::io;

fn main() {
    println!("Welcome to Tic-Tac-Toe!");

    let mut exit = false;

    while !exit {
        let mut game = Game::new();

        game.play_game();

        exit = player_is_finished();
    }
}

fn player_is_finished() -> bool {
    let mut player_input = String::new();

    println!("Are you finished playing (y/n)?:");

    match io::stdin().read_line(&mut player_input) {
        Ok(_) => {
            let temp = player_input.to_lowercase();

            temp.trim() == "y" || temp.trim() == "yes"
        }
        Err(_) => false,
    }
}

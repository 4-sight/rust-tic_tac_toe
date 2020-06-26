use rand;
use std::io;

type Board = Vec<Vec<String>>;

enum Turn {
    Player,
    Bot,
}

enum Token {
    X,
    O,
}

pub struct Game {
    board: Board,
    current_turn: Turn,
    player_token: Token,
    bot_token: Token,
}

impl Game {
    pub fn new() -> Game {
        let first_row = vec![String::from("1"), String::from("2"), String::from("3")];

        let second_row = vec![String::from("4"), String::from("5"), String::from("6")];

        let third_row = vec![String::from("7"), String::from("8"), String::from("9")];

        Game {
            board: vec![first_row, second_row, third_row],
            current_turn: Turn::Player,
            player_token: Token::X,
            bot_token: Token::X,
        }
    }

    pub fn play_game(&mut self) {
        let mut finished = false;
        self.assign_tokens();

        while !finished {
            self.play_turn();

            if self.game_is_won() {
                self.print_board();

                match self.current_turn {
                    Turn::Player => println!("You won!"),
                    Turn::Bot => println!("You lost!"),
                };

                finished = true;
            }

            self.current_turn = self.get_next_turn();
        }
    }

    fn assign_tokens(&mut self) {
        let mut player_input = String::new();
        println!("Please choose your token (x/o):");
        match io::stdin().read_line(&mut player_input) {
            Err(_) => println!("Error reading input, try again!"),
            Ok(_) => match player_input.trim() {
                "x" => {
                    self.player_token = Token::X;
                    self.bot_token = Token::O;
                }
                "X" => {
                    self.player_token = Token::X;
                    self.bot_token = Token::O;
                }
                "o" => {
                    self.player_token = Token::O;
                    self.bot_token = Token::X;
                }
                "O" => {
                    self.player_token = Token::O;
                    self.bot_token = Token::X;
                }
                &_ => {
                    println!("\nSorry I didn't understand that input\n");
                    self.assign_tokens();
                }
            },
        };
    }

    fn get_token(&self, turn: Turn) -> String {
        match turn {
            Turn::Player => return Self::token_to_string(&self.player_token),
            Turn::Bot => return Self::token_to_string(&self.bot_token),
        }
    }

    fn token_to_string(token: &Token) -> String {
        match token {
            Token::X => return String::from("X"),
            Token::O => return String::from("O"),
        }
    }

    fn play_turn(&mut self) {
        self.print_board();
        let (token, valid_move) = match self.current_turn {
            Turn::Player => (self.get_token(Turn::Player), self.get_player_move()),
            Turn::Bot => (self.get_token(Turn::Bot), self.get_bot_move()),
        };

        let (row, col) = Self::to_board_location(valid_move);

        self.board[row][col] = token;
    }

    fn print_board(&self) {
        let separator = "+---+---+---+";

        println!("\n{}", separator);

        for row in &self.board {
            println!("| {} |\n{}", row.join(" | "), separator);
        }

        print!("\n")
    }

    fn get_player_move(&self) -> u32 {
        loop {
            let mut player_input = String::new();

            println!("\nPlease enter your move (an integer between 1 and 9): ");

            match io::stdin().read_line(&mut player_input) {
                Err(_) => println!("Error reading input, try again!"),
                Ok(_) => match self.validate(&player_input) {
                    Err(err) => println!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    fn validate(&self, input: &str) -> Result<u32, String> {
        match input.trim().parse::<u32>() {
            Err(_) => Err(String::from("Please input a valid unsigned integer!")),
            Ok(number) => {
                if self.is_valid_move(number) {
                    Ok(number)
                } else {
                    Err(String::from(
                        "Please input a number, between 1 and 9, not already chosen!",
                    ))
                }
            }
        }
    }

    fn is_valid_move(&self, unchecked_move: u32) -> bool {
        match unchecked_move {
            1..=9 => {
                let (row, col) = Self::to_board_location(unchecked_move);

                match self.board[row][col].as_str() {
                    "X" | "O" => false,
                    _ => true,
                }
            }
            _ => false,
        }
    }

    fn to_board_location(game_move: u32) -> (usize, usize) {
        let row = (game_move - 1) / 3;
        let col = (game_move - 1) % 3;

        (row as usize, col as usize)
    }

    fn get_bot_move(&self) -> u32 {
        let mut bot_move: u32 = rand::random::<u32>() % 9 + 1;

        while !self.is_valid_move(bot_move) {
            bot_move = rand::random::<u32>() % 9 + 1;
        }

        println!("Bot played move at: {}", bot_move);

        bot_move
    }

    fn game_is_won(&self) -> bool {
        let mut all_same_row = false;
        let mut all_same_col = false;

        for index in 0..3 {
            all_same_row |= self.board[index][0] == self.board[index][1]
                && self.board[index][1] == self.board[index][2];
            all_same_col |= self.board[0][index] == self.board[1][index]
                && self.board[1][index] == self.board[2][index];
        }

        let all_same_diag_1 =
            self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2];
        let all_same_diag_2 =
            self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0];

        all_same_row || all_same_col || all_same_diag_1 || all_same_diag_2
    }

    fn get_next_turn(&self) -> Turn {
        match self.current_turn {
            Turn::Player => Turn::Bot,
            Turn::Bot => Turn::Player,
        }
    }
}

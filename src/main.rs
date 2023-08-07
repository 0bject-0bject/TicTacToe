/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

static WINNING_COMBOS: [[i32; 3]; 8] = [
    [0, 1, 2], // Top row
    [3, 4, 5], // Middle row
    [6, 7, 8], // Bottom row
    [0, 3, 6], // Left column
    [1, 4, 7], // Middle column
    [2, 5, 8], // Right column
    [0, 4, 8], // Top left to bottom right
    [2, 4, 6]  // Top right to bottom left
];

use std::{sync::mpsc::{channel, TryRecvError}, thread, time::Duration};

/// Clears the screen
fn cls() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}


// How do I get keyboard input without the user pressing the Enter key? - https://stackoverflow.com/a/73765863
extern {
    fn _getch() -> core::ffi::c_char;
}

fn getch() -> u8 {
    unsafe {
        _getch() as u8
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let speed = if args.len() > 0 {
        let entered_speed = args[0].parse::<u64>().unwrap();

        match entered_speed {
            100..=1000 => entered_speed,
            _ => {
                println!("Error: refresh speed (ms) must be between 100 and 1000");
                std::process::exit(1);
            },
        }
    } else {
        400
    };

    // Enable ansi support on windows
    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support().unwrap();

    // Clear the screen
    cls();

    // Create a channel to send keyboard input to the game thread
    let (tx_keyboard, rx_game) = channel();
    // Create a channel to send game state to the keyboard thread
    let (tx_game, rx_keyboard) = channel();
    
    // Spawn the keyboard thread
    let keyboard_thread = thread::spawn(move || {
        loop {
            // Get the utf-8 key
            let key = getch();

            // Send the utf-8 key to the game thread
            tx_keyboard.send(key).unwrap();

            // Wait for a short time to avoid hogging the CPU
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Create the game state
    let mut game_state = Gamestate::new();

    // Spawn the game thread
    let game_thread = thread::spawn(move || {
        loop {
            // Check for messages from the keyboard thread
            match rx_game.try_recv() {
                Ok(key) => {
                    // Handle the key
                    if key == 27 {
                        println!("Exiting...");
                        // Exit the program
                        std::process::exit(0);
                    } else {
                        if game_state.get_turn() == 1 {
                            game_state.set_turn(2);
                        } else {
                            game_state.set_turn(1);
                        }
                        // Update the game state based on the key
                        if let Err(e) = game_state.update(key as char) {
                            println!("{}", e);
                        }
                    }
                }
                Err(TryRecvError::Empty) => {
                    // No message received, continue with the game loop
                    if let Err(e) = game_state.update(' ') {
                        println!("{}", e);
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    println!("Error: channel disconnected");
                    std::process::exit(1);
                }
            }

            // Send the game state to the keyboard thread
            tx_game.send(game_state.clone()).unwrap();

            // Wait for a short time to avoid hogging the CPU
            thread::sleep(Duration::from_millis(speed));
        }
    });

    // Main thread
    loop {
        // Wait for a message from the keyboard thread
        match rx_keyboard.recv() {
            Ok(game_state) => {
                // Render the game state
                game_state.render();
            }
            Err(_) => {
                // Handle the error
                break;
            }
        }
    }

    // Wait for the threads to finish
    keyboard_thread.join().unwrap();
    game_thread.join().unwrap();
}


#[derive(Clone)]
struct Gamestate {
    board: [[u8; 3]; 3],
    turn: u8,
}

impl Gamestate {
    fn new() -> Gamestate {
        Gamestate {
            board: [[0; 3]; 3],
            turn: 0,
        }
    }

    fn update(&mut self, key: char) -> Result<(), String> {
        match key {
            '1' => {
                if self.board[0][0] == 0 {
                    self.board[0][0] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '2' => {
                if self.board[0][1] == 0 {
                    self.board[0][1] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '3' => {
                if self.board[0][2] == 0 {
                    self.board[0][2] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '4' => {
                if self.board[1][0] == 0 {
                    self.board[1][0] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '5' => {
                if self.board[1][1] == 0 {
                    self.board[1][1] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '6' => {
                if self.board[1][2] == 0 {
                    self.board[1][2] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '7' => {
                if self.board[2][0] == 0 {
                    self.board[2][0] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '8' => {
                if self.board[2][1] == 0 {
                    self.board[2][1] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            '9' => {
                if self.board[2][2] == 0 {
                    self.board[2][2] = self.turn;
                } else {
                    return Err(String::from("Invalid move"));
                }
            }
            _ => {
                // Do nothing
            }
        }

        self.check_win();
        self.check_tie();

        Ok(())
    }

    fn set_turn(&mut self, turn: u8) {
        self.turn = turn;
    }

    fn get_turn(&self) -> u8 {
        self.turn
    }

    fn check_win(&self) {
        for combo in WINNING_COMBOS.iter() {
            let mut player1 = 0;
            let mut player2 = 0;
            for i in combo.iter() {
                if self.board[*i as usize / 3][*i as usize % 3] == 1 {
                    player1 += 1;
                } else if self.board[*i as usize / 3][*i as usize % 3] == 2 {
                    player2 += 1;
                }
            }
            if player1 == 3 {
                println!("Player 1 (x) wins!");
                std::process::exit(0);
            } else if player2 == 3 {
                println!("Player 2 (o) wins!");
                std::process::exit(0);
            }
        }
    }

    fn check_tie(&self) {
        let mut tie = true;
        for row in self.board.iter() {
            for cell in row.iter() {
                if *cell == 0 {
                    tie = false;
                }
            }
        }
        if tie {
            println!("Tie!");
            std::process::exit(0);
        }
    }

    fn num_to_char(&self, num: u8) -> char {
        match num {
            0 => ' ',
            1 => 'X',
            2 => 'O',
            _ => ' ',
        }
    }

    fn render(&self) {
        cls();
        println!("┌───┬───┬───┐");
        println!(
            "│ {} │ {} │ {} │",
            self.num_to_char(self.board[0][0]),
            self.num_to_char(self.board[0][1]),
            self.num_to_char(self.board[0][2])
        );
        println!("├───┼───┼───┤");
        println!(
            "│ {} │ {} │ {} │",
            self.num_to_char(self.board[1][0]),
            self.num_to_char(self.board[1][1]),
            self.num_to_char(self.board[1][2])
        );
        println!("├───┼───┼───┤");
        println!(
            "│ {} │ {} │ {} │",
            self.num_to_char(self.board[2][0]),
            self.num_to_char(self.board[2][1]),
            self.num_to_char(self.board[2][2])
        );
        println!("└───┴───┴───┘");
    }
}

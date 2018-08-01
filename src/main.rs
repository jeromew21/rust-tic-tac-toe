use std::io;
use std::io::Write;

mod ttt;

use ttt::Board;
use ttt::GameState;


fn input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Input error");
    guess.pop();

    guess
}

fn main() {
    let name = "there"; //input("Type your name:");
    println!("Hello, {}", name);

    let mut board = Board::new_board();

    let mut player_turn = true;
    match input("Play as ([X]/O)? ").as_str() {
        "o" | "O" => {
            player_turn = false;
        },
        _ => {}
    }

    loop {
        board.show();

        match board.game_over() {
            GameState::X => {
                println!("X wins");
                if !player_turn {
                    println!("Human victory!");
                }
                break;
            },
            GameState::O => {
                println!("O wins");
                if player_turn {
                    println!("Defeated by the evil AI");
                }
                break;
            },
            GameState::Draw => {
                println!("Cats game");
                break;
            },
            _ => {}
        }

        if player_turn {
            let num = 
                match input("Type a move: ").parse::<usize>() {
                    Ok(i) => {
                        if i > 0 && i <= 9 {
                            if !board.make_move(i - 1, board.turn) {
                                println!("Invalid move. Make sure you choose an empty space.");
                                continue;
                            }
                        } else {
                            println!("Invalid number.");
                            continue;
                        }
                    },
                    Err(error) => {
                        println!("Please type a number (1-9).");
                        continue;
                    }
                };
            
        } else {
            println!("Making AI move");
            board.make_ai_move();
        }
        
        player_turn = !player_turn;
    }
}

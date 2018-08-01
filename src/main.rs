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

    let mut playerTurn = true;
    loop {
        board.show();

        match board.game_over() {
            GameState::X => {
                println!("X wins");
                break;
            },
            GameState::O => {
                println!("O wins");
                break;
            },
            GameState::Draw => {
                println!("Cats game");
                break;
            },
            _ => {}
        }

        if playerTurn {
            let num = 
                match input("Type a move: ").parse::<usize>() {
                    Ok(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            0
                        }
                    },
                    Err(error) => {
                        println!("Invalid input");
                        continue;
                    }
                };
            if !board.make_move(num, board.turn) {
                println!("Invalid move");
                continue;
            };
        } else {
            board.make_ai_move();
        }
        
        playerTurn = !playerTurn;
    }
}

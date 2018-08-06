use std::io;
use std::io::Write;
use std::env;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod ttt;

use ttt::ttt::Board;
use ttt::ttt::GameState;

struct Record {
    wins: i32,
    losses: i32,
    draws: i32
}

const RECORD_FILE:&str = ".tttrecord";

fn show_title(text: &str) {
    let cmd = Command::new("sh")
            .arg("-c")
            .arg(format!("figlet {}", text))
            .output();

    if let Ok(output) = cmd {
        if let Ok(s) = String::from_utf8(output.stdout) {
            print!("{}", s);
            return;
        }
    }    
}

fn input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Input error");
    guess.pop();

    guess
}

fn load_record() -> Record {
    match File::open(RECORD_FILE) {
        Ok(data) => {
            let mut reader = BufReader::new(data);
            let mut nums = Vec::new();
            for line in reader.lines() {
                let line = line.expect("Couldn't read line");
                let n:i32 = line.trim().parse::<i32>().expect("Bad file");
                nums.push(n);
            }

            return Record {
                wins: *nums.get(0).expect("Malformatted file"),
                losses: *nums.get(1).expect("Malformatted file"),
                draws: *nums.get(2).expect("Malformatted file"),
            }
        },
        _ => {
            File::create(RECORD_FILE);
        }
    }
    save_record(Record {
        wins: 0,
        losses: 0,
        draws: 0
    })
}

fn save_record(record: Record) -> Record{
    let data = format!("{}\n{}\n{}\n", 
        record.wins.to_string(),
        record.losses.to_string(),
        record.draws.to_string()
    );
    fs::write(RECORD_FILE, data);
    record
}

fn play_vs_ai() {
    show_title("Hello There");
    let mut record:Record = load_record();

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
            GameState::X | GameState::O | GameState::Draw => {
                match board.game_over() {
                    GameState::X | GameState::O => {
                        match board.game_over() {
                            GameState::X => println!("X wins"),
                            GameState::O => println!("O wins"),
                            _ => panic!("Fuck")
                        }

                        if !player_turn {
                            println!("Human victory!");
                            record.wins += 1;
                        } else {
                            println!("Defeated by the evil AI");
                            record.losses += 1;
                        }
                    },
                    GameState::Draw => {
                        println!("Cat's game");
                        record.draws += 1;
                    },
                    _ => panic!("Fuck")
                }
                
                println!("\nRecord: {} wins, {} losses, {} draws",
                    record.wins, record.losses, record.draws
                );
                save_record(record);
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

fn ai_vs_ai(show: bool) -> GameState {
    let mut board = Board::new_board();

    loop {
        if show {
            board.show();
        }

        match board.game_over() {
            GameState::X => {
                println!("X wins");
                return GameState::X;
            },
            GameState::O => {
                println!("O wins");
                return GameState::O;
            },
            GameState::Draw => {
                println!("Cats game");
                return GameState::Draw;
            },
            _ => {
                board.make_ai_move();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "ai" => {
                    loop {
                        ai_vs_ai(true);
                    }
                },
                _ => play_vs_ai()
            }
        },
        _ => play_vs_ai()
    }
    
}

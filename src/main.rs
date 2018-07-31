use std::io;

enum Space {
    X,
    O,
    Empty
}

struct Board {
    board: [Space; 9]
}

impl Board {
    fn show(&self) {
        let mut i: u32 = 0;
        for s in self.board.iter() {
            if i % 3 == 0 && i != 0 {
                println!()
            }
            match s {
                Space::X => print!("X "),
                Space::O => print!("O "),
                Space::Empty => print!("  ")
            }
            i += 1;
        }
        println!()
    }

    fn make_move(&mut self, location: usize, sp: Space) {
        self.board[location] = sp;
    }

    fn new_board() -> Board {
        Board {
            board: [
                Space::Empty, Space::Empty, Space::Empty,
                Space::Empty, Space::Empty, Space::Empty,
                Space::Empty, Space::Empty, Space::Empty,
            ]
        }
    }
}


fn input(message: &str) -> String {
    println!("{}", message);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Input error");
    guess.pop();

    guess
}

fn main() {
    let name = input("Type your name:");

    println!("Hello, {}", name);

    let board = Board::new_board();
    board.show();
}


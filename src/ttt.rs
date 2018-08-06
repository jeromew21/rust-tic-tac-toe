pub mod ttt {
    extern crate rand;
    use self::rand::Rng;

    pub enum Space {
        X,
        O,
        Empty
    }

    pub enum GameState {
        Playing,
        X,
        O,
        Draw
    }

    pub struct Board<'a> {
        pub board: [&'a Space; 9],
        pub turn: &'a Space,
        moves_made: i32,
    }

    struct BoardNode<'b> {
        board: Board<'b>,
        move_index: Option<usize>,
    }

    struct AI<'c> {
        root: BoardNode<'c>
    }

    const BOARD_WINS: [(usize, usize, usize); 8] =
        [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

    impl<'a> Board<'a> {
        pub fn show(&self) {
            let mut i: u32 = 0;
            for s in self.board.iter() {
                if i % 3 == 0 {
                    println!()
                }
                match s {
                    Space::X => print!("X "),
                    Space::O => print!("O "),
                    Space::Empty => print!(". ")
                }
                i += 1;
            }
            println!();
            println!();
        }

        pub fn switch_turn(&mut self) {
            match self.turn {
                Space::X => {
                    self.turn = &Space::O;
                },
                Space::O => {
                    self.turn = &Space::X;
                },
                _ => panic!("Board error turn wasn't X or O")
            };
        }

        pub fn make_move(&mut self, location: usize, space: &Space) -> bool {
            match self.turn {
                space => {
                    if location < 0 || location > 8 {
                        return false;
                    }
                    match self.board[location] {
                        Space::Empty => {
                            self.board[location] = space;
                            self.moves_made += 1;
                            self.switch_turn();
                            return true;
                        },
                        _ => {return false;}
                    }
                },
                _ => panic!("Wrong turn")
            }
        }

        pub fn game_over(&self) -> GameState {
            for triplet in BOARD_WINS.iter() {
                if let Space::X = self.board[triplet.0] {
                    if let Space::X = self.board[triplet.1] {
                        if let Space::X = self.board[triplet.2] {
                            return GameState::X;
                        }
                    }
                }
                if let Space::O = self.board[triplet.0] {
                    if let Space::O = self.board[triplet.1] {
                        if let Space::O = self.board[triplet.2] {
                            return GameState::O;
                        }
                    }
                }
            }
            for space in self.board.iter() {
                if let Space::Empty = space {
                    return GameState::Playing
                }
            }
            GameState::Draw
        }

        fn real_copy(&self) -> Board<'a> {
            Board {
                board: self.board.clone(),
                turn: self.turn,
                moves_made: self.moves_made
            }
        }

        pub fn make_ai_move(&mut self) {
            let ai = AI::from_board(self);
            if let Some(c) = ai.best_move(9 - self.moves_made) {
                self.make_move(c, self.turn);
            }
        }

        pub fn new_board() -> Board<'a> {
            Board {
                board: [
                    &Space::Empty, &Space::Empty, &Space::Empty,
                    &Space::Empty, &Space::Empty, &Space::Empty,
                    &Space::Empty, &Space::Empty, &Space::Empty,
                ],
                turn: &Space::X,
                moves_made: 0
            }
        }
    }

    impl<'b> BoardNode<'b> {
        fn children(&self) -> Vec<BoardNode> {
            let mut res = Vec::new();
            let side = self.board.turn;
            for i in 0..9 {
                let mut cp = self.board.real_copy();
                if cp.make_move(i, side) {
                    res.push(BoardNode {
                        board: cp,
                        move_index: Some(i)
                    })
                }
            }
            res
        }
    }

    impl<'c> AI<'c> {
        fn from_board(bd: &Board<'c>) -> AI<'c> {
            AI {
                root: BoardNode {
                    board: bd.real_copy(),
                    move_index: None
                }
            }
        }

        fn minimax(&self, node: &BoardNode, depth: i32, is_max: bool) -> i32 {
            if depth <= 0 {
                return 0
            }
            let mut lowest = 99;
            let mut highest = -99;
            let mut is_leaf = true;
            for i in node.children() {
                if let Some(c) = i.move_index {
                    is_leaf = false;
                    match (node.board.turn, i.board.game_over()) {
                        (Space::X, GameState::X) | (Space::O, GameState::O) => {
                            return if is_max { depth * depth } 
                                else { depth * -1 }
                        },
                        _ => {
                            if is_max {
                                let score = self.minimax(&i, depth - 1, false);
                                if score > highest {
                                    highest = score;
                                }
                            } else {
                                let score = self.minimax(&i, depth - 1, true);
                                if score < lowest {
                                    lowest = score;
                                }
                            }
                        }
                    }
                }
            }
            if is_leaf {return 0}; // no children
            if is_max { highest } else { lowest }
        }

        fn best_move(&self, depth: i32) -> Option<usize> {
            let mut highest = -99;
            let mut best: Vec<usize> = Vec::new();

            for i in self.root.children() {
                if let Some(c) = i.move_index {
                    //Brute heuristic: If we can win, let's win
                    //Costless in terms of big Omega
                    match (self.root.board.turn, i.board.game_over()) {
                        (Space::X, GameState::X) | (Space::O, GameState::O) => {
                            return Some(c);
                        },
                        _ => {
                            let score = self.minimax(&i, depth, false);
                            if score > highest {
                                highest = score;
                                best = Vec::new();
                                best.push(c);
                            } else if score == highest {
                                best.push(c);
                            }
                        }
                    }
                }
            }      

            let i = rand::thread_rng().gen_range(0, best.len());
            
            match best.get(i) {
                Some(c) => {
                    return Some(*c);
                }
                None => panic!("Bad random move")
            }
        }
    }
}

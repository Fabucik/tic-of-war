use ratatui::text::Line;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Player {
    Nought,
    Cross,
    None,
}

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub pos_x: usize,
    pub pos_y: usize,
    pub current_player: Player,
    pub board: Vec<Vec<Player>>,
    pub finished: bool,
    pub winner: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let board = vec![
            vec![Player::None, Player::None, Player::None],
            vec![Player::None, Player::None, Player::None],
            vec![Player::None, Player::None, Player::None],
        ];

        Self {  pos_x: 0,
                pos_y: 0,
                current_player: Player::Cross,
                board,
                finished: false,
                winner: Player::None }
    }

    pub fn reset(&mut self) {
        let board = vec![
            vec![Player::None, Player::None, Player::None],
            vec![Player::None, Player::None, Player::None],
            vec![Player::None, Player::None, Player::None],
        ];

        self.pos_x = 0;
        self.pos_y = 0;
        self.current_player = Player::Cross;
        self.board = board;
        self.finished = false;
        self.winner = Player::None;
    } 

    pub fn move_position(&mut self, mvmt: &Movement) {
        if self.finished {
            return;
        }

        match mvmt {
            Movement::Up if self.pos_y != 0 => {
                self.pos_y -= 1;
            }
            Movement::Down if self.pos_y < 2 => {
                self.pos_y += 1;
            }
            Movement::Left if self.pos_x != 0 => {
                self.pos_x -= 1;
            }
            Movement::Right if self.pos_x < 2 => {
                self.pos_x += 1;
            }

            _ => ()
        }
    }

    pub fn enter_position(&mut self) {
        if !self.finished && self.board[self.pos_x][self.pos_y] == Player::None {
            self.board[self.pos_x][self.pos_y] = self.current_player;
            self.switch_player();

            self.check_winner();
            self.check_stalemate();
        }
    }

    fn switch_player(&mut self) {
        match self.current_player {
            Player::Cross => self.current_player = Player::Nought,
            Player::Nought => self.current_player = Player::Cross,

            Player::None => ()
        }
    }

    fn check_winner(&mut self) {
        let b = &self.board;

        for i in 0..3 {
            // row win
            if b[i][0] != Player::None && b[i][0] == b[i][1] && b[i][1] == b[i][2] {
                self.winner = b[i][0];
                break;
            }

            // column win
            if b[0][i] != Player::None && b[0][i] == b[1][i] && b[1][i] == b[2][i] {
                self.winner = b[0][i];
                break;
            }
        }

        if b[0][0] != Player::None && b[0][0] == b[1][1] && b[1][1] == b[2][2] {
            self.winner = b[0][0];
        }

        if b[0][2] != Player::None && b[0][2] == b[1][1] && b[1][1] == b[2][0] {
            self.winner = b[0][2];
        }

        if self.winner != Player::None {
            self.finished = true;
            self.pos_x = 0;
            self.pos_y = 0;
        }
    }

    fn check_stalemate(&mut self) {
        for x in &self.board {
            for y in x {
                if y == &Player::None {
                    return;
                }
            }
        }

        self.winner = Player::None;
        self.finished = true;
        self.pos_x = 0;
        self.pos_y = 0;
    }
}

impl Player {
    pub fn to_ascii(&self) -> Vec<Line<'_>> {
        match self {
            Self::None => vec![
                Line::from("        "),
                Line::from("        "),
                Line::from("        "),
                Line::from("        "),
                Line::from("        "),
                Line::from("        "),
            ],
            Self::Cross => vec![
                Line::from("__   __"),
                Line::from("\\ \\ / /"),
                Line::from(" \\ V / "),
                Line::from("  > <  "),
                Line::from(" / . \\ "),
                Line::from("/_/ \\_\\"),
            ],
            Self::Nought => vec![
                Line::from("  ____  "),
                Line::from(" / __ \\ "),
                Line::from("| |  | |"),
                Line::from("| |  | |"),
                Line::from("| |__| |"),
                Line::from(" \\____/ ")
            ],
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Cross => 'X',
            Self::Nought => 'O'
        }
    }
}
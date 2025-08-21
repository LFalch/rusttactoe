#![warn(clippy::all)]

use std::{
    fmt::{self, Display},
    ops::Not,
};
use ansi_term::Colour;

use crate::ctrls::Controllers;

fn clear() {
    // TODO: Make this work on Windows #1
    // NOTE: Should work on Windows ?
    print!("{}[2J", 27 as char);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    #[inline]
    fn as_str(self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => Colour::Blue,
            Player::O => Colour::Yellow,
        }
        .bold()
        .paint(self.as_str())
        .fmt(f)
    }
}

impl Not for Player {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Board {
    board: [Option<Player>; 9],
}

macro_rules! tab {
    () => {
        "         "
    };
}
/// 9 spaces
const TAB: &str = tab!();
const WIN_CASES: [(usize, usize, usize); 8] = [(0, 1, 2), (3, 4, 5), (6, 7, 8), (0, 3, 6), (0, 4, 8), (1, 4, 7), (2, 5, 8), (2, 4, 6)];

impl Board {
    fn field_display(&self, i: usize) -> impl Display {
        struct Thing(Result<Player, usize>);
        impl Display for Thing {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    Ok(p) => p.fmt(f),
                    Err(i) => (i + 1).fmt(f),
                }
            }
        }

        Thing(self.board[i].ok_or(i))
    }
    fn draw_row(&self, i: usize) {
        let off = i * 3;
        println!(
            "{} {} | {} | {}",
            TAB,
            self.field_display(off),
            self.field_display(off + 1),
            self.field_display(off + 2),
        )
    }
    fn draw(&self) {
        const ROW_DIVIDER: &str = concat!(tab!(), "--- --- ---");
        println!();
        self.draw_row(0);
        println!("{}", ROW_DIVIDER);
        self.draw_row(1);
        println!("{}", ROW_DIVIDER);
        self.draw_row(2);
        println!();
    }
    #[inline]
    fn check_line(&self, a: usize, b: usize, c: usize) -> bool {
        match (self.board[a], self.board[b], self.board[c]) {
            (Some(a), Some(b), Some(c)) => a == b && b == c,
            _ => false,
        }
    }
    #[inline]
    fn winner(&self) -> bool {
        WIN_CASES.iter().any(|&(a, b, c)| self.check_line(a, b, c))
    }
}

mod ctrls;

fn main() {
    let mut board = Board::default();
    let mut player = Player::X;

    let controllers = match Controllers::new_from_env_args() {
        Ok(c) => c,
        Err(e) => return eprintln!("{e}"),
    };

    for turn in 1.. {
        if turn > board.board.len() {
            board.draw();
            println!("Turns out there are no available spots left.");
            println!("Game has tied.");
            break;
        }

        let play_square = loop {
            board.draw();

            let play = controllers.get_move(player, &board);
            let play_square = &mut board.board[play];
            
            if play_square.is_none() {
                break play_square;
            } else {
                println!("Invalid move. Try again.");
            }
        };
        *play_square = Some(player);
        if board.winner() {
            clear();
            println!("Player {} wins on move {}!!", player, turn);
            board.draw();
            break;
        }
        player = !player;
    }
    println!("Bye!");
}

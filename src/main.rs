mod solver;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use crate::solver::solve;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Colour {
    Red,
    Blue,
    Yellow,
    Black
}

impl Colour {
    pub const RED_CHAR: char = 'r';
    pub const BLUE_CHAR: char = 'b';
    pub const YELLOW_CHAR: char = 'y';
    pub const BLACK_CHAR: char = 'x';

    pub const fn get_char(&self) -> char {
        match &self {
            Colour::Red => Colour::RED_CHAR,
            Colour::Blue => Colour::BLUE_CHAR,
            Colour::Yellow => Colour::YELLOW_CHAR,
            Colour::Black => Colour::BLACK_CHAR
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Tile {
    Normal(Colour, u8),
    Joker
}

impl Tile {
    pub const JOKER_CHAR: &'static str = "j";

    pub fn is_joker(&self) -> bool {
        match &self {
            Tile::Joker => true,
            Tile::Normal(_, _) => false
        }
    }

    pub fn colour(&self) -> Option<Colour> {
        match &self {
            Tile::Normal(c, _) => Some(c.clone()),
            Tile::Joker => None
        }
    }

    pub fn number(&self) -> Option<u8> {
        match &self {
            Tile::Normal(_, n) => Some(*n),
            Tile::Joker => None
        }
    }

    pub fn from_str<U: AsRef<str>>(string: U) -> Result<Tile, &'static str> {
        let string = string.as_ref();
        match string.len() {
            0 => Err("No string"),
            1 => match string {
                Tile::JOKER_CHAR => Ok(Tile::Joker),
                _ => Err("Not joker")
            },
            _ => {
                Ok(Tile::Normal(
                    match string.chars().next().unwrap() {
                        Colour::RED_CHAR => Colour::Red,
                        Colour::BLUE_CHAR => Colour::Blue,
                        Colour::YELLOW_CHAR => Colour::Yellow,
                        Colour::BLACK_CHAR => Colour::Black,
                        _ => return Err("Invalid colour")
                    },
                    u8::from_str(string.chars().skip(1).collect::<String>().as_str())
                        .map_or_else(
                            |_| Err("Invalid number"),
                            |n| { if 1 <= n && n <= 13 { Ok(n) } else { Err("Number out of range") } },
                        )?
                ))
            },
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Tile::Joker => {
                let mut s = Tile::JOKER_CHAR.to_string();
                s.push(' ');
                s
            },
            Tile::Normal(colour, number) => {
                format!("{}{} ", colour.get_char(), number)
            }
        }
    }

    pub fn format_list(list: &[Tile]) -> String {
        let mut string = String::with_capacity(list.len() * 3);

        for ts in list.chunks(10) {
            for t in ts {
                string += &t.to_string();
            }
            string.push('\n');
        }

        string
    }
}

pub struct State {
    board: VecDeque<Tile>,
    hand: VecDeque<Tile>,
}

impl State {
    pub fn new() -> State {
        State { board: VecDeque::new(), hand: VecDeque::new() }
    }

    fn sorted_vec_insert(vec: &mut VecDeque<Tile>, new_tile: Tile)  {
        let location = vec.binary_search(&new_tile);
        match location {
            Ok(i) => vec.insert(i, new_tile),
            Err(i) =>vec.insert(i, new_tile)
        };
    }

    pub fn add_to_board(&mut self, tile: Tile) {
        Self::sorted_vec_insert(&mut self.board, tile)
    }

    pub fn add_to_hand(&mut self, tile: Tile) {
        Self::sorted_vec_insert(&mut self.hand, tile)
    }

    pub fn format(&mut self) -> String {
        let mut string = "Board:\n".to_string();
        string += &Tile::format_list(self.board.make_contiguous());
        string += "Hand:\n";
        string += &Tile::format_list(self.hand.make_contiguous());

        string
    }

    pub fn board(&mut self) -> &mut VecDeque<Tile> { &mut self.board }
    pub fn hand(&mut self) -> &mut VecDeque<Tile> { &mut self.hand }
}

fn main() {
    time_graph::enable_data_collection(true);

    let mut state = State::new();

    let board_init = vec!["r1", "r4", "r12", "b1", "b4", "b12", "y1", "y2", "y3", "x1",
                          "x1", "x2", "x3", "x4", "x4", "x6", "x8", "x12", "j"];
    for t in board_init { state.add_to_board(Tile::from_str(t).unwrap()); }
    // let hand_init = vec!["y6", "y6", "b9", "x9", "r7", "y7", "r1", "r2", "x8", "x12", "r13"];
    // for t in hand_init { state.add_to_hand(Tile::from_str(t).unwrap()); }
    println!("{}", solve(&state).format());

    let graph = time_graph::get_full_graph();
    println!("{}", graph.as_table());

    return;

    loop {
        println!("\n's' to solve");
        println!("Prefix 'b' to add a tile to the board");
        println!("Prefix 'h' to add a tile to your hand");
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.len() == 0 { println!("Provide an input"); continue; }

        let code = input.chars().next().unwrap();
        if code == 'b' {
            let t = Tile::from_str(&input[1..]);
            match t {
                Ok(t) => state.add_to_board(t),
                Err(e) => println!("{e}")
            }
        }
        else if code == 'h' {
            let t = Tile::from_str(&input[1..]);
            match t {
                Ok(t) => state.add_to_hand(t),
                Err(e) => println!("{e}")
            }
        }
        else if code == 's' {
            println!("{}", solve(&state).format())
        }
        else {
            println!("Invalid input"); continue;
        }

        println!();
        println!("{}", state.format());
    }
}

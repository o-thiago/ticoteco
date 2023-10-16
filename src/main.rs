use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Player(char),
}

struct Question<'a>(&'a str);

impl Question<'_> {
    fn ask<T: Display + FromStr + Debug>(&self) -> T
    where
        T::Err: Debug,
    {
        print!("{}", self.0);
        text_io::read!()
    }

    fn ask_forcefully<T: Display + FromStr + Debug>(
        &self,
        condition_handler: impl Fn(&T) -> bool,
        on_error: impl Fn(),
    ) -> T
    where
        T::Err: Debug,
    {
        loop {
            let answer: T = self.ask();
            if condition_handler(&answer) {
                return answer;
            } else {
                on_error();
            }
        }
    }
}

fn main() {
    let players_amount: usize = Question("How many players? ").ask_forcefully(
        |amount| amount > &1,
        || println!("There must be at least 2 players!"),
    );

    let mut player_chars = HashSet::new();

    for i in 0..players_amount {
        let player_name: char = Question(&format!("Enter the name of player {}: ", i + 1))
            .ask_forcefully(
                |player_name| !(player_chars.contains(player_name)),
                || {
                    println!("This name is already being used!");
                },
            );

        player_chars.insert(player_name);
    }

    let board_size: usize = Question("Enter the size of the board: ").ask_forcefully(
        |size| size >= &players_amount,
        || {
            println!(
                "The board must be big enough for {} players!",
                players_amount
            )
        },
    );

    let mut board_matrix = vec![vec![Cell::Empty; board_size]; board_size];

    for player in player_chars.iter().cycle() {
        println!();
        println!("Player {}'s turn!", player);

        let ask_position = |position_of: &str| -> usize {
            let position_value: usize =
                Question(format!("Enter the {} position: ", position_of).as_str()).ask_forcefully(
                    |position| position <= &board_size,
                    || println!("The position must be less than {}!", board_size),
                );
            position_value - 1
        };

        loop {
            let row = ask_position("row");
            let col = ask_position("column");

            let cell_state = &board_matrix[row][col];
            if cell_state == &Cell::Empty {
                board_matrix[row][col] = Cell::Player(*player);
                break;
            } else {
                println!("The cell is already occupied!");
            }
        }

        for row in board_matrix.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!("_"),
                    Cell::Player(player) => print!("{}", player),
                }
            }
            println!();
        }

        let check_won = |condition: bool| -> bool {
            if condition {
                println!("{} won!", player);
                std::process::exit(0);
            }
            return condition;
        };

        let won_horizontally = check_won(
            board_matrix
                .iter()
                .any(|row| row.iter().all(|cell| cell == &Cell::Player(*player))),
        );

        if !won_horizontally {
            continue;
        }

        let board_iter = || 0..board_size;

        let won_vertically =
            check_won(board_iter().any(|col| {
                (0..board_size).all(|row| board_matrix[row][col] == Cell::Player(*player))
            }));

        if !won_vertically {
            continue;
        }

        let won_diagonally = check_won(
            board_iter().all(|i| board_matrix[i][i] == Cell::Player(*player))
                || board_iter()
                    .all(|i| board_matrix[i][board_size - i - 1] == Cell::Player(*player)),
        );

        if !won_diagonally {
            continue;
        }
    }
}

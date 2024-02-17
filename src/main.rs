use rand::distributions::Uniform;
use rand::Rng;
use std::error::Error;
use std::io;
use std::io::Write;
use std::result::Result;

#[derive(Debug, Clone, Copy)]
enum Move {
    Paper,
    Rock,
    Scissor,
}

impl Move {
    fn wins_against(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Move::Rock, Move::Scissor)
            | (Move::Paper, Move::Rock)
            | (Move::Scissor, Move::Paper) => Some(*self),
            _ => None,
        }
    }
}

impl TryFrom<String> for Move {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "R" => Ok(Move::Rock),
            "S" => Ok(Move::Scissor),
            "P" => Ok(Move::Paper),
            _ => Err(From::from("Invalid moviment specified!")),
        }
    }
}

fn get_user_move() -> Result<Move, Box<dyn Error>> {
    let mut s = String::new();

    print!(" Move: ");
    io::stdout().flush()?;

    io::stdin()
        .read_line(&mut s)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Move::try_from(s.trim().to_string())
}

fn get_computer_move() -> Move {
    match rand::thread_rng().sample(Uniform::new(0, 2)) {
        0 => Move::Rock,
        1 => Move::Scissor,
        _ => Move::Paper,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("R = Rock | P = Paper | S = Scissor");

    loop {
        let user_move = get_user_move()?;
        let computer_move = get_computer_move();

        println!("-> User move: {:?}", &user_move);
        println!("-> Computer move: {:?}", &computer_move);
        print!("--> Result: ");

        if user_move.wins_against(&computer_move).is_some() {
            println!("{:?} Wins!", &user_move)
        } else if computer_move.wins_against(&user_move).is_some() {
            println!("{:?} Wins!", &computer_move)
        } else {
            println!("it a tie!");
        }
    }

}

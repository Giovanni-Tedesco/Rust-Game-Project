use std::f32::NEG_INFINITY;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};
// Some fooling around with traits
#[derive(Clone, Debug)]
struct Environment {
	state: Vec< Vec<i32> >,
	turn: i8,
}

struct Nature<'a> {
    env: &'a mut Environment,
    agents: &'a Vec<Agent>,
}

// Agent
struct Agent {}

impl Agent {

    fn new() -> Agent {
        return Agent{};
    }

    fn action(&self, env: &Environment) -> String {
        let legal_moves: Vec<String> = env.legal_moves();
        let mut rng = rand::thread_rng();
        let range = Uniform::from(1..(legal_moves.len()));
        let throw = range.sample(&mut rng);

        let ret: String = legal_moves[throw].clone();
        println!("{}", ret);

        return ret;
    }

    fn minimax(&self, env: &Environment) -> i8 {

        if env.is_win() {
            if env.get_turn() == 1 {
                return 1;
            } else {
                return -1;
            }
        } else if env.is_draw() {
            return 0;
        } else {
            let player: i8 = env.get_turn();

            if player == 1 {
                let mut max_num: i8 = f32::NEG_INFINITY as i8;
                for next_move in env.legal_moves() {
                    let mut new_environment: Environment = env.clone();
                    new_environment.update(next_move);
                    let score = self.minimax(&new_environment);
                    if score >= max_num {
                        max_num = score
                    }
                }
                return max_num;
            } else {
                let mut min_num: i8 = f32::INFINITY as i8;
                for next_move in env.legal_moves() {
                    let mut new_environment: Environment = env.clone();
                    new_environment.update(next_move);
                    let score: i8 = self.minimax(&new_environment);
                    if score <= min_num {
                        min_num = score;
                    }
                }

                return min_num;
            }
        }
    }
}

// Abstraction / Interface for the Environment
trait State {
    // What the move will do. Moves will be in the form of strings
    fn update(&mut self, player_move: String);
    // Has the board reached a terminal position?
    // Returns the player who won, either -1, 1, or 0
    fn is_win(&self) -> bool;
    fn is_draw(&self) -> bool;
    // Return a list of legal move strings that the engine can play
    // This will vary based on the game being played
    fn legal_moves(&self) -> Vec< String >;

    fn get_turn(&self) -> i8;
}

// A sample environment implementation
impl Environment {
	fn new(size: i32) -> Environment {
		return Environment {
			state: vec![vec![0; size as usize]; size as usize],
			turn: 1
		}
    }

    fn update_turn(&mut self) {
        if self.turn == 1 {
            self.turn = 2
        } else {
            self.turn = 1
        }
    }
}

// More or less game logic ?
impl State for Environment {

    fn update(&mut self, player_move: String) {
        // Note this is where 1 is 'X' and 2 is 'O'
        // ASCII Go brrrrr
        let mv: &[u8] = player_move.as_bytes();
        let mv1: usize = ((mv[0] as u8 - 97)) as usize;

        let mv2: usize = (mv[1] as u8 - 49) as usize;

        self.state[mv1][mv2] = self.get_turn() as i32;
        self.update_turn();
    }

    fn get_turn(&self) -> i8 {
        return self.turn;
    }

    fn is_draw(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.state[i][j] == 0 {
                    return false
                }
            }
        }

        return true;
    }

    fn is_win(&self) -> bool {

        // Rows
        for i in 0..3 {
            if all_equal(&self.state[i], 1) {
                // println!("Row Wins");
                return true;
            } else if all_equal(&self.state[i], 2) {
                // println!("Row Wins");
                return true;
            }
        }
        // Columns
        for j in 0..3 {
            let mut moves: Vec<i32> = Vec::new();
            for i in 0..3 {
                moves.push(self.state[i][j]);
            }
            if all_equal(&moves, 1) {
                // println!("Column Wins");
                return true;
            } else if all_equal(&moves, 2) {
                // println!("Column Wins");
                return true;
            }
        }

        // Diagonals
        // Left to right,
        let mut i = 0;
        let mut j = 0;
        let mut moves: Vec<i32> = Vec::new();
        while i < 3 && j < 3 {
            moves.push(self.state[i][j]);
            i += 1;
            j += 1;
        }

        if all_equal(&moves, 1) {
            // println!("Diagonal LtR wins");
            return true;
        } else if all_equal(&moves, 2) {
            // println!("Diagonal LtR wins");
            return true;
        }

        moves.clear();
        // Top right to bottom left
        i = 0;
        j = 3;

        while i < 3 {
            // println!("{}", self.state[i][j]);
            moves.push(self.state[i][j - 1]);
            i += 1;
            j -= 1;
        }

        if all_equal(&moves, 1) {
            return true;
        }

        if all_equal(&moves, 2) {
            return true;
        }

        for i in 0..3 {
            for j in 0..3 {
                if self.state[i][j] != 0 {
                    return false;
                }
            }
        }

        return false;
    }
    // Should be as simple as finding what is missing.
    fn legal_moves(&self) -> Vec<String> {
        let mut ret_vec: Vec<String> = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                if self.state[i][j] == 0 {
                    ret_vec.push(convert(i as i32, j as i32));
                }
            }
        }

        return ret_vec;
    }

}


fn all_equal(row: &Vec< i32 >, search: i32) -> bool {

    for element in row {
        if *element != search {
            return false;
        }
    }

    return true;
}

fn convert(i: i32, j: i32) -> String {

    let x: char = (i as u8 + 97) as char;
    let y: char = (j as u8 + 49) as char;

    let mut ret: String = String::new();

    ret.push(x);
    ret.push(y);

    return ret;

}
fn main() {

    let mut gameboard = Environment::new(3);

    let a1 = Agent::new();

    // gameboard.update(String::from("a2"));
    // gameboard.update(String::from("a3"));
    // gameboard.update(String::from("b2"));
    // gameboard.update(String::from("b3"));
    // gameboard.update(String::from("c1"));

    println!("{}", a1.minimax(&gameboard));


    // println!("{}", gameboard.is_terminal());

    // gameboard.update(a1.action(&gameboard));


	for i in 0..3 {
		for j in 0..3 {
			print!("|{:?}|", gameboard.state[i][j]);
		}
		println!("\n---------");
    }


}

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
struct Agent {
    max_depth: i32,
}


impl Agent {

    fn new(depth: i32) -> Agent {
        return Agent{
            max_depth: depth
        };
    }

    fn action(&self, env: &Environment) -> String {

        let legal_moves: Vec<String> = env.legal_moves();

        if legal_moves.len() == 0{
            return String::from("");
        } else if legal_moves.len() == 1 {
            return legal_moves[0].clone();
        }

        let turn = env.get_turn();

        let mut current_score = 0.0;
        if turn == 1 {
            current_score = -10.0
        } else {
            current_score = 10.0;
        }

        // TODO: Make with index instead of string.
        let mut output_move = String::from("");

        // TODO: Instead find all good moves and pick one randomly.
        for next_move in legal_moves {
            let mut internal_env: Environment = env.clone();
            internal_env.update(&next_move.clone());
            let pos_score = self.alphabeta(&internal_env, 10, f64::NEG_INFINITY, f64::INFINITY);

            if turn == 1 {
                if pos_score > current_score {
                    current_score = pos_score;
                    output_move = next_move.clone();
                }
            } else {
                if pos_score < current_score {
                    current_score = pos_score;
                    output_move = next_move.clone();
                }
            }
        }
        return output_move;
    }

    fn play_random(env: &Environment) -> f64 {
        let result = env.is_win();
        if env.is_draw() {
            return 0.0;
        } else if result != 0.0{
            return result
        } else {
            let mut temp_env = env.clone();
            let mut rng = rand::thread_rng();
            let moves = temp_env.legal_moves();
            let range = Uniform::from(0..moves.len());


            let num = range.sample(&mut rng);

            temp_env.update(&moves[num].clone());

            return Agent::play_random(&temp_env);
        }

    }

    fn score_position(&self, env: &Environment) -> f64 {
        // Play randomly 100 times and score the position.
        let mut sum = 0.0;
        for i in 0..100 {
            sum += Agent::play_random(env);
        }

        return sum / 100.0 ;
    }

    fn alphabeta(&self, env: &Environment, depth: i32, alpha: f64, beta: f64) -> f64 {

        // TODO: Implement a cache of some sort.
        // TODO: Improve scoring function.
        // TODO: Improve end game checking. Shouldn't have to run all the end game
        //       calculations every time.

        let result = env.is_win();

        if depth == 0 {
            return self.score_position(env)
        }
        if result != 0.0 {
            if env.get_turn() == 1 {
                return 1.0;
            } else {
                return -1.0;
            }
        } else if env.is_draw() {
            return 0.0;
        } else {
            let player: i8 = env.get_turn();

            let mut value;

            let new_environments = env
            .legal_moves()
            .iter()
            .map(|a | env.what_if(a))
            .collect::<Vec<_>>();

            if player == 1 {
                value = f64::NEG_INFINITY;
                let mut next_alpha = alpha;
                for new_environment in new_environments {

                    let score = self.alphabeta(&new_environment, depth - 1, next_alpha, beta);

                    if score > value {
                        value = score
                    }
                    next_alpha = next_alpha.max(value);

                    if next_alpha >= beta {
                        break;
                    }
                }
            } else {
                value = f64::INFINITY;
                let mut next_beta= beta;

                for new_environment in new_environments {
                    let score = self.alphabeta(&new_environment, depth, alpha, next_beta);
                    // let score = self.minimax(&new_environment, depth);
                    if score < value {
                        value = score;
                    }

                    next_beta = next_beta.min(value);

                    if next_beta <= alpha {
                        break;
                    }
                }
            }

            return value;
        }
    }


    fn minimax(&self, env: &Environment, depth: i32) -> f64 {
        // TODO: Implement a depth function / scoring function.
        let result = env.is_win();

        if depth == 0 {
            return self.score_position(env)
        }
        if result != 0.0 {
            if env.get_turn() == 1 {
                return 1.0;
            } else {
                return -1.0;
            }
        } else if env.is_draw() {
            return 0.0;
        } else {
            let player: i8 = env.get_turn();

            if player == 1 {
                let mut max_num = f64::NEG_INFINITY;

                for next_move in env.legal_moves() {

                    let mut new_environment: Environment = env.clone();

                    new_environment.update(&next_move);

                    let score = self.minimax(&new_environment, depth - 1);

                    if score >= max_num {
                        max_num = score
                    }
                }
                return max_num;
            } else {
                let mut min_num  = f64::INFINITY;
                for next_move in env.legal_moves() {
                    let mut new_environment: Environment = env.clone();
                    new_environment.update(&next_move);
                    let score = self.minimax(&new_environment, depth - 1);
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
    fn update(&mut self, player_move: &String);
    // Has the board reached a terminal position?
    // Returns the player who won, either -1, 1, or 0
    // Checks if the position is a win
    fn is_win(&self) -> f64;
    // Checks if the position is a draw
    fn is_draw(&self) -> bool;
    // Return a list of legal move strings that the engine can play
    // This will vary based on the game being played
    // fn result(&self) -> i8;
    fn what_if(&self, action: &String) -> Environment;

    fn legal_moves(&self) -> Vec< String >;
    // Gets the players turn
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

    fn update(&mut self, player_move: &String) {
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

    fn what_if(&self, action: &String) -> Environment {
        let mut new_board = self.clone();
        new_board.update(action);
        return new_board
    }

    fn is_win(&self) -> f64 {

        // Rows
        for i in 0..3 {
            if all_equal(&self.state[i], 1) {
                // println!("Row Wins");
                return 1.0;
            } else if all_equal(&self.state[i], 2) {
                // println!("Row Wins");
                return -1.0;
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
                return 1.0;
            } else if all_equal(&moves, 2) {
                // println!("Column Wins");
                return -1.0;
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
            return 1.0;
        } else if all_equal(&moves, 2) {
            // println!("Diagonal LtR wins");
            return -1.0;
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
            return 1.0;
        }

        if all_equal(&moves, 2) {
            return -1.0;
        }

        return 0.0;
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

fn play(env: &mut Environment, a1: &Agent, a2: &Agent) {

    while !(env.is_draw() || env.is_win() != 0.0) {
        let turn = env.get_turn();

        if turn == 1 {
            env.update(&a1.action(&env));
        } else {
            env.update(&a2.action(&env));
        }

        display(&env);
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

fn display(gameboard: &Environment) {
	for i in 0..3 {
		for j in 0..3 {
			print!("|{:?}|", gameboard.state[i][j]);
		}
		println!("\n---------");
    }

    println!("\n\n")
}

fn main() {

    let mut gameboard = Environment::new(3);

    let a1 = Agent::new(10);
    let a2 = Agent::new(10
    );

    play(&mut gameboard, &a1, &a2);

}

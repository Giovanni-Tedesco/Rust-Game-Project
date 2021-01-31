// Some fooling around with traits
struct Environment {
	state: Vec< Vec<i32> >,
	turn: i32,
}

struct Nature<'a> {
    env: &'a mut Environment,
    agents: &'a Vec<Agent>,
}

// Agent
struct Agent {

}

impl Agent {

    fn action(env: &Environment) -> String {
        return String::from("")

    }
}

// Abstraction / Interface for the Environment
trait State {

    // What the move will do. Moves will be in the form of strings
    fn update(&mut self, player_move: String);

    // Has the board reached a terminal position?
    fn is_terminal(&self) -> bool;

    // Return a list of legal move strings that the engine can play
    // This will vary based on the game being played
    fn legal_moves(&self) -> Vec< String >;
}

// A sample environment implementation
impl Environment {
	fn new(size: i32) -> Environment {
		return Environment {
			state: vec![vec![0; size as usize]; size as usize],
			turn: 1
		}
    }

    fn get_turn(&self) -> i32 {
        return self.turn;
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

        self.state[mv1][mv2] = self.get_turn();
        self.update_turn();
    }

    fn is_terminal(&self) -> bool {
        return false;
    }

    // Should be as simple as finding what is missing.
    fn legal_moves(&self) -> Vec<String> {
        let ret_vec: Vec<String> = Vec::new();

        return Vec::new();


    }

}

fn convert(i: i32, j: i32) -> String {
    let x: char = 'a';
    let y: char = '1';
    let mut ret: String = String::new();

    ret.push(x);
    ret.push(y);

    println!("{}", ret);
    return ret;

}
fn main() {

    let mut gameboard = Environment::new(3);

    gameboard.update(String::from("a1"));
    gameboard.update(String::from("b3"));


	for i in 0..3 {
		for j in 0..3 {
			print!("|{:?}|", gameboard.state[i][j]);
		}
		println!("\n---------");
    }
    convert(0, 2);

    for i in 0..3 {
        for j in 0..3 {
            if gameboard.state[i][j] == 0 {

            }
        }
    }
	// println!("|{:?}|", gameboard.turn);
    println!("Hello, world!");
}

use core::fmt;
use std::time::Instant;

type BoardArray = [u8; 16];


struct BoardState {
    board: BoardArray,
    left_done: bool,
    right_done: bool,
    up_done: bool,
    down_done: bool,
    zero_pos: usize,
    range: u32,
    hash: usize
}


impl BoardState {
    fn is_diff_board(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            return true;
        }

        for i in 0..self.board.len() {
            if self.board[i] != other.board[i] {
                return  true;
            }
        }

        return false;
    }

    fn move_right(&mut self) -> Option<Self> {
        self.right_done = true;

        let (x, _) = get_xy_pos(self.zero_pos);

        if x == 3 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos + 1];
        new_board[self.zero_pos + 1] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(new_board, self.zero_pos + 1, self.range + 1));
    }
    
    fn move_left(&mut self) -> Option<Self> {
        self.left_done = true;

        let (x, _) = get_xy_pos(self.zero_pos);

        if x == 0 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos - 1];
        new_board[self.zero_pos - 1] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(new_board, self.zero_pos - 1, self.range + 1));
    }

    fn move_up(&mut self) -> Option<Self> {
        self.up_done = true;

        let (_, y) = get_xy_pos(self.zero_pos);

        if y == 0 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos - 4];
        new_board[self.zero_pos - 4] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(new_board, self.zero_pos - 4, self.range + 1));
    }


    fn move_down(&mut self) -> Option<Self> {
        self.down_done = true;

        let (_, y) = get_xy_pos(self.zero_pos);

        if y == 3 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos + 4];
        new_board[self.zero_pos + 4] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(new_board, self.zero_pos + 4, self.range + 1));
    }

    fn can_move(&self) -> bool {
        return !self.left_done || !self.right_done || !self.up_done || !self.down_done;
    }

    fn do_move(&mut self) -> [Option<BoardState>; 4] {
        return [
            self.move_left(),
            self.move_right(),
            self.move_up(),
            self.move_down()
        ];
    }

    fn is_single_step_diff(&self, other: &Self) -> bool {
        if self.zero_pos == other.zero_pos {
            return false;
        }

        let mut changes_count: usize = 0;

        for i in 0..self.board.len() {
            if self.board[i] != other.board[i] {
                changes_count += 1;
            }
        }

        return changes_count == 2;
    }

    fn diff(&self, other: &Self) -> usize {
        let mut changes_count: usize = 0;

        for i in 0..self.board.len() {
            if self.board[i] != other.board[i] {
                changes_count += 1;
            }
        }

        return changes_count;
    }

    fn new(board: BoardArray, zero_pos: usize, range: u32) -> Self {
        let mut hash: usize = 0;

        for i in 0..board.len() {
            hash = (i + 1) * board[i] as usize;
        }


        return BoardState {
            board,
            zero_pos,
            range,
            hash,
            left_done: false,
            right_done: false,
            up_done: false,
            down_done: false,
        };
    }

    fn new_from_board(board: BoardArray) -> Self {
        let zero_pos = board.iter().position(|x| *x == 0).unwrap();

        return Self::new(board, zero_pos, 0);
    }
}


impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{:02} {:02} {:02} {:02}\n{:02} {:02} {:02} {:02}\n{:02} {:02} {:02} {:02}\n{:02} {:02} {:02} {:02}\n",
            self.board[0], self.board[1], self.board[2], self.board[3],
            self.board[4], self.board[5], self.board[6], self.board[7],
            self.board[8], self.board[9], self.board[10], self.board[11],
            self.board[12], self.board[13], self.board[14], self.board[15],
        )
    }
}

fn main() {
    let initial_state = BoardState::new_from_board(
        [
            01, 02, 03, 04,
            05, 06, 07, 08,
            09, 10, 11, 12,
            13, 14, 15, 00
        ]
    );

    let end_state = BoardState::new_from_board(
        [
            01, 03, 10, 04, 
            05, 02, 00, 07, 
            09, 11, 06, 08, 
            13, 14, 15, 12
        ]
    );

    let start_time = Instant::now();
    calculate(initial_state, end_state);
    let duration = start_time.elapsed();

    println!("Calculation time: {:?}", duration);
}

fn calculate(inital_state: BoardState, end_state: BoardState) {
    let mut states: Vec<BoardState> = Vec::with_capacity(100000);
    states.push(inital_state);

    let mut solution_found = false;

    while !solution_found {
        let not_activated_state = states.iter_mut()
            .find(|x| x.can_move())
            .unwrap();

        let unmoved_state = not_activated_state.do_move();

        for new_state in unmoved_state {
            if let Some(new_state) = new_state {
                let old_index = states.iter().position(|x| {
                    x.range > new_state.range && x.is_diff_board(&new_state) 
                });

                if let Some(old_index) = old_index {
                    states.remove(old_index);
                }

                solution_found = solution_found || !end_state.is_diff_board(&new_state);
                states.push(new_state);
                
                if states.len() % 1000 == 0 {
                    let min = states.iter().min_by_key(|x| x.diff(&end_state));
                    println!("Current states: {}", states.len());
                    println!("Min diff: {}", min.unwrap().diff(&end_state));
                }
            }
            
        }
    }

    print_result(states, end_state);   
}

fn print_result(states: Vec<BoardState>, end_state: BoardState) {
    let mut state = states.iter().find(|x| !x.is_diff_board(&end_state));
    

    while let Some(unwrapped_state) = state {
        println!("{}", unwrapped_state);

        state = states.iter().find(|x| x.range < unwrapped_state.range && unwrapped_state.is_single_step_diff(x));
    }
}

fn get_xy_pos(zero_pos: usize) -> (usize, usize) {
    let x = zero_pos % 4;
    let y = (zero_pos - x) / 4;

    return (x, y);
}


#[test]
fn test_xy_pos() {
    assert_eq!((0, 0), get_xy_pos(0));
    assert_eq!((3, 2), get_xy_pos(11));
 }

#[test]
fn test_movement() {
    let mut board_state = BoardState::new(
        [
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 10, 11, 12,
            13, 14, 15, 0
        ], 15, 0);

    let mut new_board = board_state.move_left().unwrap();
    assert_eq!(new_board.board[14], 0);
    assert_eq!(new_board.board[15], 15);

    let new_board = new_board.move_right().unwrap();
    assert_eq!(new_board.board[14], 15);
    assert_eq!(new_board.board[15], 0);

    let mut new_board = board_state.move_up().unwrap();
    assert_eq!(new_board.board[11], 0);
    assert_eq!(new_board.board[15], 12);
    let new_board = new_board.move_down().unwrap();
    assert_eq!(new_board.board[11], 12);
    assert_eq!(new_board.board[15], 0);
}

#[test]
fn generate_test_movement() {
    let mut board_state = BoardState::new(
        [
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 10, 11, 12,
            13, 14, 15, 0
        ], 15, 0);

    let mut new_state = board_state.move_up().unwrap();
    let mut new_state = new_state.move_up().unwrap();
    let mut new_state = new_state.move_left().unwrap();
    let mut new_state = new_state.move_left().unwrap();
    let mut new_state = new_state.move_down().unwrap();
    let new_state = new_state.move_right().unwrap();

    println!("{}", new_state);
    
}

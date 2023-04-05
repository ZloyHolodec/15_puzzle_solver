use core::fmt;
use std::{cmp::min, time::Instant};

const BOARD_SIZE: usize = 4;

type BoardArray = [u8; BOARD_SIZE * BOARD_SIZE];

#[derive(Clone)]
struct BoardState {
    board: BoardArray,
    left_done: bool,
    right_done: bool,
    up_done: bool,
    down_done: bool,
    zero_pos: usize,
    range: u32,
    hash: usize,
}

impl BoardState {
    fn is_diff_board(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            return true;
        }

        for i in 0..self.board.len() {
            if self.board[i] != other.board[i] {
                return true;
            }
        }

        return false;
    }

    fn move_right(&mut self) -> Option<Self> {
        self.right_done = true;

        let (x, _) = get_xy_pos(self.zero_pos);

        if x == BOARD_SIZE - 1 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos + 1];
        new_board[self.zero_pos + 1] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(
            new_board,
            self.zero_pos + 1,
            self.range + 1,
        ));
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

        return Some(BoardState::new(
            new_board,
            self.zero_pos - 1,
            self.range + 1,
        ));
    }

    fn move_up(&mut self) -> Option<Self> {
        self.up_done = true;

        let (_, y) = get_xy_pos(self.zero_pos);

        if y == 0 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos - BOARD_SIZE];
        new_board[self.zero_pos - BOARD_SIZE] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(
            new_board,
            self.zero_pos - BOARD_SIZE,
            self.range + 1,
        ));
    }

    fn move_down(&mut self) -> Option<Self> {
        self.down_done = true;

        let (_, y) = get_xy_pos(self.zero_pos);

        if y == BOARD_SIZE - 1 {
            return None;
        }

        let mut new_board = self.board.clone();
        let buf = new_board[self.zero_pos + BOARD_SIZE];
        new_board[self.zero_pos + BOARD_SIZE] = 0;
        new_board[self.zero_pos] = buf;

        return Some(BoardState::new(
            new_board,
            self.zero_pos + BOARD_SIZE,
            self.range + 1,
        ));
    }

    fn can_move(&self) -> bool {
        return !self.left_done || !self.right_done || !self.up_done || !self.down_done;
    }

    fn do_move(&mut self) -> [Option<BoardState>; 4] {
        return [
            self.move_left(),
            self.move_right(),
            self.move_up(),
            self.move_down(),
        ];
    }

    fn reset_state(&mut self, other: &BoardState) {
        self.range = other.range;
        self.left_done = false;
        self.right_done = false;
        self.up_done = false;
        self.down_done = false;
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
        let mut distance: usize = 0;

        for i in 0..self.board.len() {
            let my_value = self.board[i];
            let (my_x, my_y) = get_xy_pos(i);

            let other_pos = other.board.iter().position(|x| *x == my_value).unwrap();
            let (other_x, other_y) = get_xy_pos(other_pos);

            distance += (my_x as i32 - other_x as i32).pow(2) as usize;
            distance += (my_y as i32 - other_y as i32).pow(2) as usize;
        }

        return distance;
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
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02} {:02} {:02} {:02}\n\
            {:02} {:02} {:02} {:02}\n\
            {:02} {:02} {:02} {:02}\n\
            {:02} {:02} {:02} {:02}\n",
            self.board[0], self.board[1], self.board[2], self.board[3],
            self.board[4], self.board[5], self.board[6], self.board[7],
            self.board[8], self.board[9], self.board[10], self.board[11],
            self.board[12], self.board[13], self.board[14], self.board[15],
        )
    }
}

fn main() {
    // this is actually what we want to have at the end
    // 00 is empty block
    #[rustfmt::skip]
    let initial_state = BoardState::new_from_board(
        [
            01, 02, 03, 04,
            05, 06, 07, 08,
            09, 10, 11, 12,
            13, 14, 15, 00
        ]
    );

    // this is how we starting(what you see on screen at begining)
    // 00 is empty block
    #[rustfmt::skip]
    let end_state = BoardState::new_from_board(
        [
            05, 01, 02, 04, 
            14, 06, 03, 07, 
            13, 15, 10, 08,
            09, 00, 12, 11
        ]
    );

    let start_time = Instant::now();
    calculate(initial_state, end_state);
    let duration = start_time.elapsed();

    println!("Calculation time: {:?}", duration);
}

fn calculate(inital_state: BoardState, end_state: BoardState) {
    let mut states: Vec<Vec<BoardState>> = Vec::with_capacity(BOARD_SIZE.pow(2));

    for _ in 0..BOARD_SIZE.pow(2) {
        states.push(Vec::with_capacity(1_000_000));
    }

    states[inital_state.zero_pos].push(inital_state);

    let mut solution_found = false;
    let mut new_zero_pos: usize = 0;
    let mut states_amount: usize = 0;
    let mut min_range_on_chunk: u32 = 9999;

    while !solution_found {
        let unmoved_state =
            get_not_activated_state(&mut states, new_zero_pos % BOARD_SIZE.pow(2), &end_state);
        new_zero_pos += 1;

        for new_state in unmoved_state {
            if let Some(new_state) = new_state {
                min_range_on_chunk = min(min_range_on_chunk, new_state.range);
                solution_found = solution_found || !end_state.is_diff_board(&new_state);

                states_amount =
                    update_states_list(&mut states[new_state.zero_pos], new_state, states_amount);

                if states_amount % 1000 == 0 {
                    log_amount_of_states(states_amount, min_range_on_chunk);
                    min_range_on_chunk = 9999;
                }
            }
        }
    }

    print_result(states, end_state);
}

fn update_states_list(
    states: &mut Vec<BoardState>,
    new_state: BoardState,
    mut states_amount: usize,
) -> usize {
    let old_index = states
        .iter()
        .position(|x| x.range > new_state.range && !x.is_diff_board(&new_state));

    if let Some(old_index) = old_index {
        states[old_index].reset_state(&new_state);
    } else {
        states.push(new_state);
        states_amount += 1;
    }

    states_amount
}

fn log_amount_of_states(states_amount: usize, min_dist: u32) {
    println!("Current states: {}", states_amount);
    println!("Min diff: {}", min_dist);
}

fn get_not_activated_state(
    arrays: &mut Vec<Vec<BoardState>>,
    index: usize,
    end_state: &BoardState,
) -> [Option<BoardState>; 4] {
    let result_board = arrays[index]
        .iter_mut()
        .filter(|x| x.can_move())
        .min_by_key(|x| x.diff(end_state));

    match result_board {
        Some(result) => result.do_move(),
        None => [None, None, None, None],
    }
}

fn print_result(states: Vec<Vec<BoardState>>, end_state: BoardState) {
    let mut all_states: Vec<BoardState> = Vec::new();

    for mut state in states {
        all_states.append(&mut state);
    }

    let mut state = all_states.iter().find(|x| !x.is_diff_board(&end_state));

    while let Some(unwrapped_state) = state {
        println!("{}", unwrapped_state);

        state = all_states
            .iter()
            .find(|x| x.range < unwrapped_state.range && unwrapped_state.is_single_step_diff(x));
    }
}

fn get_xy_pos(zero_pos: usize) -> (usize, usize) {
    let x = zero_pos % BOARD_SIZE;
    let y = (zero_pos - x) / BOARD_SIZE;

    return (x, y);
}

#[test]
fn test_xy_pos() {
    assert_eq!((0, 0), get_xy_pos(0));
    assert_eq!((3, 2), get_xy_pos(11));
}

#[test]
fn test_movement() {
    #[rustfmt::skip]
    let mut board_state = BoardState::new_from_board(
        [
            01, 02, 03, 04,
            05, 06, 07, 08,
            09, 10, 11, 12,
            13, 14, 15, 0
        ]
    );

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

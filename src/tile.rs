use transpose;
use rand::Rng;
use rand::prelude::ThreadRng;

use std::fmt::{Display, Formatter};

pub enum Direction {
    Left = 0, Right = 1, Up = 2, Down
}

pub struct Board {
    values: Vec<u16>,
    rng: ThreadRng,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            values : vec![0; 16], 
            rng: rand::thread_rng(),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
         self.draw(f)
    }
}

// No need to impl since move up/dn can be achieved by using two extra in-place transpose actions 
// fn move_row_up(row: &Vec<u16>) -> Vec<u16> {
//     unimplemented!()
// }
// fn move_row_down(row: &Vec<u16>) -> Vec<u16> {
//     unimplemented!()
// }

fn move_row_left(row: &Vec<u16>) -> Vec<u16> {
    // TODO: use reverse iterator to avoid extra one clone and two reverse opeartions 
    let mut row_rev = row.clone();
    row_rev.reverse();
    let mut result = move_row_right(&row_rev);
    result.reverse();
    result
}

fn move_row_right(row: &Vec<u16>) -> Vec<u16> {
    let mut pre_value = std::u16::MAX; 
    let mut new_row = Vec::with_capacity(row.len());
    let mut pre_idx = std::usize::MAX;
    let row_len = row.len();
    for (idx, val) in row.into_iter().enumerate() {
        let mut tmp_val = *val;
        if *val == 0 {
        } else {
           if pre_value == std::u16::MAX {
               // set first element
               pre_value = *val;
               pre_idx = idx;
               
           } else {
               // merge
               if *val == pre_value {
                  new_row[pre_idx] = 0; 
                  pre_idx = std::usize::MAX;
                  pre_value = std::u16::MAX;
                  tmp_val = *val * 2;
               } else {
                   // set first element 
                   pre_value = *val;
                   pre_idx = idx;
               }
           }
        }
        new_row.push(tmp_val);
    }

    // move
    let mut l = 0;
    while l < row_len {
        // only need to swap the zero from right to left
        if new_row[l] == 0 {
            // found non-zero and swap 
            let mut idx = l;
            let mut cur = l;
            while idx > 0  {
                idx -= 1;
                if new_row[idx] > 0 {
                    new_row.swap(cur, idx);
                    cur = idx;
                } else {
                    break;
                }
            }
        }
        l += 1;
    }
    
    new_row 
}


impl Board {
    pub fn new(val: Vec<u16>) -> Self {
        Board { 
            values: val,
            rng: rand::thread_rng(),
        }
    }
    fn get_color(&self, val: u16) -> &str {
        match val {
            1..=4 => "\x1b[31;1m",
            8..=32 => "\x1b[33;1m",
            64..=128 => "\x1b[35;1m",
            256..=256 => "\x1b[32;1m",
            512..=1024 => "\x1b[36;1m",
            2048 => "\x1b[38;1m",
            _ => "\x1b[40;1m",
        }
    }

    fn get_color_reset(&self) -> &str {
        "\x1b[0m"
    }

    fn rows(&self) -> [Vec<u16>; 4] {
        let mut row1: Vec<u16> = vec![0;4];
        let mut row2: Vec<u16> = vec![0;4];
        let mut row3: Vec<u16> = vec![0;4];
        let mut row4: Vec<u16> = vec![0;4];
        row1.copy_from_slice(&self.values[0..4]);
        row2.copy_from_slice(&self.values[4..8]);
        row3.copy_from_slice(&self.values[8..12]);
        row4.copy_from_slice(&self.values[12..16]);
        [row1, row2, row3, row4]
    }


    pub fn get_values(&self) -> &Vec<u16> {
        &self.values
    }

    fn move_horizontal(&mut self, dir: Direction) {
        let mut start = 0;
        let mut end = 4;
        for row in self.rows().iter() {
             match dir {
               Direction::Left => {self.values.splice(start..end, move_row_left(row));}, 
               Direction::Right => {self.values.splice(start..end, move_row_right(row));}, 
               _ => panic!("Not support direction")
             }
             start += 4; 
             end += 4;
        }
    }

    fn transpose(&mut self) {
        let mut scratch = vec![0; 4];
        transpose::transpose_inplace(&mut self.values, &mut scratch, 4, 4);
    }

    fn move_vertical(&mut self, dir: Direction) {
        // TODO: we use two extra transpose actions that allow all directions use the same logics. But it's not optimal process
        self.transpose();
        match dir {
            Direction::Up => self.move_horizontal(Direction::Left),
            Direction::Down=> self.move_horizontal(Direction::Right),
            _ => panic!("Not Supported"),
        }
        self.transpose();
    }

    pub fn move_up(&mut self) {
        self.move_vertical(Direction::Up);
    }

    pub fn move_down(&mut self) {
        self.move_vertical(Direction::Down);
    }

    pub fn move_left(&mut self) {
        self.move_horizontal(Direction::Left);
    }

    pub fn move_right(&mut self) {
        self.move_horizontal(Direction::Right);
    }

    fn set_tile_value(&mut self, index: u8, value: u16) {
        self.values[index as usize] = value;
    }

    pub fn generate_new_tile(&mut self) -> bool {
        // get all zero tile 
        let mut zero_list = Vec::new();
        for (idx, val) in self.values.iter().enumerate() {
            if *val == 0 {
                zero_list.push(idx);
            }
        }

        if zero_list.len() > 0 {
            // choose zero cell 
            let rand_idx: usize = self.rng.gen::<usize>() % zero_list.len();
            // choose value 2 or 4
            let r: f32  = self.rng.gen();
            let rand_val = if r < 0.2 {4} else {2};
            self.set_tile_value(zero_list[rand_idx] as u8, rand_val);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        for val in self.values.iter_mut() {
            *val = 0;
        }
    }

    /// board drawing
    pub fn draw(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut texts = String::new();
        let newline  = "\n\r";
        let padding = "   ";

        // start  
        texts.push_str( &format!("{p}╔═══════╦═══════╦═══════╦═══════╗{n}",
                p = padding, n = newline));
        // content
        for (i, value) in self.values.iter().enumerate() {
             let val_str = (*value).to_string();
             match i {
                  i if i % 4 == 0 => {
                        texts.push_str( &format!("{p}║{c}{v:^7}{r}", 
                                p = padding, 
                                c = self.get_color(*value), 
                                v = if *value > 0 { &val_str } else {" "}, 
                                r = self.get_color_reset()))
                  }, 
                  i if i % 4 == 3 => { 
                    texts.push_str( &format!("║{c}{v:^7}{r}║{n}", 
                            c = self.get_color(*value), 
                            v = if *value > 0 { &val_str } else {" "}, 
                            r = self.get_color_reset(),
                            n = newline));
                    if i != 15 {
                      texts.push_str( &format!("{p}╠═══════╬═══════╬═══════╬═══════╣{n}", 
                              p = padding, n = newline));
                    }
                  },
                  _ =>  {
                    texts.push_str( &format!("║{c}{v:^7}{r}", 
                            c = self.get_color(*value), 
                            v = if *value > 0 { &val_str } else {" "}, 
                            r = self.get_color_reset()));
                  }
             }
        }
        // end 
        texts.push_str( &format!("{p}╚═══════╩═══════╩═══════╩═══════╝{n}",
                p = padding, n = newline));

        write!(f, "{}", texts)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_draw() {
    /* expectd screen
    ╔═══════╦═══════╦═══════╦═══════╗
    ║       ║   2   ║   4   ║   8   ║
    ╠═══════╬═══════╬═══════╬═══════╣
    ║  16   ║  32   ║  64   ║  128  ║
    ╠═══════╬═══════╬═══════╬═══════╣
    ║  256  ║  512  ║ 1024  ║ 2048  ║
    ╠═══════╬═══════╬═══════╬═══════╣
    ║ 4096  ║ 8192  ║ 16384 ║ 32768 ║
    ╚═══════╩═══════╩═══════╩═══════╝
    */
        let mut values = Vec::new();
        for i in 0..16 { 
           values.push(if i != 0 {1 << i} else {0});
        }
        let board = Board::new(values);
        println!("{}", board);
    }

    #[test]
    fn test_board_ini() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                0, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let board = Board::new(values.clone());
        assert_eq!(board.get_values().to_vec(), values);
    }

    #[test]
    fn test_board_move_right() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                0, 8, 2, 32,
                0, 2, 4, 32,
            ];
        let mut board = Board::new(values);
        // println!("{}", board);
        board.move_right();
        assert_eq!(board.get_values().to_vec(), expected_values);
        // let new_board = Board::new(board.get_values());
        // println!("{}", new_board);
    }

    #[test]
    fn test_board_move_left() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                2, 512, 16, 0,
                2, 8, 16, 0,
                8, 2, 32, 0,
                2, 4, 32, 0
            ];
        let mut board = Board::new(values);
       // println!("{}", board);
        board.move_left();
        assert_eq!(board.get_values().to_vec(), expected_values);
        // let new_board = Board::new(board.get_values().to_vec());
        //println!("{}", new_board);
    }

    #[test]
    fn test_board_move_up() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                4, 4, 512, 32,
                2, 4, 8, 64,
                0, 0, 2, 0,
                0, 0, 4, 0,
            ];
        let mut board = Board::new(values);
        // println!("{}", board);
        board.move_up();
        assert_eq!(board.get_values().to_vec(), expected_values);
        // let new_board = Board::new(board.get_values().to_vec());
        // println!("{}", new_board);
    }

    #[test]
    fn test_board_move_down() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                0, 0, 512, 0,
                0, 0, 8, 0,
                4, 4, 2, 32,
                2, 4, 4, 64,
            ];
        let mut board = Board::new(values);
        // println!("{}", board);
        board.move_down();
        assert_eq!(board.get_values().to_vec(), expected_values);
        // let new_board = Board::new(board.get_values().to_vec());
        // println!("{}", new_board);
    }

    #[test]
    fn test_board_transpose() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                0, 0, 4, 2,
                2, 2, 4, 0,
                512, 8, 2, 4,
                16, 16, 32,32 
            ];
        let mut board = Board::new(values);
        // println!("{}", board);
        board.transpose();
        assert_eq!(board.get_values().to_vec(), expected_values);
        // let new_board = Board::new(board.get_values().to_vec());
        // println!("{}", new_board);
    }

    #[test]
    fn test_board_set_tile_value() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let expected_values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 512,
                2, 0, 4, 1024,
            ];
        let mut board = Board::new(values);
        board.set_tile_value(11, 512);
        board.set_tile_value(15, 1024);
        assert_eq!(board.get_values().to_vec(), expected_values);
    }

    #[test]
    fn test_board_generate_new_tile() {
        let values = vec![
                0, 2, 512, 16,
                0, 2, 8, 16,
                4, 4, 2, 32,
                2, 0, 4, 32,
            ];
        let mut board = Board::new(values);
        let r = board.generate_new_tile();
        assert_eq!(r, true);

        let values = vec![
                2, 2, 512, 16,
                2, 2, 8, 16,
                4, 4, 2, 32,
                2, 4, 4, 32,
            ];
        let mut board = Board::new(values);
        let r = board.generate_new_tile();
        assert_eq!(r, false);
    }

    #[test]
    fn test_board_default() {
        let board: Board  = Default::default();
        let expected_values = vec![
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
            ];
        assert_eq!(board.get_values().to_vec(), expected_values);
    }
}

#[test]
fn test_row_move_left() {
    let test_case = vec![0,2,2,4];
    let expected = vec![4,4,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,0,2,4];
    let expected = vec![4,4,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![16,0,4,4];
    let expected = vec![16,8,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![0,2,0,2];
    let expected = vec![4,0,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,2,2,2];
    let expected = vec![4,4,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,4,2,0];
    let expected = vec![2,4,2,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![16,0,2,0];
    let expected = vec![16,2,0,0];
    let result = move_row_left(&test_case);
    assert_eq!(result, expected);
}

#[test]
fn test_row_move_right() {
    let test_case = vec![0,2,2,4];
    let expected = vec![0,0,4,4];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,0,2,4];
    let expected = vec![0,0,4,4];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![16,0,4,4];
    let expected = vec![0,0,16,8];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![0,2,0,2];
    let expected = vec![0,0,0,4];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,2,2,2];
    let expected = vec![0,0,4,4];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![2,4,2,0];
    let expected = vec![0,2,4,2];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);

    let test_case = vec![16,0,2,0];
    let expected = vec![0,0,16,2];
    let result = move_row_right(&test_case);
    assert_eq!(result, expected);
}

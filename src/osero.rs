use serde_json::json;
use serde_json::Value;

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
  Black,
  White,
  None,
}

pub struct Fields {
  fields: Vec<State>,
}

impl Fields {
  pub fn new() -> Self {
    let mut field = vec![State::None; 64];
    field[3 * 8 + 3] = State::Black;
    field[4 * 8 + 4] = State::Black;
    field[4 * 8 + 3] = State::White;
    field[3 * 8 + 4] = State::White;
    Self { fields: field }
  }

  pub fn calc(&mut self, v: Value) -> String {
    // Convert to a string
    let index: u64 = v["data"]["index"].as_u64().unwrap();
    if index > 64 {
      // Do Nothing
      return "".to_owned();
    }
    self.fields[index as usize] = State::Black;
    // どこにおくかを返す
    let putable = self.putable(State::White);
    if putable.len() == 0 {
      return "".to_owned();
    }
    self.fields[putable[0] as usize] = State::White;
    
    let res = json!({
        "type": "put white",
        "index": putable[0],
    });
    res.to_string()
  }
  fn putable(&self, state: State) -> Vec<usize> {
    let mut result = vec![];
    for row in 0..8 {
      for column in 0..8 {
        if self.fields[row * 8 + column] == State::None {
          // check row
          let row_c = check_row(row, column, state, &self.fields);
          let col_c = check_col(row, column, state, &self.fields);
          let diag_c = check_diag(row, column, state, &self.fields);
          if row_c || col_c || diag_c {
            result.push(row * 8 + column)
          }
        }
      }
    }
    result
  }

  fn bfs(&mut self) {}
}

fn check_row(row: usize, col: usize, me: State, v: &Vec<State>) -> bool {
  let vs = if me == State::Black {
    State::White
  } else {
    State::Black
  };

  if col == 0 {
    // left side
    let mut col_right = col + 1;
    let mut flag = false;
    if v[row * 8 + col_right] == vs {
      while col_right < 8 {
        col_right += 1;
        if v[row * 8 + col_right] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  } else if col == 7 {
    // right side
    let mut col_left = col - 1;
    let mut flag = false;
    if v[row * 8 + col_left] == vs {
      col_left = col;
      while col_left >= 1 {
        col_left -= 1;
        if v[row * 8 + col_left] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  } else {
    // left side
    let mut col_right = col + 1;
    let mut flag = false;
    if v[row * 8 + col_right] == vs {
      while col_right < 7 {
        col_right += 1;
        if v[row * 8 + col_right] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }

    // right side
    let mut col_left = col - 1;
    let mut flag = false;
    if v[row * 8 + col_left] == vs {
      while col_left >= 1 {
        col_left -= 1;
        if v[row * 8 + col_left] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  }
  false
}
fn check_col(row: usize, col: usize, me: State, v: &Vec<State>) -> bool {
  let vs = if me == State::Black {
    State::White
  } else {
    State::Black
  };

  if row == 0 {
    // upper side
    let mut row_right = row + 1;
    let mut flag = false;
    if v[row_right * 8 + col] == vs {
      while row_right < 7 {
        row_right += 1;
        if v[row_right * 8 + col] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  } else if row == 7 {
    // lower side
    let mut row_left = row - 1;
    let mut flag = false;
    if v[row_left * 8 + col] == vs {
      row_left = row;
      while row_left >= 1 {
        row_left -= 1;
        if v[row_left * 8 + col] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  } else {
    // upper side
    let mut row_right = row + 1;
    let mut flag = false;
    if v[row_right * 8 + col] == vs {
      while row_right < 7 {
        row_right += 1;
        if v[row_right * 8 + col] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
    // lower side
    let mut row_left = row - 1;
    let mut flag = false;
    if v[row_left * 8 + col] == vs {
      row_left = row;
      while row_left >= 1 {
        row_left -= 1;
        if v[row_left * 8 + col] == me {
          flag = true;
        }
      }
      if flag {
        return true;
      }
    }
  }
  false
}
fn check_diag(row: usize, col: usize, me: State, v: &Vec<State>) -> bool {
  let vs = if me == State::Black {
    State::White
  } else {
    State::Black
  };
  // 左上
  let mut iter1 = 1;
  let mut flag = false;
  if row >= iter1 && col >= iter1 && v[(row - iter1) * 8 + col - iter1] == vs {
    while row >= iter1 && col >= iter1 {
      if v[(row - iter1) * 8 + col - iter1] == me {
        flag = true;
      } else if v[(row - iter1) * 8 + col - iter1] == State::None {
        break;
      }
      iter1 += 1;
    }
    if flag {
      return true;
    }
  }

  // 右上
  let mut iter1 = 1;
  let mut flag = false;
  if row >= iter1 && col + iter1 < 7 && v[(row - iter1) * 8 + col + iter1] == vs {
    while row >= iter1 && col + iter1 < 8 {
      if v[(row - iter1) * 8 + col + iter1] == me {
        flag = true;
      } else if v[(row - iter1) * 8 + col + iter1] == State::None {
        break;
      }
      iter1 += 1;
    }
    if flag {
      return true;
    }
  }
  // 右下
  let mut iter1 = 1;
  if col + iter1 < 8 && row + iter1 < 8 && v[(row + iter1) * 8 + col + iter1] == vs {
    let mut flag = false;
    while col + iter1 < 8 && row + iter1 < 8 {
      if v[(row + iter1) * 8 + col + iter1] == me {
        flag = true;
      } else if v[(row + iter1) * 8 + col + iter1] == State::None {
        break;
      }
      iter1 += 1;
    }
    if flag {
      return true;
    }
  }
  // 左下
  let mut iter1 = 1;
  let mut flag = false;
  if col >= iter1 && row + iter1 < 8 && v[(row + iter1) * 8 + col - iter1] == vs {
    while col >= iter1 && row + iter1 < 8 {
      if v[(row + iter1) * 8 + col - iter1] == me {
        flag = true;
      } else if v[(row + iter1) * 8 + col - iter1] == State::None {
        break;
      }
      iter1 += 1;
    }
    if flag {
      return true;
    }
  }
  false
}

#[test]
fn init_test() {
  let board = Fields::new();
  let res = board.putable(State::White);
  println!("{:?}", res);
  assert_eq!(res, vec![19, 26, 37, 44])
}

#[test]
fn test_row() {
  let board = Fields::new();
  assert_eq!(check_row(2, 3, State::White, &board.fields), false);
  assert_eq!(check_row(2, 5, State::White, &board.fields), false);

  assert_eq!(check_row(3, 2, State::White, &board.fields), true);
  assert_eq!(check_row(3, 3, State::White, &board.fields), false);
  assert_eq!(check_row(3, 4, State::White, &board.fields), false);
  assert_eq!(check_row(3, 5, State::White, &board.fields), false);
  assert_eq!(check_row(4, 2, State::White, &board.fields), false);
  assert_eq!(check_row(4, 3, State::White, &board.fields), false);
  assert_eq!(check_row(4, 4, State::White, &board.fields), false);
  assert_eq!(check_row(4, 5, State::White, &board.fields), true);
}

#[test]
fn test_col() {
  let mut board = Fields::new();
  // board.fields[8] = State::Black;
  // board.fields[16] = State::Black;
  // board.fields[24] = State::White;
  // assert_eq!(check_col(0, 0, State::White, &board.fields), true);

  assert_eq!(check_col(3, 2, State::White, &board.fields), false);
  assert_eq!(check_col(5, 2, State::White, &board.fields), false);

  assert_eq!(check_col(2, 3, State::White, &board.fields), true);
  assert_eq!(check_col(3, 3, State::White, &board.fields), false);
  assert_eq!(check_col(4, 3, State::White, &board.fields), false);
  assert_eq!(check_col(5, 3, State::White, &board.fields), false);
  assert_eq!(check_col(2, 4, State::White, &board.fields), false);
  assert_eq!(check_col(3, 4, State::White, &board.fields), false);
  assert_eq!(check_col(4, 4, State::White, &board.fields), false);
  assert_eq!(check_col(5, 4, State::White, &board.fields), true);
}

#[test]
fn test_diag() {
  let board = Fields::new();
  assert_eq!(check_diag(2, 3, State::White, &board.fields), false);
  assert_eq!(check_diag(2, 5, State::White, &board.fields), false);

  assert_eq!(check_diag(3, 2, State::White, &board.fields), false);
  assert_eq!(check_diag(3, 3, State::White, &board.fields), false);
  assert_eq!(check_diag(3, 4, State::White, &board.fields), false);
  assert_eq!(check_diag(3, 5, State::White, &board.fields), false);
  assert_eq!(check_diag(4, 2, State::White, &board.fields), false);
  assert_eq!(check_diag(4, 3, State::White, &board.fields), false);
  assert_eq!(check_diag(4, 4, State::White, &board.fields), false);
  assert_eq!(check_diag(4, 5, State::White, &board.fields), false);
}

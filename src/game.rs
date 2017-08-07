type Field<T> = Vec<Vec<T>>;

pub struct Game {
    field: Field<bool>,
}

impl Game {
    pub fn new(buf: &[bool], row_size: usize, col_size: usize) -> Game {
        let field = Game::create(buf, row_size, col_size);
        Game { field }
    }

    pub fn next(self) -> Vec<bool> {
        self.field
            .iter()
            .enumerate()
            .map(|(y, r)| self.next_row(r, y))
            .flat_map(|x| x)
            .collect()
    }


    fn next_row(&self, row: &Vec<bool>, y: usize) -> Vec<bool> {
        row.iter()
            .enumerate()
            .map(|(x, _)| self.next_cell(y as i32, x as i32))
            .collect()
    }

    fn next_cell(&self, y: i32, x: i32) -> bool {
        let alive_num = [
            (y - 1, x - 1),
            (y, x - 1),
            (y + 1, x - 1),
            (y - 1, x),
            (y + 1, x),
            (y - 1, x + 1),
            (y, x + 1),
            (y + 1, x + 1),
        ].iter()
            .map(|&(y, x)| self.get_cell_state(y, x))
            .filter(|cell| *cell)
            .collect::<Vec<_>>()
            .len();
        match alive_num {
            3 => true,
            2 if self.is_alive(y as usize, x as usize) => true,
            _ => false,
        }
    }

    fn is_alive(&self, y: usize, x: usize) -> bool {
        self.field[y][x]
    }

    fn create(buf: &[bool], row_size: usize, col_size: usize) -> Field<bool> {
        (0..row_size)
            .into_iter()
            .map(|i| {
                let start = i * col_size;
                let end = start + col_size;
                buf[start..end].to_vec()
            })
            .collect()
    }

    fn get_cell_state(&self, row: i32, column: i32) -> bool {
        match self.field.iter().nth(row as usize) {
            Some(r) => {
                match r.iter().nth(column as usize) {
                    Some(c) => *c,
                    None => false,
                }
            }
            None => false,
        }
    }
}


#[test]
fn should_die_without_neighbors() {
    let buf = [false, false, false, false, true, false, false, false, false];
    let vec: Vec<bool> = Game::new(&buf, 3, 3).next();
    let expected = [
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    assert!(vec == expected);
}

#[test]
fn should_create_new_life_with_three_neighbors() {
    let buf = [true, true, false, true, false, false, false, false, false];
    let vec: Vec<bool> = Game::new(&buf, 3, 3).next();
    let expected = [true, true, false, true, true, false, false, false, false];
    assert!(vec == expected);
}

#[test]
fn should_survive_with_three_neighbors() {
    let buf = [
        false,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
        false,
        true,
        true,
        false,
        false,
        false,
        false,
        false,
    ];
    let vec: Vec<bool> = Game::new(&buf, 4, 4).next();
    let expected = [
        false,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
        false,
        true,
        true,
        false,
        false,
        false,
        false,
        false,
    ];
    assert!(vec == expected);
}

#[test]
fn should_die_with_one_neighbor() {
    let buf = [false, false, false, false, true, true, false, false, false];
    let vec: Vec<bool> = Game::new(&buf, 3, 3).next();
    let expected = [
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    assert!(vec == expected);
}


#[test]
fn should_die_with_five_neighbors() {
    let buf = [true, true, true, true, true, false, false, false, false];
    let vec: Vec<bool> = Game::new(&buf, 3, 3).next();
    let expected = [true, false, true, true, false, true, false, false, false];
    assert!(vec == expected);
}
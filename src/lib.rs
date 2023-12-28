use ndarray::{self, s, Array};

#[derive(Debug)]
pub struct Cell {
    row_id: usize,
    column_id: usize,
    value: u8,        // 1~9
    grid: u32,        // 0b1_1111_1111
    log: Vec<String>, // log
}
impl Cell {
    pub fn print(&self) {
        println!("--------------------------");
        println!(
            "|{}:{}| => |{}{}{}{}{}{}{}{}{}| -- |{}|",
            self.row_id,
            self.column_id,
            match self.check_bit(0) {
                true => "1",
                false => "_",
            },
            match self.check_bit(1) {
                true => "2",
                false => "_",
            },
            match self.check_bit(2) {
                true => "3",
                false => "_",
            },
            match self.check_bit(3) {
                true => "4",
                false => "_",
            },
            match self.check_bit(4) {
                true => "5",
                false => "_",
            },
            match self.check_bit(5) {
                true => "6",
                false => "_",
            },
            match self.check_bit(6) {
                true => "7",
                false => "_",
            },
            match self.check_bit(7) {
                true => "8",
                false => "_",
            },
            match self.check_bit(8) {
                true => "9",
                false => "_",
            },
            self.value
        );
        println!("--------------------------");
        for l in &self.log {
            println!("{}", l);
        }
        println!("--------------------------");
    }
    pub fn check_bit(&self, bit: usize) -> bool {
        (self.grid & (1 << bit)) != 0
    }
    pub fn clear_bit(&mut self, bit: usize) {
        if self.check_bit(bit) {
            self.grid = self.grid & !(1 << bit);
            self.log.push(format!("值 {} 被移除.", bit + 1));
        }
    }
    pub fn count(&self) -> usize {
        let mut c = 0;
        for i in 0..9 {
            if self.check_bit(i) {
                c += 1;
            }
        }
        c
    }
    pub fn check(&mut self) -> (usize, usize, u8) {
        if self.count() == 1 && self.value == 0 {
            for i in 0..9u8 {
                if self.check_bit(i.into()) {
                    self.log.push(format!(
                        "{}:{} => {}  -- 仅剩一个可能值",
                        self.row_id,
                        self.column_id,
                        i + 1
                    ));
                    return (self.row_id, self.column_id, i + 1);
                }
            }
        }
        (0, 0, 0u8)
    }
    pub fn get_values(&self) -> Vec<usize> {
        let mut vs = Vec::new();
        for i in 0..9 {
            if self.check_bit(i) {
                vs.push(i + 1);
            }
        }
        vs
    }
}
#[derive(Debug)]
pub struct Sudoku {
    data: ndarray::Array2<Cell>,
}
impl Sudoku {
    pub fn new() -> Sudoku {
        let mut v = Vec::new();
        for i in 1..=81 {
            v.push(Cell {
                row_id: (i - 1) / 9 + 1,
                column_id: (i - 1) % 9 + 1,
                value: 0,
                grid: 0b1_1111_1111,
                log: vec![String::from("Initial...")],
            });
        }
        let data = Array::from_shape_vec((9, 9), v).unwrap();
        //let data = Array::random((9, 9), Uniform::new(1, 10)); //Array::zeros((9, 9));
        Sudoku { data }
    }
    pub fn clean(&mut self) {
        for c in &mut self.data {
            c.value = 0;
            c.grid = 0b1_1111_1111;
            c.log = vec![String::from("Initial...")];
        }
    }
    pub fn init(&mut self, str: &str) {
        let mut index = 0;
        for i in str.chars() {
            match i {
                //'0' => self.set_value((index / 9 + 1, index % 9 + 1), 0),
                '1' => self.set_value((index / 9 + 1, index % 9 + 1), 1),
                '2' => self.set_value((index / 9 + 1, index % 9 + 1), 2),
                '3' => self.set_value((index / 9 + 1, index % 9 + 1), 3),
                '4' => self.set_value((index / 9 + 1, index % 9 + 1), 4),
                '5' => self.set_value((index / 9 + 1, index % 9 + 1), 5),
                '6' => self.set_value((index / 9 + 1, index % 9 + 1), 6),
                '7' => self.set_value((index / 9 + 1, index % 9 + 1), 7),
                '8' => self.set_value((index / 9 + 1, index % 9 + 1), 8),
                '9' => self.set_value((index / 9 + 1, index % 9 + 1), 9),
                _ => (),
            }
            index = index + 1;
        }
    }
    pub fn print(&self) {
        println!(" -------------------");
        for i in 1..=9 {
            let row = self.get_row(i);
            let mut display = String::new();
            for i in 0..9 {
                display += row[i].value.to_string().as_str();
                display += " ";
            }
            println!("| {}|", display);
        }
        println!(" -------------------");
    }
    pub fn print_cell(&self, r: usize, c: usize) {
        self.data[[r - 1, c - 1]].print();
    }
    pub fn get_value_count(&self, value: u8) -> usize {
        let mut count: usize = 0;
        for i in self.data.iter() {
            if i.value == value {
                count += 1;
            }
        }
        count
    }
    pub fn get_row(&self, row: usize) -> Vec<&Cell> {
        let result: Vec<&Cell> = self.data.row(row - 1).into_iter().collect();
        result
    }
    pub fn get_column(&self, column: usize) -> Vec<&Cell> {
        let result: Vec<&Cell> = self.data.column(column - 1).into_iter().collect();
        result
    }
    pub fn get_block(&self, block: usize) -> Vec<&Cell> {
        let sliceblock = match block {
            1 => self.data.slice(s![0..3, 0..3]),
            2 => self.data.slice(s![0..3, 3..6]),
            3 => self.data.slice(s![0..3, 6..9]),
            4 => self.data.slice(s![3..6, 0..3]),
            5 => self.data.slice(s![3..6, 3..6]),
            6 => self.data.slice(s![3..6, 6..9]),
            7 => self.data.slice(s![6..9, 0..3]),
            8 => self.data.slice(s![6..9, 3..6]),
            9 => self.data.slice(s![6..9, 6..9]),
            _ => self.data.slice(s![0..3, 0..3]),
        };
        let result: Vec<&Cell> = sliceblock.into_iter().collect();
        result
    }
    pub fn set_value(&mut self, pos: (usize, usize), value: u8) {
        self.data[[pos.0 - 1, pos.1 - 1]].value = value;
        self.data[[pos.0 - 1, pos.1 - 1]]
            .log
            .push(format!("值 {} 被确定!", value));
        self.data[[pos.0 - 1, pos.1 - 1]].grid = match value {
            1..=9 => ndarray_rand::rand_distr::num_traits::Pow::pow(2u32, value - 1),
            _ => self.data[[pos.0 - 1, pos.1 - 1]].grid,
        };
        let mut ps: Vec<(usize, usize)> = Vec::new();
        for c in self.get_3vec(pos) {
            if c.value == 0 {
                ps.push((c.row_id, c.column_id));
            }
        }
        for p in ps {
            self.data[[p.0 - 1, p.1 - 1]].clear_bit((value - 1).into());
        }
    }
    pub fn clear_value(&mut self, pos: (usize, usize), value: u8) {
        let _ = self.data[[pos.0 - 1, pos.1 - 1]].grid | !(1 << value);
    }
    pub fn get_3vec(&self, pos: (usize, usize)) -> Vec<&Cell> {
        let v1 = self.get_row(pos.0);
        let v2 = self.get_column(pos.1);
        let v3 = self.get_block(((pos.0 - 1) / 3) * 3 + (pos.1 - 1) / 3 + 1);
        let mut v: Vec<&Cell> = Vec::new();
        v.extend(v1.iter());
        v.extend(v2.iter());
        v.extend(v3.iter());
        v
    }
    pub fn get_empty(&self) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        for i in 1..10 {
            for j in 1..10 {
                if self.data[[i - 1, j - 1]].value == 0 {
                    result.push((i, j, self.get_block_index((i, j))));
                }
            }
        }
        result
    }
    pub fn is_unique_element(&self, v: &Vec<(usize, usize, usize)>, x: usize) -> bool {
        let mut count = 0;
        for &i in v.iter() {
            if i.2 == x {
                count += 1;
            }
        }
        return count == 1;
    }
    pub fn get_block_index(&self, pos: (usize, usize)) -> usize {
        match pos.0 {
            1 | 2 | 3 => match pos.1 {
                1 | 2 | 3 => 1,
                4 | 5 | 6 => 2,
                7 | 8 | 9 => 3,
                _ => 0,
            },
            4 | 5 | 6 => match pos.1 {
                1 | 2 | 3 => 4,
                4 | 5 | 6 => 5,
                7 | 8 | 9 => 6,
                _ => 0,
            },
            7 | 8 | 9 => match pos.1 {
                1 | 2 | 3 => 7,
                4 | 5 | 6 => 8,
                7 | 8 | 9 => 9,
                _ => 0,
            },
            _ => 0,
        }
    }
    pub fn remaining_count(&self) -> (usize, usize) {
        let mut result = 0;
        let mut totalvalues = 0;
        for i in 0..9 {
            for j in 0..9 {
                totalvalues += self.data[[i, j]].count();
                if self.data[[i, j]].value == 0 {
                    result += 1;
                }
            }
        }
        (result, totalvalues)
    }
    pub fn check_all_cells(&mut self) {
        let mut ps = Vec::new();
        for c in &mut self.data {
            match c.check() {
                (0, 0, 0) => (),
                (r, c, v) => ps.push((r, c, v)),
            }
        }
        for p in ps {
            self.set_value((p.0, p.1), p.2);
        }
    }
    pub fn check_row(&mut self) {
        let mut result = Vec::new();
        for rowid in 1..=9 {
            let row = self.get_row(rowid);
            for i in 0..9u8 {
                let mut count = 0;
                let mut pos = (0, 0);
                for c in &row {
                    if c.value == 0 {
                        if c.check_bit(i.into()) {
                            pos = (c.row_id, c.column_id);
                            count += 1;
                        }
                    }
                }
                if count == 1 {
                    result.push((pos, i));
                }
            }
        }
        for r in result {
            self.data[[r.0 .0 - 1, r.0 .1 - 1]].log.push(format!(
                "行{}中值{}仅存在于此格",
                r.0 .0,
                r.1 + 1,
            ));
            self.set_value(r.0, r.1 + 1);
        }
    }
    pub fn check_column(&mut self) {
        let mut result = Vec::new();
        for rowid in 1..=9 {
            let row = self.get_column(rowid);
            for i in 0..9u8 {
                let mut count = 0;
                let mut pos = (0, 0);
                for c in &row {
                    if c.value == 0 {
                        if c.check_bit(i.into()) {
                            pos = (c.row_id, c.column_id);
                            count += 1;
                        }
                    }
                }
                if count == 1 {
                    result.push((pos, i));
                }
            }
        }
        for r in result {
            self.data[[r.0 .0 - 1, r.0 .1 - 1]].log.push(format!(
                "列{}中值{}仅存在于此格",
                r.0 .1,
                r.1 + 1
            ));
            self.set_value(r.0, r.1 + 1);
        }
    }
    pub fn check_block(&mut self) {
        let mut result = Vec::new();
        for rowid in 1..=9 {
            let row = self.get_block(rowid);
            for i in 0..9u8 {
                let mut count = 0;
                let mut pos = (0, 0);
                for c in &row {
                    if c.value == 0 {
                        if c.check_bit(i.into()) {
                            pos = (c.row_id, c.column_id);
                            count += 1;
                        }
                    }
                }
                if count == 1 {
                    result.push((pos, i));
                }
            }
        }
        for r in result {
            self.data[[r.0 .0 - 1, r.0 .1 - 1]].log.push(format!(
                "{}:{}块中值{}仅存在于此格",
                r.0 .0,
                r.0 .1,
                r.1 + 1
            ));
            self.set_value(r.0, r.1 + 1);
        }
    }
    pub fn check_2and2(&mut self) {
        for i in 1..=9 {
            let row = self.get_row(i);
            let mut vs: Vec<usize> = Vec::new();
            for index in 0..9 {
                if row[index].count() == 2 {
                    for _index in index..9 {
                        if _index == index {
                            continue;
                        }
                        if row[index].grid == row[_index].grid {
                            //println!("{}:{} => R2x2 <= {}:{}", i, index + 1, i, _index + 1);
                            vs = row[index].get_values();
                        }
                    }
                }
            }
            if vs.len() > 0 {
                for v in &vs {
                    for j in 0..9 {
                        if self.data[[i - 1, j]].get_values() != vs {
                            self.data[[i - 1, j]].clear_bit(*v - 1);
                            //self.data[[i - 1, j]].print();
                        }
                    }
                }
            }
        }
        for i in 1..=9 {
            let row = self.get_column(i);
            let mut vs: Vec<usize> = Vec::new();
            for index in 0..9 {
                if row[index].count() == 2 {
                    for _index in index..9 {
                        if _index == index {
                            continue;
                        }
                        if row[index].grid == row[_index].grid {
                            //println!("{}:{} => C2x2 <= {}:{}", i, index + 1, i, _index + 1);
                            vs = row[index].get_values();
                        }
                    }
                }
            }
            if vs.len() > 0 {
                for v in &vs {
                    for j in 0..9 {
                        if self.data[[j, i - 1]].get_values() != vs {
                            self.data[[j, i - 1]].clear_bit(*v - 1);
                        }
                    }
                }
            }
        }
        for i in 1..=9 {
            let row = self.get_block(i);
            let mut vs: Vec<usize> = Vec::new();
            for index in 0..9 {
                if row[index].count() == 2 {
                    for _index in index..9 {
                        if _index == index {
                            continue;
                        }
                        if row[index].grid == row[_index].grid {
                            //println!("{}:{} => B2x2 <= {}:{}", i, index + 1, i, _index + 1);
                            vs = row[index].get_values();
                        }
                    }
                }
            }
            if vs.len() > 0 {
                let mut pos: Vec<(usize, usize, usize)> = Vec::new();
                for v in &vs {
                    for j in 0..9 {
                        if row[j].get_values() != vs {
                            pos.push((row[j].row_id, row[j].column_id, v - 1));
                        }
                    }
                }
                for p in pos {
                    self.data[[p.0 - 1, p.1 - 1]].clear_bit(p.2);
                }
            }
        }
    }
    pub fn check_row_column_in_block(&mut self) {
        let mut result = Vec::new();
        for block_id in 1..=9 {
            let block = self.get_block(block_id);
            for i in 1..=9 {
                let mut i_count = 0;
                for c in &block {
                    if c.check_bit(i - 1) {
                        i_count += 1;
                    }
                }
                if i_count == 1 {
                    continue;
                }
                // check row
                let mut i_row = 0;
                let mut row_id = 0;
                for c in &block[1..3] {
                    row_id = c.row_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, row_id, 0));
                }
                i_row = 0;
                for c in &block[4..6] {
                    row_id = c.row_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, row_id, 0));
                }
                i_row = 0;
                for c in &block[7..9] {
                    row_id = c.row_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, row_id, 0));
                }
                // check column
                let mut i_row = 0;
                let mut row_id = 0;
                for c in vec![block[0], block[3], block[6]] {
                    row_id = c.column_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, 0, row_id));
                }
                i_row = 0;
                for c in vec![block[1], block[4], block[7]] {
                    row_id = c.column_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, 0, row_id));
                }
                i_row = 0;
                for c in vec![block[2], block[5], block[8]] {
                    row_id = c.column_id;
                    if c.check_bit(i - 1) {
                        i_row += 1;
                    }
                }
                if i_count == i_row {
                    result.push((i, block_id, 0, row_id));
                }
            }
        }
        //println!("result : {:?}", result);
        for res in result {
            if res.2 == 0 {
                let mut row_vec = Vec::new();
                match res.1 {
                    1 | 2 | 3 => row_vec = vec![4, 5, 6, 7, 8, 9],
                    4 | 5 | 6 => row_vec = vec![1, 2, 3, 7, 8, 9],
                    7 | 8 | 9 => row_vec = vec![1, 2, 3, 4, 5, 6],
                    _ => (),
                }
                //println!("{:?}  : {}", row_vec, self.remaining_count().1) ;
                for r in row_vec {
                    //println!("{}:{} >> {}", r , res.3 , res.0);
                    self.data[[r - 1, res.3 - 1]].clear_bit(res.0 - 1);
                }
            } else {
                let mut column_vec = Vec::new();
                match res.1 {
                    1 | 4 | 7 => column_vec = vec![4, 5, 6, 7, 8, 9],
                    2 | 5 | 8 => column_vec = vec![1, 2, 3, 7, 8, 9],
                    3 | 6 | 9 => column_vec = vec![1, 2, 3, 4, 5, 6],
                    _ => (),
                }
                //println!("{:?}   : {}", column_vec, self.remaining_count().1) ;
                for c in column_vec {
                    //println!("{}:{} >> {}", res.2 , c , res.0);
                    self.data[[res.2 - 1, c - 1]].clear_bit(res.0 - 1);
                }
            }
        }
    }
    pub fn calculate(&mut self) {
        loop {
            let count = self.remaining_count();
            self.check_all_cells();
            self.check_row();
            self.check_column();
            self.check_block();
            self.check_2and2();
            self.check_row_column_in_block();
            if count == self.remaining_count() {
                break;
            }
            //self.print();
        }
    }
    pub fn exam(&self) -> bool {
        let remain = self.remaining_count();
        if remain.0 == 0 {
            if remain.1 == 81 {
                println!("**** DONE! {:?}", remain);
                return true;
            } else {
                println!("**** 题目有错误! ",);
            }
        } else {
            println!("**** Remain {:?} ", remain);
        }
        return false;
    }
    pub fn to_string(&self) -> String {
        let mut data = String::new();
        for c in &self.data {
            data.push_str(&c.value.to_string());
        }
        data
    }
    pub fn try_guess(&mut self) {
        let mut ps = Vec::new();
        for c in &self.data {
            match c.count() {
                2 => ps.push(((c.row_id, c.column_id), c.get_values())),
                3 => ps.push(((c.row_id, c.column_id), c.get_values())),
                _ => (),
            };
        }
        let store = self.to_string();
        for p in ps {
            for i in p.1 {
                self.set_value(p.0, i as u8);
                self.calculate();
                if !self.exam() {
                    self.clean();
                    self.init(&store);
                    self.calculate();
                } else {
                    return;
                }
            }
        }
    }
}

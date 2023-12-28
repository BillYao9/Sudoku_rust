use std::io::Write;
use std::time::Instant;
use std::{fs, fs::File, io};
use sudoku::Sudoku;

fn main() {
    loop {
        println!("请输入命令: (new | quit)");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("错误的命令!");
        match cmd.trim() {
            "quit" => {
                break;
            }
            "new" => {
                new_game();
            }
            _ => {
                println!("不支持的命令,仅支持new和quit");
            }
        }
    }
}

fn new_game() {
    println!("*********************************************************************************");
    let mut file = match fs::OpenOptions::new().append(true).open("log.ini") {
        Ok(f) => f,
        Err(_) => File::create("log.ini").expect("无法创建log文件"),
    };
    let mut sudoku: Sudoku = Sudoku::new();
    let mut str = String::new();
    while str.trim().len() != 81 {
        println!("请输入81个数字,未知数字请输入0,可以三位一组输入,参考下行的位置编号:");
        println!(
            "123456789123456789123456789123456789123456789123456789123456789123456789123456789"
        );
        str = String::new();
        match io::stdin().read_line(&mut str) {
            Ok(_) => (),
            Err(_) => println!("输入错误！"),
        };
    }
    sudoku.init(&str);
    sudoku.print();
    let remain = sudoku.remaining_count();
    println!("**** Remain {:?}", remain);
    println!("按回车开始计算...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let start = Instant::now();
    sudoku.calculate();
    sudoku.print();
    if !sudoku.exam() {
        println!("开始尝试猜测...");
        sudoku.try_guess();
        sudoku.print();
    }
    file.write_all(str.as_bytes()).expect("写入log文件失败！");
    file.write_all(sudoku.to_string().as_bytes()).expect("写入log文件失败！");
    let _ = file.flush();
    let end = Instant::now();
    let duration = end.duration_since(start).as_secs_f32();
    println!("程序执行时间：{:.4} 秒", duration);
    loop {
        println!("请输入行列号来显示对应块的状态:");
        let mut pos = String::new();
        io::stdin().read_line(&mut pos).unwrap();
        let pos: usize = match pos.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        match pos {
            11..=99 => sudoku.print_cell((pos - pos % 10) / 10, pos % 10),
            100 => {
                for i in 1..10 {
                    for j in 1..10 {
                        sudoku.print_cell(i, j);
                    }
                }
            }
            _ => break,
        };
    }
    println!("*********************************************************************************");
}

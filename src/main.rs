use std::io::Write;
use std::time::Instant;
use std::{fs, fs::File, io};
use sudoku::Sudoku;

fn main() {
    println!("*********************************************************************************");
    println!("******************************** Rust Sudoku 解题 *******************************");
    println!("*********************************************************************************");
    let mut file = match fs::OpenOptions::new().append(true).open("log.ini") {
        Ok(f) => f,
        Err(_) => File::create("log.ini").expect("无法创建log文件"),
    };
    'toploop: loop {
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
        let _ = file.flush();
        let end = Instant::now();
        let duration = end.duration_since(start).as_secs_f32();
        println!("程序执行时间：{:.4} 秒", duration);
        loop {
            println!("请输入操作命令:(guess|reset|new|save|quit)");
            let mut cmd = String::new();

            io::stdin().read_line(&mut cmd).expect("错误的命令!");
            match cmd.trim() {
                "guess" => (),
                "reset" => {
                    sudoku.clean();
                    sudoku.init(&str);
                    sudoku.calculate();
                    sudoku.print();
                    if !sudoku.exam() {
                        println!("开始尝试猜测...");
                        sudoku.try_guess();
                        sudoku.print();
                    }
                    continue;
                }
                "new" => break,
                "quit" => {
                    break 'toploop;
                }
                "save" => {
                    file.write_all(sudoku.to_string().as_bytes())
                        .expect("写入log文件失败！");
                    let _ = file.flush();
                }
                _ => continue,
            }
            loop {
                println!("请输入行列号来查看或者猜测对应格子\n;例如 : 34=>3行4列的格子 | 345=>将3行4列设置为5\n输入0=>退出猜测");
                let mut pos = String::new();
                io::stdin().read_line(&mut pos).unwrap();
                let pos: usize = match pos.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("请输入数字.");
                        continue;
                    }
                };
                match pos {
                    0 => break,
                    10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => continue,
                    11..=99 => sudoku.print_cell((pos - pos % 10) / 10, pos % 10),
                    100 => {
                        for i in 1..10 {
                            for j in 1..10 {
                                sudoku.print_cell(i, j);
                            }
                        }
                    }
                    111..=999 => {
                        if pos % 10 != 0 {
                            let r = (pos - pos % 100) / 100;
                            let c = (pos % 100 - pos % 10) / 10;
                            let v = pos % 10;
                            sudoku.set_value((r, c), v as u8);
                            sudoku.calculate();
                            sudoku.print();
                            if !sudoku.exam() {
                                println!("开始尝试猜测...");
                                sudoku.try_guess();
                                sudoku.print();
                            }
                            if sudoku.is_error() {
                                sudoku.clean();
                                sudoku.init(&str);
                                sudoku.calculate();
                                sudoku.print();
                            }
                        }
                    }
                    _ => continue,
                };
            }
        }
    }
    println!("*********************************************************************************");
}

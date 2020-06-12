use std::env;

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: sample <rows> <columns> [1_fraction_0_to_100 = 75]");
        return;
    }
    let rows = args[1].parse::<usize>().expect("First argument should be integer");
    let columns = args[2].parse::<usize>().expect("Second argument should be integer");
    let fraction = if args.len() >= 4 {
        args[3].parse::<usize>().expect("Third argument should be integer")
    } else { 75 };
    if rows < 2 || rows > 20_000 { panic!("Wrong rows argument") }
    if columns < 2 || columns > 20_000 { panic!("Wrong columns argument") }
    if fraction > 100 { panic!("Wrong fraction argument") }

    println!("{},{}", columns, rows);
    let mut buf = String::with_capacity(columns);
    let mut rng = rand::thread_rng();
    for i in 0..rows {
        buf.clear();
        for j in 0..columns {
            if i == 1 && j == 0 {
                buf.push('1');
            }
            else if i == rows - 2 && j == columns -1 {
                buf.push('1');
            } else {
                let num = rng.gen_range(0, 101);
                buf.push(if num < fraction {'1'} else {'0'});
            }
        }
        println!("{}", buf);
    }
}
use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("failed to read from stdin");
    let n: i64 = n_str.trim().parse().expect("n_str parse() error");

    let weird: bool = match n {
        n if n % 2 == 0 => true,
        2...5 => false,
        6...20 => true,
        _ => false,
    };

    if !weird {
        print!("Not ");
    }
    println!("Weird");
}

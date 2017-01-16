use std::io;

fn main() {
    let (i, d, s) = (4, 4.0, "HackerRank ");
    let mut ii_str = String::new();
    io::stdin().read_line(&mut ii_str).expect("failed to read from stdin");
    let ii: i64 = ii_str.trim().parse().expect("i64 parse() error");
    let mut dd_str = String::new();
    io::stdin().read_line(&mut dd_str).expect("failed to read from stdin");
    let dd: f64 = dd_str.trim().parse().expect("f64 parse() error");
    let mut ss = String::new();
    io::stdin().read_line(&mut ss).expect("failed to read from stdin");

    println!("{}\n{}\n{}{}", i + ii, d + dd, s, ss);
}

use std::io;

fn main() {
    let (i, d, s) = (4, 4.0, "HackerRank ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let splitted: Vec<&str> = input.split_whitespace().collect();

    println!("{}\n{}\n{}{}",
             i + splitted[0].parse::<i32>().unwrap(),
             d + splitted[1].parse::<f64>().unwrap(),
             s,
             splitted[2]);
}

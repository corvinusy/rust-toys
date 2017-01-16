use std::io;

fn main() {
    let mut meal_cost_str = String::new();
    io::stdin().read_line(&mut meal_cost_str).expect("failed to read from stdin");
    let meal_cost: f64 = meal_cost_str.trim().parse().expect("meal_cost parse() error");
    let mut tip_percent_str = String::new();
    io::stdin().read_line(&mut tip_percent_str).expect("failed to read from stdin");
    let tip_percent: i64 = tip_percent_str.trim().parse().expect("tip_percent parse() error");
    let mut tip_percent_str = String::new();
    io::stdin().read_line(&mut tip_percent_str).expect("failed to read from stdin");
    let tax_percent: i64 = tip_percent_str.trim().parse().expect("tax_percent parse() error");

    let tip = meal_cost * (tip_percent as f64) / 100.0;
    let tax = meal_cost * (tax_percent as f64) / 100.0;
    let total_cost = meal_cost + tip + tax;

    println!("The total meal cost is {} dollars.", total_cost.round());
}

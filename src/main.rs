mod days;

fn main() {
    println!("Hello, world!");
    days::day_one::run_day("inputs/day_1.txt");
    days::day_two::run_day("inputs/day_2.txt");
    days::day_three::run_day("inputs/day_3.txt");
    days::day_four::run_day("inputs/day_4.txt"); 
}

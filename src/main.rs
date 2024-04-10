mod days;

fn main() {
    println!("Hello, world! AoC 2020!");
    days::verbosity_set(false);
    days::day_one::run_day("inputs/day_1.txt");
    days::day_two::run_day("inputs/day_2.txt");
    days::day_three::run_day("inputs/day_3.txt");
    days::day_four::run_day("inputs/day_4.txt");
    days::day_five::run_day("inputs/day_5.txt");
    days::day_six::run_day("inputs/day_6.txt");
    days::day_seven::run_day("inputs/day_7.txt");
    days::day_eight::run_day("inputs/day_8.txt");
    days::day_nine::run_day("inputs/day_9.txt");
    days::day_ten::run_day("inputs/day_10.txt");
    days::day_eleven::run_day("inputs/day_11.txt");
}

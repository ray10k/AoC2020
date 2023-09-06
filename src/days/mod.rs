pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;
pub mod day_five;
pub mod day_six;
pub mod day_seven;

pub fn verbosity_set(verbose:bool) {
    day_five::verbosity_set(verbose);
    day_six::verbosity_set(verbose);
}
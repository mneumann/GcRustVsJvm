mod randy_random;
mod standard_random;
mod small_random;
mod fast_rand;

fn main() {
    standard_random::benchmark();
    randy_random::benchmark();
    small_random::benchmark();
    fast_rand::benchmark();
}

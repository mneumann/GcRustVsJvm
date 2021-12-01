mod fast_rand;
mod randy_random;
mod small_random;
mod standard_random;

fn main() {
    standard_random::benchmark();
    randy_random::benchmark();
    small_random::benchmark();
    fast_rand::benchmark();
}

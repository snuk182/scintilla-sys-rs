
use std::env;

pub fn main() {
    env::set_current_dir("qt").expect("Could not change dir");
}

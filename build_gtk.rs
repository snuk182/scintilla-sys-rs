
use std::env;

pub fn main() {
    env::set_current_dir("gtk").expect("Could not change dir");
}

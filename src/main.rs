mod generator;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    generator::new_dungeon(9, 9, 2);

    //dungeon.print();
}

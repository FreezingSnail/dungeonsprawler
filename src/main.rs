mod generator;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    generator::new_dungeon(300, 8, 300);

    //dungeon.print();
}

mod generator;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dungeon = generator::new_dungeon(40, 40);
    //dungeon.print();
}

mod generator;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dungeon = generator::new_dungeon(80, 80);
    dungeon.print();
}

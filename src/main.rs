mod exporter;
mod generator;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a JSON file as a command-line argument");
    }

    let json_file_path = &args[1];
    if !Path::new(json_file_path).exists() {
        panic!("The file {} does not exist", json_file_path);
    }

    let mut file = File::open(json_file_path).expect("Unable to open the file");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)
        .expect("Unable to read the file");

    let params = generator::paramaters::marshal_from_json(&json_content).unwrap();
    let outpath = params.file_path.clone();

    let d = generator::new_dungeon(&params);

    for (i, dungeon) in d.iter().enumerate() {
        let name = params.dungeons[i].name.clone();

        if params.output_format == "raycast" {
            // let name = params.dungeons[i].name.clone() + "_raycast";
            // exporter::write_dungeons_to_file(&dungeon, &name, &outpath, true).unwrap();
            let name = params.dungeons[i].name.clone();
            exporter::write_dungeons_to_lua(&dungeon, &name, &outpath).unwrap();
        } else {
            exporter::write_dungeons_to_file(&dungeon, &name, &outpath, false).unwrap();
        }
    }
}

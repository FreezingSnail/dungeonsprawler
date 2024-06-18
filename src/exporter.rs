use serde_json;
use std::fs::File;
use std::io::prelude::*;

use crate::generator::Dungeon;

pub fn write_dungeons_to_file(
    data: &Vec<Dungeon>,
    name: String,
    filename: &str,
) -> std::io::Result<()> {
    let mut dungeon_values: Vec<String> = Vec::new();

    for d in data {
        let mut dungeon = String::new();
        for row in &d.grid {
            let row_str = row
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            dungeon.push_str(&row_str);
            dungeon.push_str("\n");
        }
        dungeon_values.push(dungeon);
    }

    let filename = format!("{}{}_maps.txt", filename, name);
    if let Some(parent_dir) = std::path::Path::new(&filename).parent() {
        std::fs::create_dir_all(parent_dir)?;
    }
    let mut file = File::create(filename)?;

    for (i, d) in dungeon_values.iter().enumerate() {
        let header = format!("uint8_t {}{} = {{", name, i);
        let footer = "};";
        let data = format!("{}\n{}{}\n", header, d, footer);
        file.write_all(data.as_bytes())?;
    }

    let mut floor_names: Vec<String> = Vec::new();
    for i in 0..data.len() {
        floor_names.push(format!("{}{} ", name, i));
    }

    let floors = floor_names.join(",");
    let floor_pointer_string = format!("\nuint24_t {}_floors[] = {{ {} }};\n", name, floors);
    file.write_all(floor_pointer_string.as_bytes())?;

    Ok(())
}

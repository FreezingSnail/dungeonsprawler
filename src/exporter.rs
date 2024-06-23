use serde_json;
use std::fs::File;
use std::io::prelude::*;

use crate::generator::Dungeon;

pub fn write_dungeons_to_file(
    data: &Vec<Dungeon>,
    name: &String,
    filename: &str,
    raycast: bool,
) -> std::io::Result<()> {
    let mut dungeon_values: Vec<String> = Vec::new();

    for d in data {
        let mut dungeon = String::new();

        let grid = if raycast { &d.raycast_grid } else { &d.grid };
        let w = d.width.to_string();
        let h = d.height.to_string();
        let dims = format!("{}, {},\n", h, w);
        dungeon.push_str(&dims);
        let start = format!("{},{},\n", d.start_y, d.start_x);
        dungeon.push_str(&start);
        for (i, row) in grid.iter().enumerate() {
            let row_str = row
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");

            dungeon.push_str(&row_str);
            if i < grid.len() - 1 {
                dungeon.push_str(",");
            }
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

pub fn write_dungeons_to_lua(
    data: &Vec<Dungeon>,
    name: &String,
    filename: &str,
) -> std::io::Result<()> {
    let mut dungeon_values: Vec<String> = Vec::new();

    for d in data {
        let dungeon_data = reverse_grid_rows(&d.grid.clone());
        let dungeon_string = lua_dungeon_data(
            dungeon_data.clone(),
            d.width,
            d.height,
            d.start_x,
            d.start_y,
        );
        dungeon_values.push(dungeon_string);
    }
    let lua_name = format!("{}", name);
    let data_string = format!("{} = {{\n{}\n}}", lua_name, dungeon_values.join(",\n"));
    let lua_code = lua_text(lua_name);

    let mut dungeon_raycast_values: Vec<String> = Vec::new();

    for d in data {
        let raycast_string = lua_dungeon_data(
            d.raycast_grid.clone(),
            d.width,
            d.height,
            d.start_x,
            d.start_y,
        );
        dungeon_raycast_values.push(raycast_string);
    }
    let lua_name = format!("{}_raycast", name);
    let data_raycast_string = format!(
        "{} = {{\n{}\n}}",
        lua_name,
        dungeon_raycast_values.join(",\n")
    );
    let lua_raycast_code = lua_text(lua_name.clone());
    let addrs = lua_addr_pairs(name);

    let text = format!(
        "{}\n{}\n{}\n{}\n{}",
        data_string, lua_code, data_raycast_string, lua_raycast_code, addrs
    );

    let filename = format!("{}{}_maps.lua", filename, name);
    if let Some(parent_dir) = std::path::Path::new(&filename).parent() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;

    Ok(())
}

fn lua_text(lua_name: String) -> String {
    let lua_raycast_code = format!(
        "
{}_pointers = {{}}
totaloffset = 0
field(\"{}\")
for i,d in ipairs({}) do
    {}_pointers[i] = address()
   write(bytes(d))
end
field(\"{}_pointers\")
write(bytes({}_pointers, \"uint24\"))
",
        lua_name, lua_name, lua_name, lua_name, lua_name, lua_name
    );
    lua_raycast_code
}

fn lua_addr_pairs(name: &String) -> String {
    let lua_raycast_code = format!(
        "
field(\"{}_pairs\")
for i,d in ipairs(test_dungeon_raycast) do
    write(bytes({{{}_pointers[i],{}_raycast_pointers[i]}}, \"uint32\"))
end
    ",
        name, name, name
    );
    lua_raycast_code
}

fn lua_dungeon_data(
    grid: Vec<Vec<i32>>,
    width: u32,
    height: u32,
    start_x: u32,
    start_y: u32,
) -> String {
    let mut dungeon = String::new();
    dungeon.push_str("{\n");
    let w = width.to_string();
    let h = height.to_string();
    let dims = format!("{},{},\n", h, w);
    dungeon.push_str(&dims);
    let start = format!("{},{},\n", width - start_x - 1, start_y);
    dungeon.push_str(&start);
    for (i, row) in grid.iter().enumerate() {
        let row_str = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        dungeon.push_str(&row_str);
        if i < grid.len() - 1 {
            dungeon.push_str(",\n");
        }
    }
    dungeon.push_str("}");
    dungeon
}

fn reverse_grid_rows(map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_grid: Vec<Vec<i32>> = Vec::new();
    for row in map.iter() {
        let mut new_row: Vec<i32> = Vec::new();
        for cell in row.iter().rev() {
            new_row.push(*cell);
        }
        new_grid.push(new_row.clone());
    }
    new_grid
}

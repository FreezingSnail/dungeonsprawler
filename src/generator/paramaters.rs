use serde::{Deserialize, Serialize};

/* configurable options:
--------------------------------
# Dungeon options
- height
- width
- room types
- amount of rooms
- room size range
- start / finish min distance
- room padding / density
- sprawl or sparse

# Seralization options
- file path
- output format (fx, raycast)
- include images ( based on output format)
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DungeonOptions {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub room_types: Vec<String>,
    pub amount_of_rooms: u32,
    pub room_size_low: u32,
    pub room_size_high: u32,
    pub start_finish_min_distance: u32,
    pub room_padding_density: u32,
    pub sparse: bool,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DungeonParameters {
    // Dungeon options
    pub dungeons: Vec<DungeonOptions>,

    // Serialization options
    pub file_path: String,
    pub output_format: String,
    pub include_images: bool,
}

pub fn marshal_from_json(json: &str) -> Result<DungeonParameters, serde_json::Error> {
    serde_json::from_str(json)
}

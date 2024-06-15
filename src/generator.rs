use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::E,
};

mod painter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoomType {
    Empty,
    Wall,
    Hall,
    Start,
    End,
    Boss,
    Shop,
    Treasure,
    Secret,
    LockedDoor,
}

impl RoomType {
    fn to_int(&self) -> u32 {
        match self {
            RoomType::Empty => 0,
            RoomType::Wall => 1,
            RoomType::Hall => 2,
            RoomType::Start => 3,
            RoomType::End => 4,
            RoomType::Boss => 5,
            RoomType::Shop => 6,
            RoomType::Treasure => 7,
            RoomType::Secret => 8,
            RoomType::LockedDoor => 9,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Room {
    height: u32,
    width: u32,
    x: u32,
    y: u32,
    room_type: RoomType,
}

pub struct Dungeon {
    rooms: Vec<Room>,
    placed_rooms: Vec<Room>,
    height: u32,
    width: u32,
    grid: Vec<Vec<i32>>,
    regions_count: i32,
    regions: Vec<Vec<u32>>,
    painter: painter::Painter,
}

impl Dungeon {
    fn new(height: u32, width: u32) -> Dungeon {
        let rooms = Vec::new();
        let grid = vec![vec![0; width as usize]; height as usize];

        let regions = vec![vec![0; width as usize]; height as usize];

        Dungeon {
            rooms,
            placed_rooms: Vec::new(),
            height,
            width,
            grid,
            regions_count: 0,
            regions,
            painter: painter::Painter::new(),
        }
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn place_start_and_end(&mut self) {
        let mut rng = rand::thread_rng();
        let max_distance = (self.height.max(self.width) / 2) as i32;
        let mut valid_placement = false;
        let attempts = 20;
        let mut trys = 0;
        while !valid_placement && trys < attempts {
            trys += 1;
            let start_x = rng.gen_range(1..self.width - 1);
            let start_y = rng.gen_range(1..self.height - 1);
            let end_x = rng.gen_range(1..self.width - 1);
            let end_y = rng.gen_range(1..self.height - 1);
            let x: i32 = (start_x as i32 - end_x as i32) as i32;
            let y = (start_y as i32 - end_y as i32) as i32;
            let distance = (x.abs() + y.abs()) as i32;
            if distance >= max_distance {
                self.grid[start_y as usize][start_x as usize] = RoomType::Start.to_int() as i32;
                self.grid[end_y as usize][end_x as usize] = RoomType::End.to_int() as i32;
                self.regions_count += 1;

                self.regions[start_y as usize][start_x as usize] = self.regions_count as u32;
                self.regions_count += 1;
                self.regions[end_y as usize][end_x as usize] = self.regions_count as u32;

                let start_room = &Room {
                    height: 1,
                    width: 1,
                    x: end_x,
                    y: end_y,
                    room_type: RoomType::Start,
                };
                // self.wrap_room(start_room, start_y, start_x);
                let end_room = &Room {
                    height: 1,
                    width: 1,
                    x: end_x,
                    y: end_y,
                    room_type: RoomType::End,
                };
                // self.wrap_room(end_room, end_y, end_x);

                let placed_new = Room {
                    height: 1,
                    width: 1,
                    x: start_x,
                    y: start_y,
                    room_type: RoomType::Start,
                };

                let placed_end = Room {
                    height: 1,
                    width: 1,
                    x: end_x,
                    y: end_y,
                    room_type: RoomType::End,
                };
                self.placed_rooms.push(placed_new.clone());
                self.placed_rooms.push(placed_end.clone());

                valid_placement = true;
                self.painter.add_step(self.grid.clone());
            }
        }
    }

    fn generate(&mut self, buffer: u32) {
        let mut rng = rand::thread_rng();

        for room in &self.rooms {
            let room_type = room.room_type.to_int();
            let mut valid_placement = false;
            let attempts = 20;
            let mut trys = 0;

            let gen_width = self.width - room.width - buffer;
            let gen_height = self.height - room.height - buffer;
            while !valid_placement && trys < attempts {
                trys += 1;
                let x = rng.gen_range(buffer..gen_width);
                let y = rng.gen_range(buffer..gen_height);

                let mut overlap = false;
                for i in 0..room.height + (buffer * 2) {
                    for j in 0..room.width + (buffer * 2) {
                        if self.grid[(y - buffer + i) as usize][(x - buffer + j) as usize] != 0 {
                            overlap = true;
                            break;
                        }
                    }
                    if overlap {
                        break;
                    }
                }

                if !overlap {
                    self.regions_count += 1;
                    // println!("Placing room at ({}, {}), {}", x, y, room_type as i32);
                    // Set the interior of the room to the specified room type
                    for i in 0..room.height {
                        for j in 0..room.width {
                            self.grid[(y + i) as usize][(x + j) as usize] = room_type as i32;
                            self.regions[(y + i) as usize][(x + j) as usize] =
                                self.regions_count as u32;
                        }
                    }

                    // Wrap the outside of the room with walls
                    for i in 0..=room.height - 1 {
                        self.grid[(y + i) as usize][(x - 1) as usize] =
                            RoomType::Wall.to_int() as i32;
                        self.grid[(y + i) as usize][(x + room.width) as usize] =
                            RoomType::Wall.to_int() as i32;
                    }
                    for j in 0..=room.width - 1 {
                        self.grid[(y - 1) as usize][(x + j) as usize] =
                            RoomType::Wall.to_int() as i32;
                        self.grid[(y + room.height) as usize][(x + j) as usize] =
                            RoomType::Wall.to_int() as i32;
                    }

                    //                    println!("{}", self.grid[(y) as usize][(x) as usize]);
                    valid_placement = true;
                    let mut placed_room = room.clone();
                    placed_room.x = x - 1;
                    placed_room.y = y - 1;
                    placed_room.width += 2;
                    placed_room.height += 2;
                    self.placed_rooms.push(placed_room);

                    self.painter.add_step(self.grid.clone());
                }
            }
        }
    }

    fn wrap_room(&mut self, room: &Room, y: u32, x: u32) {
        for i in 0..=room.height - 1 {
            self.grid[(y + i) as usize][(x - 1) as usize] = RoomType::Wall.to_int() as i32;
            self.grid[(y + i) as usize][(x + room.height) as usize] =
                RoomType::Wall.to_int() as i32;
        }
        for j in 0..=room.width - 1 {
            self.grid[(y - 1) as usize][(x + j) as usize] = RoomType::Wall.to_int() as i32;

            self.grid[(y + room.width) as usize][(x + j) as usize] = RoomType::Wall.to_int() as i32;
        }
        println!();
    }

    fn is_valid_hall(&self, y: u32, x: u32) -> bool {
        let mut valid_sides = 0;
        if y >= self.height || x >= self.width {
            return false;
        }
        if self.grid[y as usize][x as usize] != RoomType::Empty.to_int() as i32 {
            // println!(
            //     "Invalid hall at {}, {}, room type is {}",
            //     x, y, self.grid[y as usize][x as usize]
            // );
            return false;
        }
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (x2, y2) in dirs {
            let new_x = x as i32 + x2;
            let new_y = y as i32 + y2;

            if (new_x >= 0 && new_x < self.width as i32)
                && (new_y >= 0 && new_y < self.height as i32)
            {
                if self.grid[new_y as usize][new_x as usize] == RoomType::Empty.to_int() as i32
                    || self.grid[new_y as usize][new_x as usize] == RoomType::Wall.to_int() as i32
                {
                    valid_sides += 1;
                }
            }
        }

        valid_sides >= 3
    }

    fn make_halls(&mut self, start: (u32, u32)) {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<(u32, u32)> = Vec::new();
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        {
            let mut valid_halls: Vec<(u32, u32)> = Vec::new();
            for dir in &dirs {
                let new_x = start.0 as i32 + dir.0;
                let new_y = start.1 as i32 + dir.1;
                if new_x >= 0
                    && new_x < self.width as i32
                    && new_y >= 0
                    && new_y < self.height as i32
                {
                    if self.is_valid_hall(new_y as u32, new_x as u32) {
                        valid_halls.push((new_x as u32, new_y as u32));
                    }
                }
            }

            if valid_halls.len() < 4 {
                return;
            }
        }

        self.regions_count += 1;
        cells.push(start);
        self.make_hall(start);

        let mut last_dr: (i32, i32) = (0, 0);

        while cells.len() > 0 {
            let mut valid_halls: Vec<(i32, i32)> = Vec::new();
            let cell = cells.last().unwrap();

            for dir in &dirs {
                let new_x = cell.0 as i32 + dir.0;
                let new_y = cell.1 as i32 + dir.1;
                if new_x >= 0
                    && new_x < self.width as i32
                    && new_y >= 0
                    && new_y < self.height as i32
                {
                    if self.is_valid_hall(new_y as u32, new_x as u32) {
                        valid_halls.push((dir.0, dir.1));
                    }
                }
            }
            if valid_halls.len() > 0 {
                let next_dr: (i32, i32);

                if valid_halls.contains(&last_dr) && rng.gen_ratio(2, 4) {
                    next_dr = last_dr;
                } else {
                    let random_spot = rng.gen_range(0..valid_halls.len());
                    next_dr = valid_halls[random_spot];
                }
                let mut next_cell = (
                    (cell.0 as i32 + (next_dr.0)) as u32,
                    (cell.1 as i32 + (next_dr.1)) as u32,
                );

                self.make_hall(next_cell);

                let next_next_cell = (
                    (cell.0 as i32 + (next_dr.0 + next_dr.0)) as u32,
                    (cell.1 as i32 + (next_dr.1 + next_dr.1)) as u32,
                );
                if self.is_valid_hall(next_next_cell.1, next_next_cell.0) {
                    self.make_hall(next_next_cell);
                    next_cell = next_next_cell;
                }
                last_dr = next_dr;
                cells.push(next_cell);
                self.painter.add_step(self.grid.clone());
            } else {
                cells.pop();
            }
        }
    }

    fn make_hall(&mut self, position: (u32, u32)) {
        let (x, y) = position;
        if y < self.height && x < self.width {
            self.grid[y as usize][x as usize] = RoomType::Hall.to_int() as i32;
            self.regions[y as usize][x as usize] = self.regions_count as u32;
            self.painter.add_step(self.grid.clone());
        }
    }

    fn fill_with_walls(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == RoomType::Empty.to_int() as i32 {
                    *cell = RoomType::Wall.to_int() as i32;
                }
            }
        }
    }

    fn connect_regions(&mut self) {
        let mut connector_regions: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for x in 0..self.height {
            for y in 0..self.width {
                // Can't already be part of a region.
                if self.regions[y as usize][x as usize] != RoomType::Empty.to_int() as u32 {
                    continue;
                }

                let mut regions: HashSet<u32> = HashSet::new();

                for (x2, y2) in &dirs {
                    let target_y = y as i32 + y2;
                    let target_x = x as i32 + x2;
                    if !(target_y < 0
                        || target_y >= self.height as i32
                        || target_x < 0
                        || target_x >= self.width as i32)
                    {
                        let region = self.regions[target_y as usize][target_x as usize];
                        if region != RoomType::Empty.to_int() as u32 {
                            regions.insert(region);
                        }
                    } else {
                    }
                }

                if regions.len() < 2 {
                    continue;
                }

                connector_regions.insert((x, y), regions.clone());
            }
        }

        let mut connectors: Vec<(u32, u32)> = connector_regions.keys().cloned().collect();

        // Keep track of which regions have been merged. This maps an original
        // region index to the one it has been merged to.
        let mut merged: HashMap<u32, u32> = HashMap::new();
        let mut open_regions: HashSet<u32> = HashSet::new();
        for i in 1..=(self.regions_count + 1) as u32 {
            merged.insert(i, i);
            open_regions.insert(i);
        }

        let mut rng = rand::thread_rng();

        //println!("Open regions: {:?}", open_regions);
        while open_regions.len() > 1 && connectors.len() > 0 {
            //self.regions_count += 1;

            let start = open_regions.len();

            // pick a random connector
            let connector_index = rng.gen_range(0..connectors.len());
            let (x, y) = connectors[connector_index];

            // join the regions on either side of the connector
            self.grid[y as usize][x as usize] = RoomType::Hall.to_int() as i32;
            self.regions[y as usize][x as usize] = (self.regions_count + 1) as u32;

            // Merge the connected regions. We'll pick one region (arbitrarily) and
            // map all of the other regions to its index.
            let regions: HashSet<_> = connector_regions
                .get(&(x, y))
                .unwrap()
                .iter()
                .map(|&region| *merged.get(&region).unwrap())
                .collect();

            //println!("Open regions: {:?}", open_regions)
            let regions_list = regions.iter().collect::<Vec<_>>();
            let dest = **regions_list.first().unwrap();
            let sources: Vec<u32> = regions.into_iter().skip(1).collect();

            // Merge all of the affected regions. We have to look at *all* of the
            // regions because other regions may have previously been merged with
            // some of the ones we're merging now.
            for i in 0..(self.regions_count + 1) as u32 {
                let mrgd = merged.get(&i);
                if mrgd.is_some() {
                    if sources.contains(mrgd.unwrap()) {
                        merged.insert(i, dest);
                    }
                }
            }

            for region in &sources {
                let removed = open_regions.remove(region);
            }

            connectors.retain(|&v| {
                // Don't allow connectors right next to each other.
                if distance((x, y), v) < 2.0 {
                    return false;
                }

                // If the connector no longer spans different regions, we don't need it.
                let regions: HashSet<_> = connector_regions
                    .get(&v)
                    .unwrap()
                    .iter()
                    .map(|&region| *merged.get(&region).unwrap())
                    .collect();

                if regions.len() > 1 {
                    return true;
                } else {
                    //    println!("keeping regions: {:?}, {}", regions, regions.len());
                }

                //println!("removing");
                // This connector isn't needed, but connect it occasionally so that the
                // dungeon isn't singly-connected.
                if rng.gen_ratio(1, 5) {
                    //  println!("----- ADDING EXTRA");
                    self.grid[v.1 as usize][v.0 as usize] = RoomType::Hall.to_int() as i32;
                    self.regions[v.1 as usize][v.0 as usize] = self.regions_count as u32;
                }

                return false;
            });

            if open_regions.len() == start {
                return;
            }
        }
    }

    fn are_start_and_end_connected(&self) -> bool {
        let start_room = &self.placed_rooms[0];
        let end_room = &self.placed_rooms[1];

        let start_x = start_room.x;
        let start_y = start_room.y;
        let end_x = end_room.x;
        let end_y = end_room.y;

        let mut visited: Vec<Vec<bool>> =
            vec![vec![false; self.width as usize]; self.height as usize];
        let mut queue: Vec<(i32, i32)> = Vec::new();

        queue.push((start_x as i32, start_y as i32));
        visited[start_y as usize][start_x as usize] = true;

        while !queue.is_empty() {
            let (x, y) = queue.remove(0);

            if x == end_x as i32 && y == end_y as i32 {
                return true;
            }

            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            for (nx, ny) in neighbors {
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if !visited[ny as usize][nx as usize]
                        && (self.grid[ny as usize][nx as usize] != RoomType::Wall.to_int() as i32
                            && self.grid[ny as usize][nx as usize]
                                != RoomType::Empty.to_int() as i32)
                    {
                        visited[ny as usize][nx as usize] = true;
                        queue.push((nx, ny));
                    }
                }
            }
        }

        false
    }

    fn add_start_and_end_halls(&mut self) {
        let start_room = &self.placed_rooms[0];
        let end_room = &self.placed_rooms[1];

        // Add hall from start room to the north

        self.grid[start_room.y as usize][(start_room.x + 1) as usize] =
            RoomType::Hall.to_int() as i32;
        self.grid[(start_room.y + 1) as usize][start_room.x as usize] =
            RoomType::Hall.to_int() as i32;
        self.grid[(start_room.y + 1) as usize][(start_room.x + 2) as usize] =
            RoomType::Hall.to_int() as i32;
        self.grid[(start_room.y + 2) as usize][(start_room.x + 1) as usize] =
            RoomType::Hall.to_int() as i32;

        self.grid[end_room.y as usize][(end_room.x + 1) as usize] = RoomType::Hall.to_int() as i32;
        self.grid[(end_room.y + 1) as usize][end_room.x as usize] = RoomType::Hall.to_int() as i32;
        self.grid[(end_room.y + 1) as usize][(end_room.x + 2) as usize] =
            RoomType::Hall.to_int() as i32;
        self.grid[(end_room.y + 2) as usize][(end_room.x + 1) as usize] =
            RoomType::Hall.to_int() as i32;
    }

    fn remove_dead_ends(&mut self) {
        let mut done = false;

        while !done {
            done = true;

            for x in 0..self.width - 1 {
                for y in 0..self.height - 1 {
                    if self.grid[y as usize][x as usize] != RoomType::Hall.to_int() as i32 {
                        continue;
                    }

                    // If it only has one exit, it's a dead end.
                    let mut exits = 0;
                    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for dir in dirs {
                        let y = (y as i32 + dir.1) as i32;
                        let x = (x as i32 + dir.0) as i32;

                        if y >= 0
                            && y < self.height as i32
                            && x >= 0
                            && x < self.width as i32
                            && self.grid[y as usize][x as usize] != RoomType::Wall.to_int() as i32
                        {
                            exits += 1;
                        }
                    }

                    if exits > 1 {
                        continue;
                    }

                    done = false;
                    self.grid[y as usize][x as usize] = RoomType::Wall.to_int() as i32;
                }
            }
        }
    }
}

fn distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> f64 {
    let dx = (x1 as i32 - x2 as i32) as f64;
    let dy = (y1 as i32 - y2 as i32) as f64;
    (dx * dx + dy * dy).sqrt()
}

pub fn connected(d: Dungeon) -> bool {
    d.are_start_and_end_connected()
}

pub fn new_dungeon(height: u32, width: u32, rooms: u32) -> Dungeon {
    let mut d = Dungeon::new(height, width);
    let mut rng = rand::thread_rng();

    d.painter.add_step(d.grid.clone());
    d.place_start_and_end();

    for _ in 0..rooms {
        let room = Room {
            height: rng.gen_range(1..=3),
            width: rng.gen_range(1..=3),
            x: 0,
            y: 0,
            room_type: match rng.gen_range(4..=7) {
                4 => RoomType::Boss,
                5 => RoomType::Shop,
                6 => RoomType::Treasure,
                7 => RoomType::Secret,
                _ => RoomType::Wall,
            },
        };
        d.add_room(room);
    }

    d.generate(2);

    for x in 0..d.height {
        for y in 0..d.width {
            if d.grid[y as usize][x as usize] == RoomType::Empty.to_int() as i32 {
                d.make_halls((x, y));
            }
        }
    }
    d.painter.paint_image(&d.grid, "dungeon_final.png");
    d.painter.paint_image_u(&d.regions, "dungeon_regions.png");

    d.connect_regions();

    let mut connected = d.are_start_and_end_connected();
    // let mut attempts = 0;
    // while !connected && attempts < 10 {
    //     for x in 0..d.height {
    //         for y in 0..d.width {
    //             if d.grid[y as usize][x as usize] == RoomType::Empty.to_int() as i32 {
    //                 d.make_halls((x, y));
    //             }
    //         }
    //     }
    //     d.connect_regions();

    //     attempts += 1;
    //     //     d.connect_rooms_to_halls();
    //     //     connected = d.are_start_and_end_connected();
    // }

    //d.fill_with_walls();
    //d.remove_dead_ends();

    println!("Connected: {}", connected);
    println!("map created");
    d.painter.paint();
    d.painter.paint_image(&d.grid, "dungeon_final.png");
    d.painter.paint_image_u(&d.regions, "dungeon_regions.png");
    //d.print();

    d
}

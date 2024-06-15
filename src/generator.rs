use rand::Rng;

mod painter;

#[derive(Debug)]
enum RoomType {
    Wall,
    Hall,
    Start,
    End,
    Boss,
    Shop,
    Treasure,
    Secret,
}

impl RoomType {
    fn to_int(&self) -> u32 {
        match self {
            RoomType::Wall => 0,
            RoomType::Hall => 1,
            RoomType::Start => 2,
            RoomType::End => 3,
            RoomType::Boss => 4,
            RoomType::Shop => 5,
            RoomType::Treasure => 6,
            RoomType::Secret => 7,
        }
    }
}

#[derive(Debug)]
struct Room {
    height: u32,
    width: u32,
    room_type: RoomType,
}

pub struct Dungeon {
    rooms: Vec<Room>,
    height: u32,
    width: u32,
    grid: Vec<Vec<i32>>,
    painter: painter::Painter,
}

impl Dungeon {
    fn new(height: u32, width: u32) -> Dungeon {
        let rooms = Vec::new();
        let mut grid = vec![vec![0; width as usize]; height as usize];
        for i in 0..height {
            for j in 0..width {
                grid[i as usize][j as usize] = -1;
            }
        }
        Dungeon {
            rooms,
            height,
            width,
            grid,
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

                self.wrap_room(
                    &Room {
                        height: 1,
                        width: 1,
                        room_type: RoomType::Start,
                    },
                    start_y,
                    start_x,
                );
                self.wrap_room(
                    &Room {
                        height: 1,
                        width: 1,
                        room_type: RoomType::End,
                    },
                    end_y,
                    end_x,
                );

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
                        if self.grid[(y - buffer + i) as usize][(x - buffer + j) as usize] != -1 {
                            overlap = true;
                            break;
                        }
                    }
                    if overlap {
                        break;
                    }
                }

                if !overlap {
                    // println!("Placing room at ({}, {}), {}", x, y, room_type as i32);
                    // Set the interior of the room to the specified room type
                    for i in 0..room.height {
                        for j in 0..room.width {
                            self.grid[(y + i) as usize][(x + j) as usize] = room_type as i32;
                        }
                    }

                    // Wrap the outside of the room with walls
                    for i in 0..=room.height {
                        self.grid[(y + i) as usize][(x - 1) as usize] =
                            RoomType::Wall.to_int() as i32;
                        self.grid[(y + i) as usize][(x + room.width) as usize] =
                            RoomType::Wall.to_int() as i32;
                    }
                    for j in 0..=room.width {
                        self.grid[(y - 1) as usize][(x + j) as usize] =
                            RoomType::Wall.to_int() as i32;
                        self.grid[(y + room.height) as usize][(x + j) as usize] =
                            RoomType::Wall.to_int() as i32;
                    }
                    self.grid[(y - 1) as usize][(x - 1) as usize] = RoomType::Wall.to_int() as i32;
                    self.grid[(y - 1) as usize][(x + room.width) as usize] =
                        RoomType::Wall.to_int() as i32;
                    self.grid[(y + room.height) as usize][(x - 1) as usize] =
                        RoomType::Wall.to_int() as i32;
                    self.grid[(y + room.height) as usize][(x + room.width) as usize] =
                        RoomType::Wall.to_int() as i32;

                    //                    println!("{}", self.grid[(y) as usize][(x) as usize]);
                    valid_placement = true;
                    self.painter.add_step(self.grid.clone());
                }
            }
        }
    }

    fn wrap_room(&mut self, room: &Room, y: u32, x: u32) {
        for i in 0..=room.height {
            self.grid[(y + i) as usize][(x - 1) as usize] = RoomType::Wall.to_int() as i32;
            self.grid[(y + i) as usize][(x + room.width) as usize] = RoomType::Wall.to_int() as i32;
        }
        for j in 0..=room.width {
            self.grid[(y - 1) as usize][(x + j) as usize] = RoomType::Wall.to_int() as i32;
            self.grid[(y + room.height) as usize][(x + j) as usize] =
                RoomType::Wall.to_int() as i32;
        }
        self.grid[(y - 1) as usize][(x - 1) as usize] = RoomType::Wall.to_int() as i32;
        self.grid[(y - 1) as usize][(x + room.width) as usize] = RoomType::Wall.to_int() as i32;
        self.grid[(y + room.height) as usize][(x - 1) as usize] = RoomType::Wall.to_int() as i32;
        self.grid[(y + room.height) as usize][(x + room.width) as usize] =
            RoomType::Wall.to_int() as i32;
    }

    pub fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i as usize][j as usize] == -1 {
                    print!("  ");
                } else {
                    print!("{} ", self.grid[i as usize][j as usize]);
                }
            }
            println!();
        }
        for room in &self.rooms {
            println!("{:?}", room);
        }
    }

    fn is_valid_hall(&mut self, y: u32, x: u32) -> bool {
        let mut valid_sides = 0;

        if self.grid[y as usize][x as usize] != 0 {
            return false;
        }

        if y > 0 && self.grid[(y - 1) as usize][x as usize] == 0 {
            valid_sides += 1;
        }
        if y < self.height - 1 && self.grid[(y + 1) as usize][x as usize] == 0 {
            valid_sides += 1;
        }
        if x > 0 && self.grid[y as usize][(x - 1) as usize] == 0 {
            valid_sides += 1;
        }
        if x < self.width - 1 && self.grid[y as usize][(x + 1) as usize] == 0 {
            valid_sides += 1;
        }
        valid_sides >= 3
    }

    fn make_halls(&mut self, start: (u32, u32)) {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<(u32, u32)> = Vec::new();
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut valid_halls: Vec<(u32, u32)> = Vec::new();
        for dir in &dirs {
            let new_x = start.0 as i32 + dir.0;
            let new_y = start.1 as i32 + dir.1;
            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                if self.is_valid_hall(new_y as u32, new_x as u32) {
                    valid_halls.push((new_x as u32, new_y as u32));
                }
            }
        }

        if valid_halls.len() < 2 {
            return;
        }

        cells.push(start);

        let random_spot = rng.gen_range(0..cells.len());
        let (x, y) = cells[random_spot];

        while cells.len() > 0 {
            let mut valid_halls: Vec<(u32, u32)> = Vec::new();
            for dir in &dirs {
                let cell = cells.last().unwrap();
                let new_x = cell.0 as i32 + dir.0;
                let new_y = cell.1 as i32 + dir.1;
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
            if valid_halls.len() > 0 {
                let random_spot = rng.gen_range(0..valid_halls.len());
                let (x, y) = valid_halls[random_spot];
                cells.push((x, y));
                self.grid[y as usize][x as usize] = RoomType::Hall.to_int() as i32;
                self.painter.add_step(self.grid.clone());
            } else {
                cells.pop();
            }
        }
    }

    fn transform_negative_to_zero(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == -1 {
                    *cell = 0;
                }
            }
        }
    }
}

pub fn new_dungeon(height: u32, width: u32) -> Dungeon {
    let mut d = Dungeon::new(height, width);
    let mut rng = rand::thread_rng();

    d.painter.add_step(d.grid.clone());
    d.place_start_and_end();

    for _ in 0..100 {
        let room = Room {
            height: rng.gen_range(2..=10),
            width: rng.gen_range(2..=8),
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

    d.generate(1);
    d.transform_negative_to_zero();

    for _ in 0..100 {
        let random_x = rng.gen_range(0..width);
        let random_y = rng.gen_range(0..height);
        d.make_halls((random_x, random_y));
    }
    println!("map created");
    //d.painter.paint();
    d.painter.paint_image(&d.grid);
    d
}

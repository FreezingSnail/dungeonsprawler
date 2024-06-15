use rand::Rng;

mod painter;

#[derive(Debug, Clone, Copy)]
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
            placed_rooms: Vec::new(),
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

                let start_room = &Room {
                    height: 1,
                    width: 1,
                    x: end_x,
                    y: end_y,
                    room_type: RoomType::Start,
                };
                self.wrap_room(start_room, start_y, start_x);
                let end_room = &Room {
                    height: 1,
                    width: 1,
                    x: end_x,
                    y: end_y,
                    room_type: RoomType::End,
                };
                self.wrap_room(end_room, end_y, end_x);

                let placed_new = Room {
                    height: 3,
                    width: 3,
                    x: start_x - 1,
                    y: start_y - 3,
                    room_type: RoomType::Start,
                };

                let placed_end = Room {
                    height: 3,
                    width: 3,
                    x: end_x - 1,
                    y: end_y - 1,
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

    fn is_valid_hall(&self, y: u32, x: u32) -> bool {
        let mut valid_sides = 0;

        if y >= self.height || x >= self.width {
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
        if self.grid[y as usize][x as usize] != 0 {
            return false;
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
                let mut next_dr: (i32, i32) = (0, 0);

                if valid_halls.contains(&last_dr) && rng.gen_ratio(3, 4) {
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
            self.painter.add_step(self.grid.clone());
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

    fn is_valid_con(&self, y: u32, x: u32) -> bool {
        let mut valid_sides = 0;

        if y >= self.height || x >= self.width {
            return false;
        }

        if y > 0 && self.grid[(y - 1) as usize][x as usize] == RoomType::Hall.to_int() as i32 {
            valid_sides += 1;
        }
        if y < self.height - 1
            && self.grid[(y + 1) as usize][x as usize] == RoomType::Hall.to_int() as i32
        {
            valid_sides += 1;
        }
        if x > 0 && self.grid[y as usize][(x - 1) as usize] == RoomType::Hall.to_int() as i32 {
            valid_sides += 1;
        }
        if x < self.width - 1
            && self.grid[y as usize][(x + 1) as usize] == RoomType::Hall.to_int() as i32
        {
            valid_sides += 1;
        }
        if self.grid[y as usize][x as usize] != 0 {
            return false;
        }

        valid_sides == 1
    }

    fn connect_rooms_to_halls(&mut self) {
        let mut rng = rand::thread_rng();
        let mut connected_rooms: Vec<usize> = Vec::new();
        let mut unconnected_rooms: Vec<usize> = (0..self.placed_rooms.len()).collect();
        let max_attempts = 1000;
        let mut attempts = 0;

        while unconnected_rooms.len() > 0 && attempts < max_attempts {
            attempts += 1;
            let room_index = rng.gen_range(0..unconnected_rooms.len());
            let room_id = unconnected_rooms[room_index];
            let room = &self.placed_rooms[room_id];

            let mut valid_wall_cells: Vec<(u32, u32)> = Vec::new();

            for i in 0..room.height {
                for j in 0..room.width {
                    let y = room.y + i;
                    let x = room.x + j;

                    if self.is_valid_con(y, x)
                        && (y != room.y || x != room.x)
                        && (y != room.y + room.height - 1 || x != room.x + room.width - 1)
                        && (y != room.y || x != room.x + room.width - 1)
                        && (y != room.y + room.height - 1 || x != room.x)
                    {
                        valid_wall_cells.push((y, x));
                    }
                }
            }

            if valid_wall_cells.len() > 0 {
                let hall_index = rng.gen_range(0..valid_wall_cells.len());
                let (hall_y, hall_x) = valid_wall_cells[hall_index];

                self.grid[hall_y as usize][hall_x as usize] = RoomType::Hall.to_int() as i32;

                if rng.gen_ratio(2, 4) {
                    connected_rooms.push(room_id);
                    unconnected_rooms.remove(room_index);
                }
            }
        }
        println!("Connected rooms: {:?}", connected_rooms.len());
    }

    fn are_start_and_end_connected(&self) -> bool {
        let start_room = &self.placed_rooms[0];
        let end_room = &self.placed_rooms[1];

        let start_x = start_room.x + 1;
        let start_y = start_room.y + 1;
        let end_x = end_room.x + 1;
        let end_y = end_room.y + 1;

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
                        && self.grid[ny as usize][nx as usize] != RoomType::Wall.to_int() as i32
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
        for x in start_room.x..start_room.x + start_room.width {
            self.grid[start_room.y as usize][x as usize] = RoomType::Hall.to_int() as i32;
        }

        // Add hall from start room to the south
        for x in start_room.x..start_room.x + start_room.width {
            self.grid[(start_room.y + start_room.height - 1) as usize][x as usize] =
                RoomType::Hall.to_int() as i32;
        }

        // Add hall from start room to the west
        for y in start_room.y..start_room.y + start_room.height {
            self.grid[y as usize][start_room.x as usize] = RoomType::Hall.to_int() as i32;
        }

        // Add hall from start room to the east
        for y in start_room.y..start_room.y + start_room.height {
            self.grid[y as usize][(start_room.x + start_room.width - 1) as usize] =
                RoomType::Hall.to_int() as i32;
        }

        // Add hall from end room to the north
        for x in end_room.x..end_room.x + end_room.width {
            self.grid[end_room.y as usize][x as usize] = RoomType::Hall.to_int() as i32;
        }

        // Add hall from end room to the south
        for x in end_room.x..end_room.x + end_room.width {
            self.grid[(end_room.y + end_room.height - 1) as usize][x as usize] =
                RoomType::Hall.to_int() as i32;
        }

        // Add hall from end room to the west
        for y in end_room.y..end_room.y + end_room.height {
            self.grid[y as usize][end_room.x as usize] = RoomType::Hall.to_int() as i32;
        }

        // Add hall from end room to the east
        for y in end_room.y..end_room.y + end_room.height {
            self.grid[y as usize][(end_room.x + end_room.width - 1) as usize] =
                RoomType::Hall.to_int() as i32;
        }
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

pub fn new_dungeon(height: u32, width: u32) -> Dungeon {
    let mut d = Dungeon::new(height, width);
    let mut rng = rand::thread_rng();

    d.painter.add_step(d.grid.clone());
    d.place_start_and_end();

    for _ in 0..40 {
        let room = Room {
            height: rng.gen_range(2..=5),
            width: rng.gen_range(2..=5),
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

    d.generate(1);
    d.transform_negative_to_zero();

    for _ in 0..100 {
        let random_x = rng.gen_range(0..width);
        let random_y = rng.gen_range(0..height);
        d.make_halls((random_x, random_y));
    }
    d.connect_rooms_to_halls();
    d.add_start_and_end_halls();
    d.remove_dead_ends();

    let connected = d.are_start_and_end_connected();
    println!("Connected: {}", connected);
    println!("map created");
    d.painter.paint();
    d.painter.paint_image(&d.grid);
    d
}

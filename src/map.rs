use rltk::{RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

pub const MAP_MAX_WIDTH: i32 = 160;
pub const MAP_MAX_HEIGHT: i32 = 80;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Checks to see if one rectangle intersects with another.
    /// Useful visualization: (https://silentmatt.com/rectangle-intersection)
    pub fn does_intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center point of the rectangle as a tuple
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    /// Converts a point of a 2D structure to a single index for a 1D structure
    pub fn map_xy_to_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn add_room_to_map(&mut self, room: &Rect) {
        // Note that ..= in rust is a way to specify an inclusive range
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.map_xy_to_index(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let index = self.map_xy_to_index(x, y);

            if index > 0 && index < (MAP_MAX_WIDTH * MAP_MAX_HEIGHT) as usize {
                self.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn add_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let index = self.map_xy_to_index(x, y);

            if index > 0 && index < (MAP_MAX_WIDTH * MAP_MAX_HEIGHT) as usize {
                self.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    pub fn generate() -> Map {
        const MAX_ROOMS: i32 = 30;
        const MIN_ROOM_SIZE: i32 = 10;
        const MAX_ROOM_SIZE: i32 = 15;

        let mut map = Map {
            tiles: vec![TileType::Wall; (MAP_MAX_WIDTH * MAP_MAX_HEIGHT) as usize],
            rooms: Vec::new(),
            width: MAP_MAX_WIDTH,
            height: MAP_MAX_HEIGHT,
        };

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
            let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
            let x = rng.roll_dice(1, MAP_MAX_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, MAP_MAX_HEIGHT - w - 1) - 1;

            let room = Rect::new(x, y, w, h);
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if room.does_intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                map.add_room_to_map(&room);

                if !map.rooms.is_empty() {
                    let (curr_x, curr_y) = room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.add_horizontal_corridor(prev_x, curr_x, prev_y);
                        map.add_vertical_corridor(prev_y, curr_y, curr_x);
                    } else {
                        map.add_vertical_corridor(prev_y, curr_y, prev_x);
                        map.add_horizontal_corridor(prev_x, curr_x, curr_y);
                    }
                }

                map.rooms.push(room);
            }
        }

        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    let wall_glyph = rltk::to_cp437('#');
    let floor_glyph = rltk::to_cp437('.');

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    floor_glyph,
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    wall_glyph,
                );
            }
        }

        x += 1;

        if x > MAP_MAX_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}

// Constructs a map with a perimeter of solid walls and 600 other walls
// placed randomly. Used for testing purposes.
// pub fn gen_map_test() -> Vec<TileType> {
//     let dimensions = (MAP_MAX_WIDTH * MAP_MAX_HEIGHT) as usize;
//     let mut map = vec![TileType::Floor; dimensions];
//     let mut rng = rltk::RandomNumberGenerator::new();

//     // Create Boundaries

//     for x in 0..MAP_MAX_WIDTH {
//         map[map_xy_to_index(x, 0)] = TileType::Wall;
//         map[map_xy_to_index(x, MAP_MAX_HEIGHT - 1)] = TileType::Wall;
//     }

//     for y in 0..MAP_MAX_HEIGHT {
//         map[map_xy_to_index(0, y)] = TileType::Wall;
//         map[map_xy_to_index(MAP_MAX_WIDTH - 1, y)] = TileType::Wall;
//     }

//     // Fill in other parts of the map at random

//     for _i in 0..600 {
//         // Note that rng.roll_dice is exclusive, which is why we still only offset by 1
//         let x = rng.roll_dice(1, MAP_MAX_WIDTH - 1);
//         let y = rng.roll_dice(1, MAP_MAX_HEIGHT - 1);
//         let index = map_xy_to_index(x, y);

//         if index != map_xy_to_index(MAP_MAX_WIDTH / 2, MAP_MAX_HEIGHT / 2) {
//             map[index] = TileType::Wall;
//         }
//     }

//     map
// }

use super::Rect;
use rltk::{RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            width: 80,
            height: 50,
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);

            let mut valid_room = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    valid_room = false
                }
            }

            if valid_room {
                map.add_room(&new_room);

                if !map.rooms.is_empty() {
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    let (new_x, new_y) = new_room.center();

                    if rng.range(0, 2) == 1 {
                        map.add_horizontal_corridor(prev_x, new_x, prev_y);
                        map.add_vertical_corridor(prev_y, new_y, new_x);
                    } else {
                        map.add_vertical_corridor(prev_y, new_y, prev_x);
                        map.add_horizontal_corridor(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn add_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);

            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn add_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);

            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 1.0, 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

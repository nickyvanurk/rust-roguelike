use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
mod map;
mod player;
mod rect;

pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::*;

mod monster_ai_system;
mod visibility_system;

use monster_ai_system::MonsterAiSystem;
use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAiSystem {};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let glyph = match rng.roll_dice(1, 2) {
            1 => rltk::to_cp437('g'),
            _ => rltk::to_cp437('o'),
        };

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Monster {})
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}

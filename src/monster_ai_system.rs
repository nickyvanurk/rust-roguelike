use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{a_star_search, console, DistanceAlg, Point};
use specs::prelude::*;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (mut map, player_pos, mut viewshed, monster, name, mut position): Self::SystemData,
    ) {
        for (mut viewshed, _, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            let distance =
                DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);

            if distance < 1.5 {
                console::log(&format!("{} shouts insults", name.name));
                continue;
            }

            if viewshed.visible_tiles.contains(&*player_pos) {
                let path = a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}

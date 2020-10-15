use super::{Map, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut map, entities, mut viewshed, pos, player): Self::SystemData) {
        for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if !viewshed.dirty {
                continue;
            }

            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x <= map.width - 1 && p.y >= 0 && p.y <= map.height - 1);

            let p: Option<&Player> = player.get(entity);

            if let Some(_) = p {
                for tile in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(tile.x, tile.y);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}

use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, (mut map, position, blockers, entities): Self::SystemData) {
        map.populate_blocked();
        map.clear_content_index();

        for (entity, pos) in (&entities, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            let p: Option<&BlocksTile> = blockers.get(entity);

            if let Some(_) = p {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(entity);
        }
    }
}

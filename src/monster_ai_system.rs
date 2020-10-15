use super::{Monster, Name, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, (player_pos, viewshed, monster, name): Self::SystemData) {
        for (viewshed, _, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}

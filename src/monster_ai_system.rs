use super::{Monster, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, (viewshed, pos, monster): Self::SystemData) {
        for (viewshed, pos, monster) in (&viewshed, &pos, &monster).join() {
            console::log("Monster considers their own existence");
        }
    }
}

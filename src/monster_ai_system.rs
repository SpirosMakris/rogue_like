use specs::prelude::*;
use super::{Viewshed, Monster, Name, Map, Position};
use rltk::{Point, console};


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  #[allow(clippy::type_complexity)]
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadExpect<'a, Point>,  // Player Position resource
    WriteStorage<'a, Viewshed>,
    ReadStorage<'a, Monster>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, Position>,
  );
  
    fn run(&mut self, data: Self::SystemData) {
      let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

      for (mut viewshed, _monster, name, mut pos) in (&mut viewshed, &monster, &name, &mut position).join() {
        // If we can see the player
        if viewshed.visible_tiles.contains(&*player_pos) {

          // Insult him a bit if we are close enough
          let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
          if distance < 1.5 {
            // Attack goes here
            console::log(format!("{} shouts insults", name.name));
            return;
          }

          // Try to find a path from our (monster) position to the player position. Works with maps indices
          let path = rltk::a_star_search(map.xy_idx(pos.x, pos.y) as i32, map.xy_idx(player_pos.x, player_pos.y) as i32, &mut *map);

          if path.success && path.steps.len() > 1 {
            // Convert 1st step of path to xy coords (0 step is current position)
            pos.x = path.steps[1] % map.width;  
            pos.y = path.steps[1] / map.width;
            // We moved os we invalidated our viewshed
            viewshed.dirty = true;
          }
        }
      }
    }
}
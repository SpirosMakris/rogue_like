use super::{Map, Monster, Name, Position, RunState, Viewshed, WantsToMelee};
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,  // Player Position resource
        ReadExpect<'a, Entity>, // Player entity resource
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            name,
            mut position,
            mut wants_to_melee,
        ) = data;

        // Only run if it's the monster's turn
        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, mut viewshed, _monster, name, mut pos) in
            (&entities, &mut viewshed, &monster, &name, &mut position).join()
        {
            // Find out distance betweenus(monster) and player
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            // Attack if player gets too close
            if distance < 1.5 {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("@ERROR: Unable to insert attack to player");
            }
            // If we can see the player
            else if viewshed.visible_tiles.contains(&*player_pos) {
                // Path to player by finding a path from our (monster) position to the player position. Works with maps indices
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    // Find current position index
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    // Unblock it since we are leaving
                    map.blocked[idx] = false;
                    // Convert 1st step of path to xy coords (0 step is current position)
                    // and move there
                    pos.x = path.steps[1] % map.width;
                    pos.y = path.steps[1] / map.width;
                    // Recalculate current index after move
                    idx = map.xy_idx(pos.x, pos.y);
                    // And block it since we are now occuping the tile
                    map.blocked[idx] = true;
                    // We moved so we invalidated our viewshed
                    viewshed.dirty = true;
                }
            }
        }
    }
}

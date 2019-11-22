use super::{gamelog::GameLog, CombatStats, Name, Player, SufferDamage};
use rltk::console;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        // Apply damage to combat stats hp
        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount;
        }

        // Now that the damage is applied, remove the msg component
        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    // Using a scope to make the borrow checker happy
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                // Is this the player?
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity);
                        if let Some(victim_name) = victim_name {
                            log.entries
                                .insert(0, format!("{} is dead", &victim_name.name));
                        }
                        // This is NOT the player, queue for deletion
                        dead.push(entity);
                    }
                    Some(_) => console::log("You dead"),
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim)
            .expect("@ERROR: Unable to delete dead entity");
    }
}

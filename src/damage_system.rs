use specs::prelude::*;
use super::{CombatStats, SufferDamage, Player};
use rltk::console;

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
    let entities = ecs.entities();

    for (entity, stats) in (&entities, &combat_stats).join() {
      if stats.hp < 1 {
        // Is this the player?
        let player = players.get(entity);
        match player {
          None => dead.push(entity),  // This is NOT the player, queue for deletion
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
use rltk::{Console, GameState, Rltk, RGB, Point, RandomNumberGenerator};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

rltk::add_wasm_support!();

// Our modules
mod components;
pub use components::*;

mod map;
pub use map::*;

mod player;
use player::*;

mod rect;
pub use rect::Rect;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod monster_ai_system;
use monster_ai_system::MonsterAI;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    ecs: World,
    pub run_state: RunState
}

impl State {
    fn run_systems(&mut self) {
        // Run Visibility System
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        // Run Monster AI System
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        
        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        // player_input(self, ctx);
        // self.run_systems();

        // The map is a resource, so get it from ecs world
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

fn main() {
    let ctx = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");

    // Create our gamestate with an ecs world in it.
    let mut gs = State { ecs: World::new(), run_state: RunState::Running };

    // Register our components with the ecs world (internally creates storage systems, etc)
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    // Insert resources into our ecs world
    let map: Map = Map::new_map_rooms_and_corridors();
    // Put the player in the center of the 1st room before moving map into the ECS world
    let (player_x, player_y) = map.rooms[0].center();

  

    // Create a player entity
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
        .with(Name { name: "Player".to_string() })
        .build();
    
    // Create monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();

        let glyph: u8;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            },
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position {x, y})
            .with(Renderable {
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(), range: 8, dirty: true
            })
            .with(Monster {})
            .with(Name { name: format!("{} #{}", &name, i) })
            .build();
    }
    
    gs.ecs.insert(map); // The map is now available from everywhere the ECS can see!
    gs.ecs.insert(Point::new(player_x, player_y)); // Add player position as an ECS resource (updated in player input)
    
    rltk::main_loop(ctx, gs);
}

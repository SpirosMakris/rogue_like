use rltk::{Console, GameState, Rltk, RGB};
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

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        // The map is a resource, so get it from ecs world
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let ctx = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");

    // Create our gamestate with an ecs world in it.
    let mut gs = State { ecs: World::new() };

    // Register our components with the ecs world (internally creates storage systems, etc)
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    // Insert resources into our ecs world
    let map: Map = Map::new_map_rooms_and_corridors();
    // Put the player in the center of the 1st room before moving map into the ECS world
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.insert(map); // The map is now available from everywhere the ECS can see!

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
        .build();

    rltk::main_loop(ctx, gs);
}

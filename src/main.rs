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
pub use player::*;


pub struct State {
    ecs: World,
}

impl State {

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}



// SYSTEMS
// struct LeftWalker {}

// impl<'a> System<'a> for LeftWalker {
//     type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

//     fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        
//         for (_lefty, pos) in (&lefty, &mut pos).join() {
//             pos.x -=1 ;
//             if pos.x < 0 { pos.x = 79; }
//         }
//     }
// }


// STATE




fn main() {
    let ctx = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");

    // Create our gamestate with an ecs world in it.
    let mut gs = State { ecs: World::new() };

    // Register our components with the ecs world (internally creates storage systems, etc)
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // Insert resources into our ecs world
    gs.ecs.insert(new_map_test());   // The map is now available from everywhere the ECS can see!

    // Create a player entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    // // Create multiple entities
    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('☺'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(LeftMover {})
    //         .build();
    // }

    rltk::main_loop(ctx, gs);
}

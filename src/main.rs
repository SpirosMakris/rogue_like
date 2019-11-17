use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;

rltk::add_wasm_support!();

// Components
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

// #[derive(Component)]
// struct LeftMover {}

// Copy & Clone allow this enum to be used
// as a `value` type, that is, passed around by value
// instead of pointers
// PartialEq allow us to use == to see if two tile types match
#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    ecs: World,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    // Make the boundary walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a
    // decent illustration. First obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {}, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        // Render a tile depending on it's type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            },
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
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


impl State {

    fn run_systems(&mut self) {
        // let mut lw_sys = LeftWalker{};
        // lw_sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}



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
    gs.ecs.insert(new_map());   // The map is now available from everywhere the ECS can see!

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

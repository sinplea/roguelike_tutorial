use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
mod map;
mod player;

pub use components::*;
pub use map::*;
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

        check_player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(MAP_MAX_WIDTH, MAP_MAX_HEIGHT)?
        .with_title("Lands of the Abyss")
        .build()?;

    let mut game = State { ecs: World::new() };

    // Register components
    game.ecs.register::<Position>();
    game.ecs.register::<Renderable>();
    game.ecs.register::<Player>();

    let map = Map::generate();
    let (player_x, player_y) = map.rooms[0].center();

    // Add map
    game.ecs.insert(map);

    // Create a player entitiy with components
    game.ecs
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
        .build();

    rltk::main_loop(context, game)
}

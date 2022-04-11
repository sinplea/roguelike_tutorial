use super::{Map, Player, Position, State, TileType, MAP_MAX_HEIGHT, MAP_MAX_WIDTH};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let x_bound = MAP_MAX_WIDTH - 1;
        let y_bound = MAP_MAX_HEIGHT - 1;
        let destination = map.map_xy_to_index(pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination] != TileType::Wall {
            pos.x = min(x_bound, max(0, pos.x + delta_x));
            pos.y = min(y_bound, max(0, pos.y + delta_y));
        }
    }
}

pub fn check_player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right | VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up | VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down | VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

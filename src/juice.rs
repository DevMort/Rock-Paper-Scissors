use crate::game::GameState;
use crate::sprites::{Paper, Rock, Scissors};
use bevy::prelude::*;

fn within_bounds(mouse_x: f32, mouse_y: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> bool {
    mouse_x >= x1 && mouse_x <= x2 && mouse_y >= y1 && mouse_y <= y2
}

fn player_turn_hover_juice(
    windows: Res<Windows>,
    mut sprites: QuerySet<(
        QueryState<&mut Transform, With<Rock>>,
        QueryState<&mut Transform, With<Paper>>,
        QueryState<&mut Transform, With<Scissors>>,
    )>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::PlayerChoosing {
        return;
    }

    let (mouse_x, mouse_y) = match windows
        .get_primary()
        .expect("Cannot get window.")
        .cursor_position()
    {
        Some(pos) => (pos.x, pos.y),
        None => (0f32, 0f32),
    };

    // rock
    for mut rock in sprites.q0().iter_mut() {
        if within_bounds(mouse_x, mouse_y, 90f32, 210f32, 90f32, 315f32) {
            rock.scale = Vec3::splat(0.6);
        } else {
            rock.scale = Vec3::splat(0.5);
        }
    }

    // paper
    for mut paper in sprites.q1().iter_mut() {
        if within_bounds(mouse_x, mouse_y, 255f32, 390f32, 90f32, 358f32) {
            paper.scale = Vec3::splat(0.6);
        } else {
            paper.scale = Vec3::splat(0.5);
        }
    }

    // scissors
    for mut scissors in sprites.q2().iter_mut() {
        if within_bounds(mouse_x, mouse_y, 420f32, 545f32, 90f32, 355f32) {
            scissors.scale = Vec3::splat(0.6);
        } else {
            scissors.scale = Vec3::splat(0.5);
        }
    }
}

pub struct JuicePlugin;
impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_turn_hover_juice);
    }
}

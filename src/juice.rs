use crate::game::{GameState, Globals};
use crate::sprites::{Paper, Rock, Scissors};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::Rng;

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

fn turn_exit(
    mut sprites: QuerySet<(
        QueryState<&mut Transform, With<Rock>>,
        QueryState<&mut Transform, With<Paper>>,
        QueryState<&mut Transform, With<Scissors>>,
    )>,
) {
    for mut r in sprites.q0().iter_mut() {
        r.scale = Vec3::splat(0.5);
    }
    for mut p in sprites.q1().iter_mut() {
        p.scale = Vec3::splat(0.5);
    }
    for mut s in sprites.q2().iter_mut() {
        s.scale = Vec3::splat(0.5);
    }
}

fn enemy_turn_hovering(
    mut sprites: QuerySet<(
        QueryState<&mut Transform, With<Rock>>,
        QueryState<&mut Transform, With<Paper>>,
        QueryState<&mut Transform, With<Scissors>>,
    )>,
    mut globals: ResMut<Globals>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::EnemyChoosing {
        return;
    }

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(1..=3);

    match i {
        1 => {
            for mut r in sprites.q0().iter_mut() {
                r.scale = Vec3::splat(0.6);
            }
            for mut p in sprites.q1().iter_mut() {
                p.scale = Vec3::splat(0.5);
            }
            for mut s in sprites.q2().iter_mut() {
                s.scale = Vec3::splat(0.5);
            }

            globals.enemy_choice = Some(String::from("rock"));
        }
        2 => {
            for mut r in sprites.q0().iter_mut() {
                r.scale = Vec3::splat(0.5);
            }
            for mut p in sprites.q1().iter_mut() {
                p.scale = Vec3::splat(0.6);
            }
            for mut s in sprites.q2().iter_mut() {
                s.scale = Vec3::splat(0.5);
            }

            globals.enemy_choice = Some(String::from("paper"));
        }
        3 => {
            for mut r in sprites.q0().iter_mut() {
                r.scale = Vec3::splat(0.5);
            }
            for mut p in sprites.q1().iter_mut() {
                p.scale = Vec3::splat(0.5);
            }
            for mut s in sprites.q2().iter_mut() {
                s.scale = Vec3::splat(0.6);
            }

            globals.enemy_choice = Some(String::from("scissors"));
        }
        _ => {
            for mut r in sprites.q0().iter_mut() {
                r.scale = Vec3::splat(0.5);
            }
            for mut p in sprites.q1().iter_mut() {
                p.scale = Vec3::splat(0.5);
            }
            for mut s in sprites.q2().iter_mut() {
                s.scale = Vec3::splat(0.5);
            }

            globals.enemy_choice = None;
        }
    }
}

pub struct JuicePlugin;
impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerChoosing).with_system(player_turn_hover_juice),
        )
        .add_system_set(SystemSet::on_exit(GameState::PlayerChoosing).with_system(turn_exit))
        .add_system_set(
            SystemSet::on_update(GameState::EnemyChoosing)
                .with_system(enemy_turn_hovering)
                .with_run_criteria(FixedTimestep::step(0.5)),
        )
        .add_system_set(SystemSet::on_exit(GameState::EnemyChoosing).with_system(turn_exit));
    }
}

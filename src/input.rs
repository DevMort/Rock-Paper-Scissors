use crate::game::{GameState, Globals};
use bevy::{app::AppExit, prelude::*};

fn within_bounds(mouse_x: f32, mouse_y: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> bool {
    mouse_x >= x1 && mouse_x <= x2 && mouse_y >= y1 && mouse_y <= y2
}

fn handle_quit(event: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if event.just_pressed(KeyCode::Q) || event.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn mouse_click(
    event: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut globals: ResMut<Globals>,
    mut state: ResMut<State<GameState>>,
) {
    let window = windows.get_primary().expect("There is no window!");

    if event.just_pressed(MouseButton::Left) {
        let mut clicked = false;
        let (mouse_x, mouse_y) = match window.cursor_position() {
            Some(p) => (p.x, p.y),
            None => (0f32, 0f32),
        };

        // clicked rock
        if within_bounds(mouse_x, mouse_y, 90f32, 210f32, 90f32, 315f32) {
            globals.player_choice = Some(String::from("rock"));
            clicked = true;
        }

        // clicked paper
        if within_bounds(mouse_x, mouse_y, 255f32, 390f32, 90f32, 358f32) {
            globals.player_choice = Some(String::from("paper"));
            clicked = true;
        }

        // clicked scissors
        if within_bounds(mouse_x, mouse_y, 420f32, 545f32, 90f32, 355f32) {
            globals.player_choice = Some(String::from("scissors"));
            clicked = true;
        }

        if clicked {
            state
                .set(GameState::EnemyChoosing)
                .expect("Failed to change state.");
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_quit).add_system(mouse_click);
    }
}

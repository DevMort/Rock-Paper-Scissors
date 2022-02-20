use crate::game;
use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 640f32;
const WINDOW_HEIGHT: f32 = 480f32;
const SPRITES_WIDTH: f32 = 310f32;
const SPRITES_HEIGHT: f32 = 656f32;
const ROCK_SPRITE_PATH: &str = "rock.png";
const PAPER_SPRITE_PATH: &str = "paper.png";
const SCISSORS_SPRITE_PATH: &str = "scissors.png";

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn the rock
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(ROCK_SPRITE_PATH),
            transform: Transform {
                translation: Vec3::new((WINDOW_WIDTH / -2.0) + SPRITES_WIDTH / 2.0, 0.0, 1.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Rock);

    // spawn the paper
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PAPER_SPRITE_PATH),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paper);

    // spawn the scissors
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(SCISSORS_SPRITE_PATH),
            transform: Transform {
                translation: Vec3::new((WINDOW_WIDTH / 2.0) - SPRITES_WIDTH / 2.0, 0.0, 1.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Scissors);
}

fn show_middle_text(mut commands: Commands, state: Res<State<game::GameState>>) {
    let text: String = match state.current() {
        game::GameState::PlayerChoosing => {
            String::from("Choose between rock, paper, and scissors.")
        }
        game::GameState::EnemyChoosing => String::from("Enemy is choosing."),
        game::GameState::PlayerWin => String::from("You won!"),
        game::GameState::EnemyWin => String::from("Enemy won!"),
    };

    // commands.spawn_bundle(NodeBundle {});
}

#[derive(Component)]
struct Rock;
#[derive(Component)]
struct Paper;
#[derive(Component)]
struct Scissors;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_sprites)
            .add_system(show_middle_text);
    }
}

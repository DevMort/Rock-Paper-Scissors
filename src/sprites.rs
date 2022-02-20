use crate::ui::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

const SPRITES_WIDTH: f32 = 310f32;
const SPRITES_HEIGHT: f32 = 656f32;
const ROCK_SPRITE_PATH: &str = "rock.png";
const PAPER_SPRITE_PATH: &str = "paper.png";
const SCISSORS_SPRITE_PATH: &str = "scissors.png";

#[derive(Component)]
pub struct Rock;
#[derive(Component)]
pub struct Paper;
#[derive(Component)]
pub struct Scissors;

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

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites);
    }
}

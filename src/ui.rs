use crate::game;
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 640f32;
pub const WINDOW_HEIGHT: f32 = 480f32;
const FONT_PATH: &str = "Brush Stroke.ttf";

#[derive(Component)]
pub struct TextComponent;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn show_middle_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        flex_wrap: FlexWrap::Wrap,
                        position: Rect {
                            bottom: Val::Percent(5f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: String::from("Choose between rock, paper, and scissors."),
                            style: TextStyle {
                                font: asset_server.load(FONT_PATH),
                                font_size: 32f32,
                                color: Color::BLACK,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TextComponent);
        });
}

fn change_middle_text(
    state: Res<State<game::GameState>>,
    mut text_query: Query<&mut Text, With<TextComponent>>,
) {
    let text: String = match state.current() {
        game::GameState::PlayerChoosing => {
            String::from("Choose between rock, paper, and scissors.")
        }
        game::GameState::EnemyChoosing => String::from("Enemy is choosing."),
        game::GameState::PlayerWin => String::from("You won!"),
        game::GameState::EnemyWin => String::from("Enemy won!"),
    };

    for mut t in text_query.iter_mut() {
        t.sections[0].value = text.clone();
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(show_middle_text)
            .add_system_set(
                SystemSet::on_enter(game::GameState::EnemyChoosing).with_system(change_middle_text),
            );
    }
}

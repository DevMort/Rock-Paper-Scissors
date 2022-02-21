use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    PlayerChoosing,
    EnemyChoosing,
    PlayerWin,
    EnemyWin,
}

pub struct Globals {
    pub player_choice: Option<String>,
    pub enemy_choice: Option<String>,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Globals {
            player_choice: None,
            enemy_choice: None,
        })
        .add_state(GameState::PlayerChoosing);
    }
}

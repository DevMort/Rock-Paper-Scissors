use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    PlayerChoosing,
    EnemyChoosing,
    PlayerWin,
    EnemyWin,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::PlayerChoosing);
    }
}

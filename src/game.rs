use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    EnemyChoosing,
    EnemyWin,
    Evaluating,
    PlayerChoosing,
    PlayerWin,
    Tie,
}

pub struct Globals {
    pub player_choice: Option<String>,
    pub enemy_choice: Option<String>,
}

#[derive(Component)]
pub struct ChooseOnCompletionTimer(pub Timer);

fn choosing_tick(
    mut timer_query: Query<&mut ChooseOnCompletionTimer>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            state
                .set(GameState::Evaluating)
                .expect("Cannot change state to evaluating.");
        }
    }
}

fn evaluate_winner(globals: ResMut<Globals>, mut state: ResMut<State<GameState>>) {
    if globals.player_choice.clone().unwrap() == globals.enemy_choice.clone().unwrap() {
        state
            .set(GameState::Tie)
            .expect("Could not change state to Tie.");
        return;
    }

    let rock = String::from("rock");
    let paper = String::from("paper");
    let scissors = String::from("scissors");
    let player_choice = globals.player_choice.clone().unwrap();
    let enemy_choice = globals.enemy_choice.clone().unwrap();
    if player_choice == rock {
        if enemy_choice == paper {
            state
                .set(GameState::EnemyWin)
                .expect("Could not change state.");
        } else if enemy_choice == scissors {
            state
                .set(GameState::PlayerWin)
                .expect("Could not change state.");
        }
    } else if player_choice == paper {
        if enemy_choice == scissors {
            state
                .set(GameState::EnemyWin)
                .expect("Could not change state.");
        } else if enemy_choice == rock {
            state
                .set(GameState::PlayerWin)
                .expect("Could not change state.");
        }
    } else if player_choice == scissors {
        if enemy_choice == rock {
            state
                .set(GameState::EnemyWin)
                .expect("Could not change state.");
        } else if enemy_choice == paper {
            state
                .set(GameState::PlayerWin)
                .expect("Could not change state.");
        }
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Globals {
            player_choice: None,
            enemy_choice: None,
        })
        .add_state(GameState::PlayerChoosing)
        .add_system_set(SystemSet::on_update(GameState::EnemyChoosing).with_system(choosing_tick))
        .add_system_set(SystemSet::on_update(GameState::Evaluating).with_system(evaluate_winner));
    }
}

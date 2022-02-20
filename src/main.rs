mod game;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        //
        // Resources
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: String::from("Rock Paper Scissors"),
            width: 640f32,
            height: 480f32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.99, 0.89, 0.85)))
        //
        // Plugins
        .add_plugin(game::GamePlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugins(DefaultPlugins)
        //
        // Run the Game
        .run();
}

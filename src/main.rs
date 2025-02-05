use std::default;

use bevy::prelude::*;
use systems::*;

pub mod events;
pub mod game;
pub mod main_menu;
pub mod systems;

use game::*;
use main_menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(translate_to_game_state)
        .add_system(translate_to_main_menu_state)
        .run();
}

#[derive(States, Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

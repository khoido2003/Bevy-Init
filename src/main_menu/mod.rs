use bevy::prelude::*;
use systems::{
    interactions::{interact_with_play_button, interact_with_quit_button},
    layout::*,
};

use crate::AppState;

pub mod components;
pub mod styles;
pub mod systems;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}

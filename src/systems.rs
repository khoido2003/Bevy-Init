use bevy::app::AppExit;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{events::*, AppState, SimulationState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),

        ..default()
    });
}

pub fn translate_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::GameOver {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState Game");
        }
    }
}

pub fn translate_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));

            commands.insert_resource(NextState(Some(SimulationState::Paused)));

            println!("Entered AppState MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>, mut commands: Commands) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is {}", event.score.to_string());

        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

use bevy::app::{self, AppExit};
use bevy::prelude::*;

use crate::components::{PlayButton, QuitButton};
use crate::main_menu::styles::*;
use crate::AppState;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background = PRESSED_BUTTON_COLOR.into();

                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => *background = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

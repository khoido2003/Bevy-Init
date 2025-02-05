use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simualation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simualtion_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simualtion_state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));

            println!("SimulationState Paused");
        }

        if simualtion_state.0 == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));

            println!("SimulationState Running");
        }
    }
}

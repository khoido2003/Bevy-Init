use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simualation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simualation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

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

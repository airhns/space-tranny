use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{counter_window::{CounterWindow, CounterWindowAccessLightsStatus, CounterWindowStatus}, entity_updates::EntityUpdates}, structs::network_messages::EntityUpdateData};

pub fn counter_window_update(
    mut updated_counter_windows: Query<(&CounterWindow, &mut EntityUpdates), Changed<CounterWindow>>,
) {

    for (counter_window_component, mut entity_updates_component) in updated_counter_windows.iter_mut() {

        let mut animation_tree_data = HashMap::new();

        animation_tree_data.insert(
        "blend_speed".to_string(),
        EntityUpdateData::Float(0.6)
        );

        match counter_window_component.status {
            CounterWindowStatus::Open => {
                animation_tree_data.insert(
                "blend_position".to_string(),
                EntityUpdateData::Float(1.)
                );
            }
            CounterWindowStatus::Closed => {
                animation_tree_data.insert(
                "blend_position".to_string(),
                EntityUpdateData::Float(-1.)
                );
            }
        }

        entity_updates_component.updates.insert(
            "animationTree1>>parameters/blend_position".to_string(),
            animation_tree_data
        );

        let mut access_light_data = HashMap::new();

        match counter_window_component.access_lights {
            CounterWindowAccessLightsStatus::Neutral => {
                access_light_data.insert(
                "emissiveColor".to_string(),
                EntityUpdateData::String("ff2a00".to_string())
                );
            }
            CounterWindowAccessLightsStatus::Granted => {
                access_light_data.insert(
                "emissiveColor".to_string(),
                EntityUpdateData::String("08ff00".to_string())
                );
            }
            CounterWindowAccessLightsStatus::Denied => {
                access_light_data.insert(
                "emissiveColor".to_string(),
                EntityUpdateData::String("dc00e3".to_string())
                );
            }
        }


        entity_updates_component.updates.insert(
            "accessLight++material".to_string(),
            access_light_data
        );

    }

}
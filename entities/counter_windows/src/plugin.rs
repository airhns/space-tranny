use bevy::{
    math::Quat,
    prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemSet, Transform},
};
use entity::{
    entity_data::initialize_entity_data,
    spawn::{summon_base_entity, SpawnEvent},
};
use networking::messages::net_system;
use rigid_body::spawn::summon_rigid_body;
use api::{
    data::{
        EntityDataProperties, EntityDataResource, PostUpdateLabels, StartupLabels, SummoningLabels,
    },
    gridmap::GridItemData,
    tab_actions::TabActionsQueueLabels,
};

use crate::physics_events::physics_events;

use super::{
    actions::actions,
    counter_window_added::{counter_window_added, counter_window_default_map_added},
    counter_window_events::{
        counter_window_events, CounterWindowLockClosed, CounterWindowLockOpen,
        CounterWindowSensorCollision, CounterWindowUnlock, InputCounterWindowToggleOpen,
    },
    counter_window_tick_timers::counter_window_tick_timers,
    entity_update::counter_window_update,
    net::NetCounterWindow,
    spawn::{
        default_summon_counter_window, summon_counter_window, summon_raw_counter_window,
        CounterWindowSummoner, BRIDGE_COUNTER_WINDOW_ENTITY_NAME,
        SECURITY_COUNTER_WINDOW_ENTITY_NAME,
    },
};
use bevy::app::CoreStage::PostUpdate;

pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CounterWindowSensorCollision>()
            .add_system(counter_window_tick_timers)
            .add_system(counter_window_events)
            .add_system(counter_window_added)
            .add_system(counter_window_default_map_added)
            .add_system(physics_events)
            .add_event::<InputCounterWindowToggleOpen>()
            .add_event::<CounterWindowLockOpen>()
            .add_event::<CounterWindowLockClosed>()
            .add_event::<CounterWindowUnlock>()
            .add_event::<NetCounterWindow>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(counter_window_update),
            )
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(actions.after(TabActionsQueueLabels::TabAction))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetCounterWindow>),
            )
            .add_system(
                summon_counter_window::<CounterWindowSummoner>
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_base_entity::<CounterWindowSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<CounterWindowSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_counter_window).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<CounterWindowSummoner>>()
            .add_system(
                (default_summon_counter_window)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: SECURITY_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["securityCounter1".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: BRIDGE_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["bridgeCounter".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}

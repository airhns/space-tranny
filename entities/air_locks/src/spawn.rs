use std::{collections::BTreeMap, sync::Arc};

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoData> for AirlockSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let description;
        let sub_name;

        if spawn_data.entity_name == SECURITY_AIRLOCK_ENTITY_NAME {
            sub_name = "security";
            description = "An air lock with ".to_string()
                + "security"
                + " department colors. It will only grant access to security personnel.";
        } else if spawn_data.entity_name == BRIDGE_AIRLOCK_ENTITY_NAME {
            sub_name = "bridge";
            description = "An air lock with ".to_string()
                + "bridge"
                + " department colors. It will only grant access to high ranked personnel.";
        } else if spawn_data.entity_name == GOVERNMENT_AIRLOCK_ENTITY_NAME {
            sub_name = "government";

            description = "An air lock with ".to_string()
                + "government"
                + " department colors. It will only grant access to a select few.";
        } else if spawn_data.entity_name == VACUUM_AIRLOCK_ENTITY_NAME {
            sub_name = "vacuum";
            description = "An air lock with ".to_string()
                + "danger markings"
                + ". On the other side is nothing but space.";
        } else {
            warn!("Unrecognized airlock sub-type {}", spawn_data.entity_name);
            sub_name = "ERR";
            description = "ERR ".to_string();
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, description);
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                name: RichName {
                    name: sub_name.to_string() + " airlock",
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            entity_name: spawn_data.entity_name.to_string(),
            entity_group: EntityGroup::AirLock,
            tab_actions_option: Some(TabActions {
                tab_actions: vec![
                    TabAction {
                        id: "actions::air_locks/toggleopen".to_string(),
                        text: "Toggle Open".to_string(),
                        tab_list_priority: 100,
                        prerequisite_check: Arc::new(toggle_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/lockopen".to_string(),
                        text: "Lock Open".to_string(),
                        tab_list_priority: 99,
                        prerequisite_check: Arc::new(lock_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/lockclosed".to_string(),
                        text: "Lock Closed".to_string(),
                        tab_list_priority: 98,
                        prerequisite_check: Arc::new(lock_closed_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/unlock".to_string(),
                        text: "Unlock".to_string(),
                        tab_list_priority: 97,
                        prerequisite_check: Arc::new(unlock_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                ],
            }),
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            default_map_spawn: spawn_data.default_map_spawn,
        }
    }
}

use bevy::{
    math::Vec3,
    prelude::{warn, Commands, EventReader, EventWriter, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::{
    entity_data::RawSpawnEvent,
    spawn::{BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, SpawnData, SpawnEvent},
};
use pawn::pawn::ShipAuthorizationEnum;
use rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable};
use api::{
    chat::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
    converters::string_transform_to_transform,
    data::NoData,
    entity_updates::EntityGroup,
    examinable::{Examinable, RichName},
    health::Health,
    tab_actions::{TabAction, TabActions},
};

use super::actions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action};

pub const DEFAULT_AIR_LOCK_Y: f32 = 1.;

impl RigidBodySummonable<NoData> for AirlockSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 1., 0.2),
            collider_transform: Transform::from_translation(Vec3::new(0., DEFAULT_AIR_LOCK_Y, 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}

use super::air_lock::AirLock;

pub struct AirlockSummoner;

pub fn summon_air_lock<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut airlock_spawns: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in airlock_spawns.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(AirLock {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });
    }
}

pub const SECURITY_AIRLOCK_ENTITY_NAME: &str = "securityAirLock1";
pub const BRIDGE_AIRLOCK_ENTITY_NAME: &str = "bridgeAirLock";
pub const GOVERNMENT_AIRLOCK_ENTITY_NAME: &str = "governmentAirLock";
pub const VACUUM_AIRLOCK_ENTITY_NAME: &str = "vacuumAirLock";

pub fn default_summon_air_lock(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<AirlockSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != SECURITY_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != BRIDGE_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != GOVERNMENT_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: AirlockSummoner,
        });
    }
}

pub fn summon_raw_air_lock(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_air_lock: EventWriter<SpawnEvent<AirlockSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != SECURITY_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != BRIDGE_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != GOVERNMENT_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_air_lock.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: AirlockSummoner,
        });
    }
}

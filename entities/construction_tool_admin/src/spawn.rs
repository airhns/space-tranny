use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "admin construction tool".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}
use std::{collections::HashMap, sync::Arc};

impl InventoryItemSummonable for ConstructionToolSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();
        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(0.0697873, -0.966557, -0.246774), 1.8711933),
                Vec3::new(-0.047, 0.024, -0.035),
            )),
        );
        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(-0.1942536, 0.9779768, 0.076334), 2.1748603),
                Vec3::new(0.042, -0., -0.021),
            )),
        );
        attachment_transforms.insert(
            "holster".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(-0.6264298, -0.1219246, 0.7698832), 2.4247889),
                Vec3::new(0., -0.093, 0.036),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                drop_transform: get_default_transform(),
                active_slot_tab_actions: vec![
                    TabAction {
                        id: "action::construction_tool_admin/construct".to_string(),
                        text: "Construct".to_string(),
                        tab_list_priority: 50,
                        prerequisite_check: Arc::new(construct_action),
                        belonging_entity: spawn_data.held_entity_option,
                    },
                    TabAction {
                        id: "action::construction_tool_admin/deconstruct".to_string(),
                        text: "Deconstruct".to_string(),
                        tab_list_priority: 49,
                        prerequisite_check: Arc::new(deconstruct_action),
                        belonging_entity: spawn_data.held_entity_option,
                    },
                    TabAction {
                        id: "action::construction_tool_admin/constructionoptions".to_string(),
                        text: "Construction Options".to_string(),
                        tab_list_priority: 48,
                        prerequisite_check: Arc::new(construction_option_action),
                        belonging_entity: spawn_data.held_entity_option,
                    },
                ],
                attachment_transforms: attachment_transforms.clone(),
                slot_type: SlotType::Holster,
                combat_melee_damage_model: DamageModel {
                    brute: 9.,
                    damage_flags: melee_damage_flags.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
use bevy::math::{Mat4, Quat, Vec3};
use bevy::prelude::{Commands, EventReader, EventWriter, Transform};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::entity_data::{RawSpawnEvent, CONSTRUCTION_TOOL_ENTITY_NAME};
use entity::spawn::{
    BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, SpawnData, SpawnEvent,
};
use inventory_item::item::InventoryItem;
use inventory_item::spawn::{InventoryItemBundle, InventoryItemSummonable};
use rigid_body::rigid_body::STANDARD_BODY_FRICTION;
use rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable};
use api::combat::{DamageFlag, DamageModel};
use api::converters::string_transform_to_transform;
use api::data::NoData;
use api::examinable::{Examinable, RichName};
use api::inventory::SlotType;
use api::tab_actions::TabAction;

use crate::action::{construct_action, construction_option_action, deconstruct_action};

use super::construction_tool::ConstructionTool;

impl RigidBodySummonable<NoData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,
            ..Default::default()
        }
    }
}

pub struct ConstructionToolSummoner;

pub fn summon_construction_tool<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(ConstructionTool::default());
    }
}

pub fn summon_raw_construction_tool(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<ConstructionToolSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != CONSTRUCTION_TOOL_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: ConstructionToolSummoner,
        });
    }
}

pub fn default_summon_construction_tool(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<ConstructionToolSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != CONSTRUCTION_TOOL_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: ConstructionToolSummoner,
        });
    }
}

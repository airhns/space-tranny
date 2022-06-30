pub mod entity_bundle;
pub mod rigidbody_bundle;

use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, ResMut},
};
use bevy_rapier3d::prelude::{Dominance, LockedAxes};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        chat::components::{Radio, RadioChannel},
        connected_player::functions::name_generator::get_dummy_name,
        data_link::components::{DataLink, DataLinkType},
        entity::{
            functions::spawn_entity::spawn_held_entity,
            resources::{EntityDataResource, PawnDesignation, SpawnPawnData},
            spawn::{DefaultSpawnEvent, SpawnEvent},
        },
        humanoid::components::Humanoid,
        inventory::components::{Inventory, Slot, SlotType},
        map::components::Map,
        pawn::{
            components::{
                ControllerInput, Pawn, PersistentPlayerData, ShipAuthorization,
                ShipAuthorizationEnum, ShipJobsEnum,
            },
            resources::UsedNames,
        },
        physics::components::{WorldMode, WorldModes},
        senser::components::Senser,
        tab_actions::functions::get_tab_action,
    },
    entities::{
        helmet_security::spawn::HELMET_SECURITY_ENTITY_NAME,
        jumpsuit_security::spawn::JUMPSUIT_SECURITY_ENTITY_NAME,
    },
};

use self::rigidbody_bundle::R;

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

pub struct HumanMaleSummoner {
    pub character_name: String,
    pub user_name: String,
    pub spawn_pawn_data: SpawnPawnData,
}

impl HumanMaleSummonable for HumanMaleSummoner {
    fn get_character_name(&self) -> String {
        self.character_name.clone()
    }
    fn get_user_name(&self) -> String {
        self.user_name.clone()
    }
    fn get_spawn_pawn_data(&self) -> SpawnPawnData {
        self.spawn_pawn_data.clone()
    }
}

pub trait HumanMaleSummonable {
    fn get_character_name(&self) -> String;
    fn get_user_name(&self) -> String;
    fn get_spawn_pawn_data(&self) -> SpawnPawnData;
}

pub fn summon_human_male<T: HumanMaleSummonable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
    entity_data: ResMut<EntityDataResource>,
) {
    for spawn_event in spawn_events.iter() {
        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let spawn_pawn_data = spawn_event.summoner.get_spawn_pawn_data();

        if spawn_event.spawn_data.showcase_data_option.is_none() {
            let mut pawn_component = Pawn {
                name: spawn_event.summoner.get_character_name().clone(),
                job: ShipJobsEnum::Security,
                ..Default::default()
            };

            spawner.remove::<Transform>();
            let mut new_transform = spawn_event.spawn_data.entity_transform;
            new_transform.translation.y = 0.9 - R;
            spawner.insert(new_transform);

            pawn_component.tab_actions_add(
                "actions::pawn/examine",
                None,
                get_tab_action("actions::pawn/examine").unwrap(),
            );
            pawn_component.tab_actions_add(
                "actions::inventory/pickup",
                None,
                get_tab_action("actions::inventory/pickup").unwrap(),
            );

            spawner.insert_bundle((
                Senser::default(),
                Radio {
                    listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                    speak_access: vec![RadioChannel::Common, RadioChannel::Security],
                },
                ShipAuthorization {
                    access: vec![ShipAuthorizationEnum::Security],
                },
                pawn_component,
                ControllerInput::default(),
            ));

            match spawn_pawn_data.designation {
                PawnDesignation::Player => {
                    spawner.insert_bundle((
                        spawn_pawn_data.connected_player_option.unwrap(),
                        DataLink {
                            links: vec![
                                DataLinkType::FullAtmospherics,
                                DataLinkType::RemoteLock,
                                DataLinkType::ShipEngineeringKnowledge,
                            ],
                        },
                        Map {
                            available_display_modes: vec![
                                ("Standard".to_string(), "standard".to_string()),
                                (
                                    "Atmospherics Liveable".to_string(),
                                    "atmospherics_liveable".to_string(),
                                ),
                                (
                                    "Atmospherics Temperature".to_string(),
                                    "atmospherics_temperature".to_string(),
                                ),
                                (
                                    "Atmospherics Pressure".to_string(),
                                    "atmospherics_pressure".to_string(),
                                ),
                            ],
                            ..Default::default()
                        },
                    ));
                }
                _ => (),
            }
        }

        spawner.insert_bundle((
            Humanoid {
                character_name: spawn_event.summoner.get_character_name().clone(),
                ..Default::default()
            },
            PersistentPlayerData {
                character_name: spawn_event.summoner.get_character_name().clone(),
                user_name: spawn_event.summoner.get_user_name().clone(),
                ..Default::default()
            },
            WorldMode {
                mode: WorldModes::Kinematic,
            },
        ));

        spawner
            .insert(Dominance::group(10))
            .insert(LockedAxes::ROTATION_LOCKED);

        let mut slot_entities: HashMap<String, Entity> = HashMap::new();

        for (slot_name, item_name) in spawn_pawn_data.inventory_setup.iter() {
            let entity_option;

            entity_option = spawn_held_entity(
                item_name.to_string(),
                &mut commands,
                spawn_event.spawn_data.entity,
                spawn_event.spawn_data.showcase_data_option.clone(),
                &entity_data,
                &mut default_spawner,
            );

            match entity_option {
                Some(entity) => {
                    slot_entities.insert(slot_name.to_string(), entity);
                }
                None => {}
            }
        }

        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let left_hand_item;
        match slot_entities.get(&"left_hand".to_string()) {
            Some(entity) => {
                left_hand_item = Some(*entity);
            }
            None => {
                left_hand_item = None;
            }
        }
        let right_hand_item;
        match slot_entities.get(&"right_hand".to_string()) {
            Some(entity) => {
                right_hand_item = Some(*entity);
            }
            None => {
                right_hand_item = None;
            }
        }
        let helmet_hand_item;
        match slot_entities.get(&"helmet".to_string()) {
            Some(entity) => {
                helmet_hand_item = Some(*entity);
            }
            None => {
                helmet_hand_item = None;
            }
        }
        let jumpsuit_hand_item;
        match slot_entities.get(&"jumpsuit".to_string()) {
            Some(entity) => {
                jumpsuit_hand_item = Some(*entity);
            }
            None => {
                jumpsuit_hand_item = None;
            }
        }
        let holster_hand_item;
        match slot_entities.get(&"holster".to_string()) {
            Some(entity) => {
                holster_hand_item = Some(*entity);
            }
            None => {
                holster_hand_item = None;
            }
        }

        spawner.insert(Inventory {
            slots: vec![
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "left_hand".to_string(),
                    slot_item: left_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/leftHand/Position3D".to_string(),
                    ),
                },
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "right_hand".to_string(),
                    slot_item: right_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/rightHand/Position3D".to_string(),
                    ),
                },
                Slot {
                    slot_type: SlotType::Helmet,
                    slot_name: "helmet".to_string(),
                    slot_item: helmet_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/head/Position3D".to_string(),
                    ),
                },
                Slot {
                    slot_type: SlotType::Jumpsuit,
                    slot_name: "jumpsuit".to_string(),
                    slot_item: jumpsuit_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/humanMale".to_string()),
                },
                Slot {
                    slot_type: SlotType::Holster,
                    slot_name: "holster".to_string(),
                    slot_item: holster_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/holster/Position3D".to_string(),
                    ),
                },
            ],
            active_slot: "left_hand".to_string(),
            ..Default::default()
        });
    }
}

pub const HUMAN_DUMMY_ENTITY_NAME: &str = "humanDummy";
pub const HUMAN_MALE_ENTITY_NAME: &str = "humanMale";

pub fn default_human_dummy(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<HumanMaleSummoner>>,
    mut used_names: ResMut<UsedNames>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name == HUMAN_DUMMY_ENTITY_NAME {
            spawner.send(SpawnEvent {
                spawn_data: spawn_event.spawn_data.clone(),
                summoner: HumanMaleSummoner {
                    character_name: get_dummy_name(&mut used_names),
                    user_name: "DUMMY_USER_NAME".to_string(),
                    spawn_pawn_data: SpawnPawnData {
                        persistent_player_data: PersistentPlayerData::default(),
                        connected_player_option: None,
                        inventory_setup: vec![
                            (
                                "jumpsuit".to_string(),
                                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
                            ),
                            (
                                "helmet".to_string(),
                                HELMET_SECURITY_ENTITY_NAME.to_string(),
                            ),
                        ],
                        designation: PawnDesignation::Dummy,
                    },
                },
            });
        }
    }
}
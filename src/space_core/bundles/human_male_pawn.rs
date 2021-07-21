use std::collections::HashMap;

use bevy::{math::{Vec2, Vec3}, prelude::{Commands, Entity, EventWriter, Transform}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMassProps, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyForces, RigidBodyMassPropsFlags, RigidBodyType};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, connected_player::ConnectedPlayer, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, inventory::{Inventory, Slot, SlotType}, pawn::Pawn, persistent_player_data::PersistentPlayerData, player_input::PlayerInput, radio::{Radio, RadioChannel}, sensable::Sensable, showcase::Showcase, space_access::SpaceAccess, standard_character::{CharacterAnimationState, StandardCharacter}, visible_checker::VisibleChecker, world_mode::{WorldMode, WorldModes}}, enums::{space_access_enum::SpaceAccessEnum, space_jobs::SpaceJobsEnum}, events::net::net_showcase::NetShowcase, functions::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, spawn_entity::{ spawn_held_entity}, transform_to_isometry::transform_to_isometry}, structs::network_messages::ReliableServerMessage};

pub struct HumanMalePawnBundle;

impl HumanMalePawnBundle {

    pub fn spawn(
        passed_transform : Transform,
        commands : &mut Commands,
        persistent_player_data_component : &PersistentPlayerData,
        connected_player_component : &ConnectedPlayer,
        passed_inventory_setup : HashMap<String,String>,
        showcase_instance : bool,
        mut net_showcase : Option<&mut EventWriter<NetShowcase>>,
    ) -> Entity {

        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: transform_to_isometry(passed_transform).into(),
            forces : RigidBodyForces {
                gravity_scale: 1.,
                ..Default::default()
            },
            ccd: RigidBodyCcd {
                ccd_enabled: false,
                ..Default::default()
            },
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y| RigidBodyMassPropsFlags::ROTATION_LOCKED_Z).into(),
            ..Default::default()
        };


        let r = 0.25;
        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            
            shape: ColliderShape::capsule(
                Vec3::new(0.0,0.0+r,0.0).into(),
                Vec3::new(0.0,1.8-r,0.0).into(),
                r
            ),
            position: Vec3::new(0., 0., 0.).into(),
            collider_type: ColliderType::Solid,
            mass_properties: ColliderMassProps::Density(1.0),
            material: ColliderMaterial {
                friction: 0.0,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        

        let mut entity_builder = commands.spawn_bundle(rigid_body_component);
        
        entity_builder.insert_bundle(
            collider_component,
        ).insert_bundle((
            EntityData {
                entity_class : "entity".to_string(),
                entity_type : "humanMale".to_string(),
                entity_group: EntityGroup::Pawn
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles:HashMap::new(),
                updates_difference: HashMap::new(),
            },
            WorldMode {
                mode : WorldModes::Kinematic
            },
            StandardCharacter {
                current_animation_state : CharacterAnimationState::Idle,
                character_name: persistent_player_data_component.character_name.clone(),
            },
            CachedBroadcastTransform::new(),
            PersistentPlayerData {
                character_name: persistent_player_data_component.character_name.clone(),
            },
        ));

        let human_male_entity =  entity_builder.id();

        let mut slot_entities : HashMap<String, Entity>= HashMap::new();

        for (slot_name, item_name) in passed_inventory_setup.iter() {

            let entity = spawn_held_entity(
                item_name.to_string(),
                commands, 
                human_male_entity,
                showcase_instance,
                Some(connected_player_component.handle),
                &mut net_showcase,
            );

            slot_entities.insert(slot_name.to_string(),entity);

        }

        let left_hand_item;
        match slot_entities.get(&"left_hand".to_string()) {
            Some(entity) => {
                left_hand_item = Some(*entity);
            },
            None => {
                left_hand_item = None;
            },
        }
        let right_hand_item;
        match slot_entities.get(&"right_hand".to_string()) {
            Some(entity) => {
                right_hand_item = Some(*entity);
            },
            None => {
                right_hand_item = None;
            },
        }
        let helmet_hand_item;
        match slot_entities.get(&"helmet".to_string()) {
            Some(entity) => {
                helmet_hand_item = Some(*entity);
            },
            None => {
                helmet_hand_item = None;
            },
        }
        let jumpsuit_hand_item;
        match slot_entities.get(&"jumpsuit".to_string()) {
            Some(entity) => {
                jumpsuit_hand_item = Some(*entity);
            },
            None => {
                jumpsuit_hand_item = None;
            },
        }

        let inventory_component = Inventory {
            slots: vec![
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "left_hand".to_string(),
                    slot_item: left_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/leftHand/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "right_hand".to_string(),
                    slot_item: right_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/rightHand/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Helmet,
                    slot_name: "helmet".to_string(),
                    slot_item: helmet_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/head/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Jumpsuit,
                    slot_name: "jumpsuit".to_string(),
                    slot_item: jumpsuit_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/humanMale".to_string()),
                },
            ],
            pickup_slot: "left_hand".to_string(),
        };

        let examine_text = "".to_string();

        let examinable_component = Examinable {
            description: examine_text,
            name: "a male human".to_string(),
        };



        let mut entity_commands = commands.entity(human_male_entity);
        
        entity_commands.insert_bundle((inventory_component,examinable_component));

        if showcase_instance {
            entity_commands.insert(
                Showcase {
                    handle: connected_player_component.handle,
                }
            );
            let entity_updates = HashMap::new();
            net_showcase.unwrap().send(NetShowcase{
                handle: connected_player_component.handle,
                message: ReliableServerMessage::LoadEntity(
                    "entity".to_string(),
                    "humanMale".to_string(),
                    entity_updates,
                    human_male_entity.to_bits(),
                    true,
                    "main".to_string(),
                    "HBoxContainer/Control2/ViewportContainer/Viewport/Spatial".to_string(),
                    false,
                )
            });
        } else {

            entity_commands.insert_bundle((
                VisibleChecker,
                Sensable{
                    is_audible : false,
                    is_light:false,
                    sensed_by_cached:vec![],
                    sensed_by:vec![],
                    always_sensed : false
                },
                ConnectedPlayer {
                    handle: connected_player_component.handle,
                    authid: connected_player_component.authid,
                    rcon : false,
                },
                Radio {
                    listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                    speak_access: vec![RadioChannel::Common, RadioChannel::Security],
                },
                SpaceAccess{
                    access : vec![SpaceAccessEnum::Security]
                },
                Pawn {
                    name: persistent_player_data_component.character_name.clone(),
                    job: SpaceJobsEnum::Security
                },
                PlayerInput{
                    movement_vector : Vec2::ZERO,
                    sprinting : false
                },
            ));
        }

        human_male_entity

    }

}
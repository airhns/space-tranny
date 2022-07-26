use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{warn, Commands, Entity, GlobalTransform, ResMut, Transform},
};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody,
};
use data_converters::converters::string_vec3_to_vec3;
use physics::physics::{get_bit_masks, ColliderGroup, CHARACTER_FLOOR_FRICTION};
use api::{
    data::Vec3Int,
    gridmap::{
        cell_id_to_world, to_doryen_coordinates, CellData, GridmapData, GridmapDetails1,
        GridmapMain,
    },
    health::{HealthFlag, StructureHealth},
};

pub fn build_gridmap_floor(commands: &mut Commands) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    //Floor

    let mut friction_component = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
    friction_component.combine_rule = CoefficientCombineRule::Average;

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(Vec3::new(0., -1., 0.)))
        .insert(GlobalTransform::default())
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(500., 1., 500.))
                .insert(friction_component)
                .insert(CollisionGroups::new(masks.0, masks.1))
                .insert(Transform::default())
                .insert(GlobalTransform::default());
        });

    //Roof

    let mut friction_component = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
    friction_component.combine_rule = CoefficientCombineRule::Min;

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(Vec3::new(0., 3., 0.)))
        .insert(GlobalTransform::default())
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(500., 1., 500.))
                .insert(friction_component)
                .insert(CollisionGroups::new(masks.0, masks.1))
                .insert(Transform::default())
                .insert(GlobalTransform::default());
        });
}

use std::collections::HashMap;

use crate::events::{Cell, CellDataWID};

use super::fov::DoryenMap;
pub fn build_main_gridmap(
    current_map_main_data: &Vec<CellDataWID>,
    mut commands: &mut Commands,
    gridmap_main: &mut ResMut<GridmapMain>,
    fov_map: &mut ResMut<DoryenMap>,
    gridmap_data: &mut ResMut<GridmapData>,
) {
    let mut health_flags = HashMap::new();

    health_flags.insert(0, HealthFlag::ArmourPlated);

    for cell_data in current_map_main_data.iter() {
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int {
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        let cell_item_id;

        match gridmap_data.main_name_id_map.get(&cell_data.item) {
            Some(x) => {
                cell_item_id = *x;
            }
            None => {
                warn!("Couldnt find item {}", cell_data.item);
                break;
            }
        };

        if cell_id_int.y == 0 {
            // Wall

            if !gridmap_data
                .non_fov_blocking_cells_list
                .contains(&cell_item_id)
            {
                let coords = to_doryen_coordinates(cell_id_int.x, cell_id_int.z);
                fov_map.map.set_transparent(coords.0, coords.1, false);
            }
        } else {
            // Floor cells dont have collision. Don't need to be an entity at this moment either.
            gridmap_main.grid_data.insert(
                cell_id_int,
                CellData {
                    item: cell_item_id,
                    orientation: cell_data.orientation,
                    health: StructureHealth {
                        health_flags: health_flags.clone(),
                        ..Default::default()
                    },
                    entity: None,
                },
            );
            continue;
        }

        let entity_op = spawn_main_cell(
            &mut commands,
            cell_id_int,
            cell_item_id,
            cell_data.orientation,
            &gridmap_data,
        );

        gridmap_main.grid_data.insert(
            cell_id_int,
            CellData {
                item: cell_item_id,
                orientation: cell_data.orientation,
                health: StructureHealth {
                    health_flags: health_flags.clone(),
                    ..Default::default()
                },
                entity: Some(entity_op),
            },
        );
    }
}

pub fn build_details1_gridmap(
    current_map_details1_data: &Vec<CellDataWID>,
    gridmap_details1: &mut ResMut<GridmapDetails1>,
    gridmap_data: &mut ResMut<GridmapData>,
) {
    for cell_data in current_map_details1_data.iter() {
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int {
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        gridmap_details1.data.insert(
            cell_id_int,
            CellData {
                item: *gridmap_data
                    .details1_name_id_map
                    .get(&cell_data.item)
                    .unwrap(),
                orientation: cell_data.orientation,
                health: StructureHealth::default(),
                entity: None,
            },
        );
    }
}

pub fn spawn_main_cell(
    commands: &mut Commands,
    cell_id: Vec3Int,
    cell_item_id: i64,
    _cell_rotation: i64,
    gridmap_data: &GridmapData,
) -> Entity {
    let cell_properties;
    match gridmap_data.main_cell_properties.get(&cell_item_id) {
        Some(x) => {
            cell_properties = x;
        }
        None => {
            warn!("Unknown cellid {}. Initialization of gridmap cell in startup gridmap systems missing.", cell_item_id);
            return Entity::from_bits(0);
        }
    }

    let mut world_position = Transform::from_translation(cell_id_to_world(cell_id));
    world_position.translation += cell_properties.collider_position.translation;
    world_position.rotation *= cell_properties.collider_position.rotation;

    let mut entity_builder = commands.spawn();
    entity_builder
        .insert(RigidBody::Fixed)
        .insert(GlobalTransform::default())
        .insert(world_position)
        .insert(Cell { id: cell_id });

    let entity_id = entity_builder.id();

    let masks = get_bit_masks(ColliderGroup::Standard);

    let mut friction_component = Friction::coefficient(cell_properties.friction);
    friction_component.combine_rule = cell_properties.combine_rule;

    entity_builder.with_children(|children| {
        children
            .spawn()
            .insert(cell_properties.collider.clone())
            .insert(Transform::identity())
            .insert(friction_component)
            .insert(CollisionGroups::new(masks.0, masks.1));
    });

    entity_id
}

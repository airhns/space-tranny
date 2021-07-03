use bevy::prelude::{Entity, EventReader, Query, info};

use crate::space_core::{components::{inventory::Inventory, pickupable::Pickupable, world_mode::{WorldMode, WorldModes}}, events::general::use_world_item::UseWorldItem};

pub fn pickup_world_item(
    mut use_world_item_events : EventReader<UseWorldItem>,
    mut inventory_entities : Query<&mut Inventory>,
    mut pickupable_entities : Query<(&mut Pickupable, &mut WorldMode)>,
) {

    for event in use_world_item_events.iter() {

        let pickuper_components_option = inventory_entities.get_mut(event.pickuper_entity);
        let pickuper_components;

        match pickuper_components_option {
            Ok(components) => {
                pickuper_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }


        let mut pickuper_inventory = pickuper_components;

        let pickup_slot = pickuper_inventory.pickup_slot.clone();

        let pickup_slot = pickuper_inventory.get_slot_mut(&pickup_slot);

        if !matches!(pickup_slot.slot_item, None) {
            continue;
        }

        let pickupable_entities_components;

        let pickupable_entity = Entity::new(event.pickupable_entity_id);

        match pickupable_entities.get_mut(pickupable_entity) {
            Ok(components) => {
                pickupable_entities_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }


        let mut pickupable_world_mode = pickupable_entities_components.1;
        let mut pickupable_component = pickupable_entities_components.0;

        if !matches!(pickupable_component.in_inventory_of_entity, None) {
            continue;
        }



        pickupable_component.in_inventory_of_entity = Some(event.pickuper_entity);
        pickup_slot.slot_item = Some(pickupable_entity);
        pickupable_world_mode.mode = WorldModes::Held;

        


    }

}

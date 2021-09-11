use bevy::prelude::{Entity, EventWriter, ResMut};

use crate::space_core::{events::net::net_console_commands::NetConsoleCommands, resources::{network_messages::ReliableServerMessage, used_names::UsedNames}};

pub fn player_selector_to_entities(
    command_executor_entity : Entity,
    command_executor_handle : u32,
    player_selector : &str,
    used_names : &mut ResMut<UsedNames>,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
) -> Vec<Entity> {

    let mut target_entities = vec![];

    if player_selector == "*" {

        for entity in used_names.names.values() {
            target_entities.push(*entity);
        }

    } else if player_selector == "@me" {

        target_entities.push(command_executor_entity);

    } else {

        // Assume we only target one player.

        let mut found_one_match = false;
        let mut conflicting_names = vec![];

        for (player_name, entity) in used_names.names.iter() {

            if player_name.to_lowercase().contains(&player_selector.to_lowercase()) {

                if found_one_match {
                    found_one_match=false;
                    conflicting_names.push((player_name, entity));
                } else {
                    found_one_match=true;
                    conflicting_names.push((player_name, entity));
                }

            }

        }

        if found_one_match {

            target_entities.push(*conflicting_names[0].1);

        } else {

            let mut conflicting_message = "[color=#ff6600]Player selector is not specific enough:\n".to_string();

            for (name, _entity) in conflicting_names.iter() {

                conflicting_message = conflicting_message + name + "\n";

            }

            conflicting_message = conflicting_message + "[/color]";

            net_console_commands.send(NetConsoleCommands {
                handle: command_executor_handle,
                message: ReliableServerMessage::ConsoleWriteLine(conflicting_message),
            });

        }

    }

    target_entities

}
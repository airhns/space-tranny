use std::collections::HashMap;

use bevy::prelude::{Entity, EventWriter, Local, Query};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_console_commands::NetConsoleCommands, structs::network_messages::ReliableServerMessage};

const RCON_PASSWORD  : &str = "KA-BAR";


#[derive(Default)]
pub struct BruteforceProtection {

    pub tracking_data : HashMap<u32, u8>,
    pub blacklist : Vec<u32>,

}

pub fn rcon_authorization(
    bruteforce_protection : &mut Local<BruteforceProtection>,
    connected_players : &mut Query<&mut ConnectedPlayer>,
    client_handle : u32,
    client_entity : Entity,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
    input_password : String,
) {



    if bruteforce_protection.blacklist.contains(&client_handle) {

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=#ff0000]Too many past attempts, blacklisted.[/color]"
                .to_string()
            ),
        });
        return;
        
    }

    if input_password == RCON_PASSWORD {

        let mut connected_player_component = connected_players.get_mut(client_entity).unwrap();

        connected_player_component.rcon = true;

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=#3cff00]RCON status granted![/color]"
                .to_string()
            ),
        });


    } else {
        match bruteforce_protection.tracking_data.get_mut(&client_handle) {
            Some(attempt_amount) => {
                *attempt_amount+=1;
                if attempt_amount > &mut 10 {
                    bruteforce_protection.blacklist.push(client_handle);
                }
            },
            None => {
                bruteforce_protection.tracking_data.insert(client_handle, 1);
            },
        }

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=#ff6600]Wrong password.[/color]"
                .to_string()
            ),
        });

    }
    

}
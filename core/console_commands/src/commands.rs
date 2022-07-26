use bevy::prelude::{info, Entity, EventWriter, ResMut};
use pawn::pawn::UsedNames;
use api::{
    console_commands::ConsoleCommandVariant,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};

pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetConsoleCommands {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetEntityConsole {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetEntityConsole {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

pub fn player_selector_to_entities(
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u64>,
    mut player_selector: &str,
    used_names: &mut ResMut<UsedNames>,
    net_console_commands: &mut EventWriter<NetEntityConsole>,
) -> Vec<Entity> {
    if player_selector == "*" {
        return used_names.names.values().copied().collect();
    } else if player_selector == "@me" {
        return vec![command_executor_entity];
    }

    let precise_match = if (player_selector.starts_with('"') && player_selector.ends_with('"'))
        || (player_selector.starts_with('\'') && player_selector.ends_with('\''))
    {
        // Remove surrounding quotes
        let mut chars = player_selector.chars();
        chars.next();
        chars.next_back();
        player_selector = chars.as_str();
        true
    } else {
        false
    };

    let matching_names: Vec<_> = used_names
        .names
        .iter()
        .filter(|(player_name, _)| {
            let player_name_lower = player_name.to_lowercase();
            let player_selector = player_selector.to_lowercase();
            let val;
            if precise_match {
                val = player_name_lower == player_selector;
            } else {
                val = player_name_lower.contains(&player_selector);
            }

            val
        })
        .collect();

    let message = match &matching_names[..] {
        [(_, &entity)] => return vec![entity],
        [] => {
            format!("Couldn't find player \"{player_selector}\"")
        }
        [conflicts @ ..] => {
            let mut names = String::new();
            for (name, _entity) in conflicts.iter() {
                names.push_str(name);
                names.push('\n');
            }

            format!("Player selector \"{player_selector}\" is not specific enough.\n{names}")
        }
    };
    if let Some(handle) = command_executor_handle_option {
        net_console_commands.send(NetEntityConsole {
            handle,
            message: ReliableServerMessage::ConsoleWriteLine(format!(
                "[color=#ff6600]{message}[/color]"
            )),
        });
    }
    vec![]
}

#[derive(Default)]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), ConsoleCommandVariant::String)],
    ));

    commands.list.push((
        "rconStatus".to_string(),
        "For server administrators only. Check if the server has granted you the RCON status."
            .to_string(),
        vec![],
    ));

    info!("Loaded {} different console commands.", commands.list.len());
}

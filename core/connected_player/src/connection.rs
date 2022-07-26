use bevy::{
    math::Vec2,
    prelude::{info, Commands, Component, EventReader, EventWriter, Query, Res, ResMut},
};
use bevy_renet::renet::ServerEvent;
use console_commands::{commands::AllConsoleCommands, rcon::GiveAllRCON};
use humanoid::humanoid::{CharacterAnimationState, Humanoid};
use map::map_input::MapData;
use networking::messages::NetPlayerConn;
use pawn::pawn::{ControllerInput, PersistentPlayerData, UsedNames};
use api::{
    data::{ConnectedPlayer, HandleToEntity, ServerId, TickRate},
    gridmap::GridmapData,
    pawn::PawnDesignation,
};

use crate::{connection_events::on_new_player_connection, health_ui::ClientHealthUICache};

#[derive(Component)]
pub struct Boarding;

#[derive(Component)]
pub struct SetupPhase;

#[derive(Component)]
pub struct OnBoard;

#[derive(Clone)]
pub struct SpawnPawnData {
    pub persistent_player_data: PersistentPlayerData,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub inventory_setup: Vec<(String, String)>,
    pub designation: PawnDesignation,
}

pub fn connections(
    tick_rate: Res<TickRate>,
    mut auth_id_i: ResMut<AuthidI>,
    server_id: Res<ServerId>,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut commands: Commands,
    mut reader: EventReader<ServerEvent>,
    mut net_on_new_player_connection: EventWriter<NetPlayerConn>,
    mut connected_players: Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
        &mut Humanoid,
    )>,
    mut used_names: ResMut<UsedNames>,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
    gridmap_data: Res<GridmapData>,
    map_data: Res<MapData>,
    console_commands: Res<AllConsoleCommands>,
    give_all_rcon: Res<GiveAllRCON>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::ClientConnected(handle, _) => {
                info!("Incoming connection on [{}]", handle,);

                on_new_player_connection(
                    &mut net_on_new_player_connection,
                    handle,
                    &tick_rate,
                    &mut auth_id_i,
                    &server_id,
                    &mut handle_to_entity,
                    &mut commands,
                    &mut used_names,
                    &gridmap_data,
                    &map_data,
                    &console_commands,
                    &give_all_rcon,
                );
            }
            ServerEvent::ClientDisconnected(handle) => {
                on_player_disconnect(
                    *handle,
                    &mut handle_to_entity,
                    &mut connected_players,
                    &mut used_names,
                    &mut client_health_ui_cache,
                );
            }
        }
    }
}

pub fn on_player_disconnect(
    handle: u64,
    handle_to_entity: &mut ResMut<HandleToEntity>,
    connected_players: &mut Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
        &mut Humanoid,
    )>,
    used_names: &mut ResMut<UsedNames>,
    client_health_ui_cache: &mut ResMut<ClientHealthUICache>,
) {
    info!("[{}] disconnected!", handle);

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok((
                    mut persistent_player_data,
                    mut connected_player_component,
                    mut player_input_component,
                    mut standard_character_component,
                )) => {
                    standard_character_component.current_lower_animation_state =
                        CharacterAnimationState::Idle;
                    connected_player_component.connected = false;
                    player_input_component.movement_vector = Vec2::ZERO;
                    player_input_component.sprinting = false;
                    player_input_component.is_mouse_action_pressed = false;
                    player_input_component.auto_move_enabled = false;

                    // When reconnecting into the old pawn works remove this.
                    used_names
                        .user_names
                        .remove(&persistent_player_data.user_name);
                    persistent_player_data.user_name = "disconnectedUser".to_string();
                }
                Err(_rr) => {}
            }
        }
        None => {}
    }

    match entity {
        Some(ent) => {
            handle_to_entity.inv_map.remove(&ent);
            client_health_ui_cache.cache.remove(&ent);
        }
        None => {}
    }

    handle_to_entity.map.remove(&handle);
}

#[derive(Default)]
pub struct AuthidI {
    pub i: u16,
}

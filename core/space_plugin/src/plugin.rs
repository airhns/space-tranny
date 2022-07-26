use std::time::Duration;

use air_locks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use atmospherics::plugin::AtmosphericsPlugin;
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    asset::AssetPlugin,
    core::{CorePlugin, DefaultTaskPoolOptions},
    log::LogPlugin,
    prelude::{App, Plugin},
    render::{settings::WgpuSettings, RenderPlugin},
    scene::ScenePlugin,
    transform::TransformPlugin,
    window::WindowPlugin,
};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_renet::renet::NETCODE_KEY_BYTES;
use combat::plugin::CombatPlugin;
use computers::plugin::ComputersPlugin;
use connected_player::plugin::ConnectedPlayerPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool_admin::plugin::ConstructionToolAdminPlugin;
use counter_windows::plugin::CounterWindowsPlugin;
use entity::plugin::EntityPlugin;
use gridmap::plugin::GridmapPlugin;
use health::plugin::HealthPlugin;
use helmet_security::plugin::HelmetsPlugin;
use human_male::plugin::HumanMalePlugin;
use humanoid::plugin::HumanoidPlugin;
use inventory::plugin::InventoryPlugin;
use inventory_item::plugin::InventoryItemPlugin;
use jumpsuit_security::plugin::JumpsuitsPlugin;
use line_arrow::plugin::{LineArrowPlugin, PointArrowPlugin};
use map::plugin::MapPlugin;
use networking::plugin::NetworkingPlugin;
use omni_light::plugin::OmniLightPlugin;
use pawn::plugin::PawnPlugin;
use physics::plugin::PhysicsPlugin;
use pistol_l1::plugin::PistolL1Plugin;
use reflection_probe::plugin::ReflectionProbePlugin;
use rigid_body::plugin::RigidBodyPlugin;
use senser::plugin::SenserPlugin;
use sfx::plugin::SfxPlugin;
use api::{chat::MOTD, data::TickRate};
use sounds::SoundsPlugin;
use tab_actions::plugin::TabActionsPlugin;
use world_environment::plugin::WorldEnvironmentPlugin;

pub struct SpacePlugin {
    pub custom_motd: Option<String>,
    pub physics_rate: Option<u8>,
    pub bevy_rate: Option<u8>,
    pub threads_amount: Option<u8>,
    pub give_all_rcon: bool,
    pub custom_encryption_key: Option<[u8; NETCODE_KEY_BYTES]>,
}
impl Default for SpacePlugin {
    fn default() -> Self {
        Self {
            custom_motd: None,
            physics_rate: None,
            bevy_rate: None,
            // Dev values.
            threads_amount: Some(2),
            give_all_rcon: true,
            custom_encryption_key: None,
        }
    }
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugin(CorePlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(LogPlugin::default())
            .add_plugin(TransformPlugin::default())
            .insert_resource(wgpu_settings)
            .add_plugin(WindowPlugin {
                add_primary_window: false,
                exit_on_close: false,
            })
            .add_plugin(AssetPlugin)
            .add_plugin(ScenePlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(ConnectedPlayerPlugin {
                custom_motd: self.custom_motd.clone(),
            })
            .add_plugin(AsanaPlugin)
            .add_plugin(WorldEnvironmentPlugin)
            .add_plugin(GridmapPlugin)
            .add_plugin(PawnPlugin)
            .add_plugin(HumanMalePlugin)
            .add_plugin(SfxPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(EntityPlugin)
            .add_plugin(AtmosphericsPlugin)
            .add_plugin(ConsoleCommandsPlugin {
                give_all_rcon: self.give_all_rcon,
            })
            .add_plugin(ConstructionToolAdminPlugin)
            .add_plugin(TabActionsPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(AirLocksPlugin)
            .add_plugin(CounterWindowsPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(NetworkingPlugin {
                custom_encryption_key: self.custom_encryption_key,
            })
            .add_plugin(PhysicsPlugin)
            .add_plugin(HumanoidPlugin)
            .add_plugin(RigidBodyPlugin)
            .add_plugin(ComputersPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(OmniLightPlugin)
            .add_plugin(ReflectionProbePlugin)
            .add_plugin(InventoryItemPlugin)
            .add_plugin(SenserPlugin)
            .add_plugin(JumpsuitsPlugin)
            .add_plugin(HelmetsPlugin)
            .add_plugin(PistolL1Plugin)
            .add_plugin(LineArrowPlugin)
            .add_plugin(PointArrowPlugin)
            .add_plugin(SoundsPlugin);

        match self.threads_amount {
            Some(amn) => {
                app.insert_resource(DefaultTaskPoolOptions::with_num_threads(amn.into()));
            }
            None => {}
        }

        let mut tick_rate = TickRate::default();
        if self.physics_rate.is_some() {
            tick_rate.physics_rate = self.physics_rate.unwrap();
        }
        let mut bevy_rate = tick_rate.bevy_rate;
        if self.bevy_rate.is_some() {
            bevy_rate = self.bevy_rate.unwrap();
            tick_rate.bevy_rate = bevy_rate;
        }
        app.insert_resource::<TickRate>(tick_rate);
        app.insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_secs_f64(1. / (bevy_rate as f64))),
            },
        });

        match &self.custom_motd {
            Some(motd) => {
                app.insert_resource(MOTD {
                    message: motd.clone(),
                });
            }
            None => {
                app.init_resource::<MOTD>();
            }
        }
    }
}

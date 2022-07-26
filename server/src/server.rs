use crate::server_is_live;
use bevy::prelude::App;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use api::data::StartupLabels;
use space_plugin::plugin::SpacePlugin;

pub fn start_server() {
    App::new()
        .add_startup_system(
            server_is_live
                .label(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        )
        .add_plugin(SpacePlugin::default())
        .run();
}

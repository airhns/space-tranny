use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};
use entity::{
    entity_data::{initialize_entity_data, HELMET_SECURITY_ENTITY_NAME},
    spawn::{summon_base_entity, SpawnEvent},
};
use inventory_item::spawn::summon_inventory_item;
use rigid_body::spawn::summon_rigid_body;
use api::data::{EntityDataProperties, EntityDataResource, StartupLabels, SummoningLabels};

use super::spawn::{
    default_summon_helmet_security, summon_helmet, summon_raw_helmet, HelmetSummoner,
};

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(summon_helmet::<HelmetSummoner>.after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_base_entity::<HelmetSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_rigid_body::<HelmetSummoner>).after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_inventory_item::<HelmetSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_helmet).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<HelmetSummoner>>()
            .add_system(
                (default_summon_helmet_security)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: HELMET_SECURITY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}

use bevy::prelude::{Commands, Component, Entity};
use bevy_rapier3d::prelude::{CollisionGroups, Damping, GravityScale, Sleeping};
use api::data::Vec3Int;

pub fn get_bit_masks(group: ColliderGroup) -> (u32, u32) {
    match group {
        ColliderGroup::Standard => (
            //membership
            0b00000000000000000000000000000001,
            //filter
            0b00000000000000000000000000000001,
        ),
        ColliderGroup::NoCollision => (
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
        ),
    }
}

pub enum ColliderGroup {
    NoCollision,
    Standard,
}

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

#[derive(Component)]
pub struct RigidBodyDisabled;

pub fn disable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
    damping: &mut Damping,
) {
    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_flags.memberships = masks.0;
    collider_flags.filters = masks.1;

    gravity.0 = 0.;

    rigidbody_activation.sleeping = true;

    damping.angular_damping = 10000.;
    damping.linear_damping = 10000.;

    commands.entity(rigidbody_entity).insert(RigidBodyDisabled);
}

pub fn enable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
    damping: &mut Damping,
) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    collider_flags.memberships = masks.0;
    collider_flags.filters = masks.1;

    gravity.0 = 1.;

    rigidbody_activation.sleeping = false;

    damping.angular_damping = 0.;
    damping.linear_damping = 0.;

    commands
        .entity(rigidbody_entity)
        .remove::<RigidBodyDisabled>();
}

pub struct ReachResult {
    pub distance: f32,
    pub hit_entity: Option<(Entity, bool)>,
    pub hit_cell: Option<Vec3Int>,
}

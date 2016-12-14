//! The physics world.

pub use world::world::{
    RigidBodyId,
    SensorId,
    World,
    WorldBroadPhase,
    RigidBodies,
    RigidBodyCollisionWorld,
    WorldCollisionObject
};

mod world;

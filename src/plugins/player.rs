//! Defines common behaviors of players, including plants and zombies.

use super::land::{GridPos, LandPlants, LAND_SIZE};
use super::PLAYERS_Z;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::log::error;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use num::ToPrimitive;
use std::time::Instant;
use vleue_kinetoscope::{AnimatedImage, AnimatedImageController};

/// Components common for plants and zombies
#[derive(Debug, Clone, Component)]
#[require(Transform)]
pub struct PlayerCommon {
    pub name: &'static str,
    pub health: f32,
    pub spawned_time: Instant,
}

#[derive(Event)]
pub struct DieEvent;

impl PlayerCommon {
    pub fn new(name: &'static str, health: impl ToPrimitive) -> Self {
        PlayerCommon {
            name,
            health: health.to_f32().unwrap(),
            spawned_time: Instant::now(),
        }
    }

    pub fn damage(&mut self, commands: &mut Commands, amount: f32) {
        self.health -= amount;

        if self.health <= 0. {
            commands.trigger(DieEvent);
        }
    }
}

impl Default for PlayerCommon {
    fn default() -> Self {
        PlayerCommon {
            name: "Default",
            health: 100.,
            spawned_time: Instant::now(),
        }
    }
}

/// A resource storing textures for plants and zombies
#[derive(Resource)]
pub struct PlayerTextureResources {
    pub basic_zombie: Handle<AnimatedImage>,
    pub conehead_zombie: Handle<AnimatedImage>,
    pub eating_zombie: Handle<AnimatedImage>,
    pub eating_conehead_zombie: Handle<AnimatedImage>,
    pub sunflower: Handle<AnimatedImage>,
    pub sun: Handle<Image>,
    pub shooter: Handle<AnimatedImage>,
    pub shooter_bullet: Handle<Image>,
    pub wallnut: Handle<Image>,
}

impl PlayerTextureResources {
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(PlayerTextureResources {
            basic_zombie: asset_server.load("zombie.gif"),
            conehead_zombie: asset_server.load("conehead_zombie_moving.gif"),
            eating_zombie: asset_server.load("eating_zombie.gif"),
            eating_conehead_zombie: asset_server.load("ConeheadZombieAttack.gif"),
            sunflower: asset_server.load("SunFlower.gif"),
            sun: asset_server.load("Sun_transparent_background.png"),
            shooter: asset_server.load("PeaShooter.gif"),
            shooter_bullet: asset_server.load("pea.png"),
            wallnut: asset_server.load("Wall-nut1.png"),
        });
    }
}

pub fn dead_cleaner(
    trigger: Trigger<DieEvent>,
    mut commands: Commands,
    players: Query<(Entity, &mut PlayerCommon)>,
) {
    for (ent, pc) in players {
        if pc.health <= 0. {
            commands.get_entity(ent).unwrap().despawn();
        }
    }
}

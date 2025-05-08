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
    pub health: f32,
    pub spawned_time: Instant,
}

impl PlayerCommon {
    pub fn new(health: impl ToPrimitive) -> Self {
        PlayerCommon {
            health: health.to_f32().unwrap(),
            spawned_time: Instant::now(),
        }
    }

    pub fn damage(&mut self, amount: f32) {
        self.health -= amount;
    }
}

impl Default for PlayerCommon {
    fn default() -> Self {
        PlayerCommon {
            health: 100.,
            spawned_time: Instant::now(),
        }
    }
}

/// A resource storing textures for plants and zombies
#[derive(Resource)]
pub struct PlayerTextureResources {
    pub basic_zombie: Handle<AnimatedImage>,
    pub sunflower: Handle<AnimatedImage>,
    pub sun: Handle<Image>,
    pub shooter: Handle<AnimatedImage>,
    pub shooter_bullet: Handle<Image>,
}

impl PlayerTextureResources {
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(PlayerTextureResources {
            basic_zombie: asset_server.load("TheAdvancing_zombie.gif"),
            sunflower: asset_server.load("SunFlower.gif"),
            sun: asset_server.load("Sun.png"),
            shooter: asset_server.load("PeaShooter.gif"),
            shooter_bullet: asset_server.load("pea.png"),
        });
    }
}

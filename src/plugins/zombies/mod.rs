use crate::plugins::player::PlayerCommon;
use bevy::prelude::*;

pub mod basic_zombie;

#[derive(Component)]
#[require(PlayerCommon, Transform)]
pub struct ZombieCommon;

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {}
}

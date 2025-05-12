use super::{super::PLAYERS_Z, ZombieCommon};
use crate::plugins::{
    land::{LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    GridPos, PlayerTextureResources,
};
use bevy::log::info;
use bevy::prelude::*;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct ConeheadZombie;

impl ConeheadZombie {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: &Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new("Conehead Zombie", 200.),
            ZombieCommon::new(
                textures.conehead_zombie.clone(),
                textures.eating_conehead_zombie.clone(),
            ),
            ConeheadZombie,
            AnimatedImageController::play(textures.conehead_zombie.clone()),
            pos.to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

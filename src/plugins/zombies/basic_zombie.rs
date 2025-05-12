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
pub struct BasicZombie;

impl BasicZombie {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: &Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new("BasicZombie", 100.),
            ZombieCommon::new(
                textures.basic_zombie.clone(),
                textures.eating_zombie.clone(),
            ),
            BasicZombie,
            AnimatedImageController::play(textures.basic_zombie.clone()),
            pos.round()
                .to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y + 0.5),
        ));
    }
}

use super::{super::PLAYERS_Z, PlantCommon};
use crate::plugins::{
    land::{LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    GridPos, PlayerTextureResources, FLYING_Z,
};
use bevy::log::info;
use bevy::prelude::*;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct Sunflower;

impl Sunflower {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new(100.),
            PlantCommon,
            Sunflower,
            AnimatedImageController::play(textures.sunflower.clone()),
            pos.round()
                .to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

pub fn sunflow_gen_sun(
    time: Res<Time>,
    commands: &mut Commands,
    textures: Res<PlayerTextureResources>,
    sunflowers: Query<&PlayerCommon, With<Sunflower>>,
) {
    let x = time.elapsed().as_secs();

    for sunflower in sunflowers {
        if sunflower.spawned_time.elapsed().as_secs() % 20 == 0 {
            info!("Spawn sun now");
        }
    }
}

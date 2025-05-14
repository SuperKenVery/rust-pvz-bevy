use super::{super::PLAYERS_Z, PlantCommon};
use crate::plugins::{
    land::{LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    toolbar::{SunCount, SunCounter},
    GridPos, PlayerTextureResources, FLYING_Z,
};
use bevy::log::info;
use bevy::prelude::*;
use core::time::Duration;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct Wallnut;

impl Wallnut {
    pub fn create(
        pos: impl Into<GridPos>,
        commands: &mut Commands,
        textures: Res<PlayerTextureResources>,
    ) {
        let pos: GridPos = pos.into();
        commands.spawn((
            PlayerCommon::new("Wallnut", 250),
            Wallnut,
            Sprite::from_image(textures.wallnut.clone()),
            PlantCommon,
            pos.round()
                .to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

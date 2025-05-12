use super::{super::PLAYERS_Z, ZombieCommon, ZombieState};
use crate::plugins::{
    land::{LandPlants, LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    GridPos, PlayerTextureResources,
};
use bevy::log::info;
use bevy::prelude::*;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct JumpingZombie;

impl JumpingZombie {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: &Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new("Jumping Zombie", 100.),
            ZombieCommon::new(
                textures.jumping_zombie.clone(),
                textures.eating_zombie.clone(),
            ),
            JumpingZombie,
            AnimatedImageController::play(textures.jumping_zombie.clone()),
            pos.to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

/// For jumping zombie, it can jump over the first plant it meets.
pub fn jump_over_first_plant(
    zombies: Query<(&mut Transform, &ZombieCommon), (With<JumpingZombie>, Changed<ZombieCommon>)>,
    land_plants: Res<LandPlants>,
) {
    'iter_zombies: for (mut pos, zombie) in zombies {
        // We only do business when it's eating the first plant.
        // So if it's not eating, we do nothing.
        if zombie.state != ZombieState::Eating {
            continue;
        }

        // Now it's eating. Check if we're at the first plant.
        let grid_pos: GridPos = (*pos).into();
        let (zombie_x, zombie_y): (i32, i32) = grid_pos.into();
        // Check the tiles to the right of this zombie.
        // If all of them are empty, then we're at the first plant.
        '_iter_land_columns: for x in (zombie_x + 1)..(LAND_SIZE.x as i32) {
            if let Some(_plant) = land_plants.get((x, zombie_y)) {
                // It's not empty, so we're not at the first plant.
                // We should do nothing for this zombie and
                // go over to the next zombie.
                continue 'iter_zombies;
            }
        } // otherwise we're at the first plant

        // We are at the first plant!
        // Jump over it
        let new_pos = GridPos::new(zombie_x - 1, zombie_y);
        pos.translation = new_pos.to_world().extend(pos.translation.z);
    }
}

use super::{
    super::PLAYERS_Z, basic_zombie::BasicZombie, conehead_zombie::ConeheadZombie, ZombieCommon,
};
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

#[derive(Resource)]
pub struct ZombieCreateTimer {
    timer: Timer,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ZombieCreateTimer {
        timer: Timer::from_seconds(10., TimerMode::Repeating),
    });
}

#[derive(Debug)]
enum ZombieType {
    Basic,
    Conehead,
}

pub fn create_zombie_randomly(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ZombieCreateTimer>,
    textures: Res<PlayerTextureResources>,
) {
    if time.elapsed().as_secs() < 45 {
        return;
    }

    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        let zombie_type: &ZombieType =
            fastrand::choice(&[ZombieType::Basic, ZombieType::Conehead]).unwrap();
        let row = fastrand::i32(0..(LAND_SIZE.y as i32));
        let pos = GridPos::new(LAND_SIZE.x + 2., row);
        info!("Creating zombie {zombie_type:?} at {pos:?}");

        match zombie_type {
            ZombieType::Basic => BasicZombie::create(pos, &mut commands, &textures),
            ZombieType::Conehead => ConeheadZombie::create(pos, &mut commands, &textures),
        }
    }
}

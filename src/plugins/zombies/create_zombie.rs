use super::{
    super::PLAYERS_Z, basic_zombie::BasicZombie, conehead_zombie::ConeheadZombie, ZombieCommon,
};
use crate::{
    plugins::{
        land::{LAND_SIZE, LAND_TILE_SIZE},
        player::PlayerCommon,
        zombies::jumping_zombie::JumpingZombie,
        GridPos, PlayerTextureResources,
    },
    GameState,
};
use bevy::log::info;
use bevy::prelude::*;
use core::time::Duration;
use num::traits::real::Real;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[cfg(not(feature = "debug_mode"))]
const INITIAL_INTERVAL: f32 = 10.;
#[cfg(feature = "debug_mode")]
const INITIAL_INTERVAL: f32 = 3.;

#[derive(Resource)]
pub struct ZombieCreateTimer {
    timer: Timer,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ZombieCreateTimer {
        timer: Timer::from_seconds(INITIAL_INTERVAL, TimerMode::Repeating),
    });
}

#[derive(Debug)]
enum ZombieType {
    Basic,
    Conehead,
    Jumping,
}

#[cfg(feature = "debug_mode")]
const WIN_SECONDS: u64 = 60 * 1;
#[cfg(not(feature = "debug_mode"))]
const WIN_SECONDS: u64 = 60 * 10;

pub fn create_zombie_randomly(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ZombieCreateTimer>,
    textures: Res<PlayerTextureResources>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    #[cfg(not(feature = "debug_mode"))]
    if time.elapsed().as_secs() < 45 {
        return;
    }

    if time.elapsed().as_secs() > WIN_SECONDS {
        next_state.set(GameState::End { win: true });
    }

    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        let zombie_type: &ZombieType =
            fastrand::choice(&[ZombieType::Basic, ZombieType::Conehead, ZombieType::Jumping])
                .unwrap();
        let row = fastrand::i32(0..(LAND_SIZE.y as i32));
        let pos = GridPos::new(LAND_SIZE.x + 2., row);

        match zombie_type {
            ZombieType::Basic => BasicZombie::create(pos, &mut commands, &textures),
            ZombieType::Conehead => ConeheadZombie::create(pos, &mut commands, &textures),
            ZombieType::Jumping => JumpingZombie::create(pos, &mut commands, &textures),
        }

        let x = time.elapsed().as_secs_f32();
        let lower = ((1. / 50.) * x + 1.).powf(0.3);
        let new_duration = (1. / lower) * INITIAL_INTERVAL;
        timer
            .timer
            .set_duration(Duration::from_secs_f32(new_duration));
    }
}

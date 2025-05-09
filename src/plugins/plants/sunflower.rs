use super::{super::PLAYERS_Z, PlantCommon};
use crate::plugins::{
    land::{LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    toolbar::{SunCount, SunCounter},
    GridPos, PlayerTextureResources, FLYING_Z,
};
use bevy::log::info;
use bevy::prelude::*;
use std::time::Duration;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct Sunflower {
    produce_timer: Timer,
}

impl Sunflower {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new("Sunflower", 100.),
            PlantCommon,
            Sunflower {
                produce_timer: Timer::new(Duration::from_secs(15), TimerMode::Repeating),
            },
            AnimatedImageController::play(textures.sunflower.clone()),
            pos.round()
                .to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

#[derive(Component)]
pub struct Sun;

pub fn sunflow_gen_sun(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<PlayerTextureResources>,
    sunflowers: Query<(&mut Sunflower, &Transform)>,
) {
    for (mut sunflower, pos) in sunflowers {
        sunflower.produce_timer.tick(time.delta());
        if sunflower.produce_timer.finished() {
            commands
                .spawn((
                    Sun,
                    Sprite::from_image(textures.sun.clone()),
                    pos.clone(),
                    Pickable::default(),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Click>>,
                     mut commands: Commands,
                     mut sun_count: ResMut<SunCount>| {
                        // The sun is clicked, collect it
                        let mut sun = commands.entity(trigger.target());
                        sun.despawn();
                        sun_count.0 += 50;
                    },
                );
        }
    }
}

pub fn sun_go_up(time: Res<Time>, mut commands: Commands, suns: Query<&mut Transform, With<Sun>>) {
    for mut sun in suns {
        sun.translation.y += time.delta().as_millis() as f32 / 100.;
    }
}

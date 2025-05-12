use super::{super::PLAYERS_Z, PlantCommon};
use crate::{
    plugins::{
        land::{LAND_SIZE, LAND_TILE_SIZE},
        player::PlayerCommon,
        toolbar::{SunCount, SunCounter},
        GridPos, PlayerTextureResources, FLYING_Z,
    },
    Dying, SCREEN_RESOLUTION,
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
    pub fn create(
        pos: impl Into<GridPos>,
        commands: &mut Commands,
        textures: Res<PlayerTextureResources>,
    ) {
        let pos: GridPos = pos.into();
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
pub struct Sun {
    move_up: bool,
}

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
                    Sun { move_up: true },
                    Sprite::from_image(textures.sun.clone()),
                    pos.clone(),
                    Pickable::default(),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Click>>,
                     mut commands: Commands,
                     mut sun_count: ResMut<SunCount>| {
                        // The sun is clicked, collect it
                        commands.entity(trigger.target()).insert(Dying);
                        sun_count.0 += 50;
                    },
                );
        }
    }
}

pub fn move_sun(
    time: Res<Time>,
    mut commands: Commands,
    suns: Query<(Entity, &mut Transform, &Sun)>,
) {
    for (entity, mut sun_pos, dir) in suns {
        let direction = match dir.move_up {
            true => 1.,
            false => -1.,
        };
        sun_pos.translation.y += time.delta().as_millis() as f32 / 100. * direction;

        if sun_pos.translation.y >= SCREEN_RESOLUTION.y + 80. / 2. {
            commands.entity(entity).insert(Dying);
        }
    }
}

/// A resource containing a timer for the global random sun
#[derive(Resource)]
pub struct GlobalSunTimer(pub Timer);

pub fn init_global_sun_res(mut commands: Commands) {
    commands.insert_resource(GlobalSunTimer(Timer::from_seconds(
        #[cfg]
        15.,
        TimerMode::Repeating,
    )));
}

pub fn gen_sun_from_sky(
    mut commands: Commands,
    mut gstimer: ResMut<GlobalSunTimer>,
    time: Res<Time>,
    textures: Res<PlayerTextureResources>,
) {
    gstimer.0.tick(time.delta());

    if gstimer.0.finished() {
        let pos = GridPos::new(fastrand::f32() * LAND_SIZE.x, LAND_SIZE.y + 2.);
        commands
            .spawn((
                Sun { move_up: false },
                Sprite::from_image(textures.sun.clone()),
                pos.to_world_transform(FLYING_Z),
                Pickable::default(),
            ))
            .observe(
                |trigger: Trigger<Pointer<Click>>,
                 mut commands: Commands,
                 mut sun_count: ResMut<SunCount>| {
                    // The sun is clicked, collect it
                    commands.entity(trigger.target()).insert(Dying);
                    sun_count.0 += 50;
                },
            );
    }
}

use super::{super::PLAYERS_Z, PlantCommon};
use crate::plugins::{
    land::{LAND_SIZE, LAND_TILE_SIZE},
    player::PlayerCommon,
    zombies::ZombieCommon,
    GridPos, PlayerTextureResources, FLYING_Z,
};
use bevy::log::info;
use bevy::prelude::*;
use std::time::Duration;
use vleue_kinetoscope::{
    AnimatedImage, AnimatedImageController, AnimatedImagePlugin, AnimationPlayed,
};

#[derive(Component)]
pub struct Peashooter {
    shoot_timer: Timer,
}

impl Peashooter {
    pub fn create(pos: GridPos, commands: &mut Commands, textures: Res<PlayerTextureResources>) {
        commands.spawn((
            PlayerCommon::new("Peashooter", 100.),
            PlantCommon,
            Peashooter {
                shoot_timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            },
            AnimatedImageController::play(textures.shooter.clone()),
            pos.round()
                .to_world_transform(PLAYERS_Z + LAND_SIZE.y - pos.y),
        ));
    }
}

#[derive(Component)]
pub struct Bullet;

pub fn peashooter_shoot(
    time: Res<Time>,
    mut commands: Commands,
    pea_shooters: Query<(&Transform, &mut Peashooter), With<Peashooter>>,
    textures: Res<PlayerTextureResources>,
) {
    for (transform, mut shooter) in pea_shooters {
        shooter.shoot_timer.tick(time.delta());
        if shooter.shoot_timer.finished() {
            commands.spawn((
                Bullet,
                Sprite::from_image(textures.shooter_bullet.clone()),
                transform.clone(),
            ));
        }
    }
}

pub fn peashooter_bullet_move(time: Res<Time>, bullets: Query<&mut Transform, With<Bullet>>) {
    for mut bullet in bullets {
        bullet.translation.x += time.delta().as_millis() as f32 / 1.5;
    }
}

pub fn peashooter_bullet_collide(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut zombies: Query<(&Transform, &mut PlayerCommon), With<ZombieCommon>>,
) {
    for (ent, bullet) in bullets {
        for (zombie, mut player) in &mut zombies {
            if (bullet.translation - zombie.translation).length() < 5. {
                player.damage(&mut commands, 10.);
                let Ok(mut entity) = commands.get_entity(ent) else {
                    error!("Failed to get entity when removing collided bullet");
                    continue;
                };
                entity.despawn();
            }
        }
    }
}

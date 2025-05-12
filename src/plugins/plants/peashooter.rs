use super::{super::PLAYERS_Z, PlantCommon};
use crate::{
    plugins::{
        land::{LAND_SIZE, LAND_TILE_SIZE},
        player::PlayerCommon,
        zombies::{LandZombies, ZombieCommon},
        GridPos, PlayerTextureResources, FLYING_Z,
    },
    SCREEN_RESOLUTION,
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
    land_zombies: Res<LandZombies>,
    transform_query: Query<&Transform, With<ZombieCommon>>,
) {
    for (transform, mut shooter) in pea_shooters {
        shooter.shoot_timer.tick(time.delta());
        if !shooter.shoot_timer.finished() {
            continue;
        }

        let shooter_pos: GridPos = (*transform).into();
        if land_zombies.is_empty(shooter_pos.round().y as usize, transform_query) {
            continue;
        }
        commands.spawn((
            Bullet,
            Sprite::from_image(textures.shooter_bullet.clone()),
            transform.clone(),
        ));
    }
}

pub fn peashooter_bullet_move(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    zombie_pos: Query<&Transform, (With<ZombieCommon>, Without<Bullet>)>,
    mut zombie_health: Query<&mut PlayerCommon, With<ZombieCommon>>,
    land_zombies: Res<LandZombies>,
) {
    debug!("peashooter_bullet_move");
    for (entity, mut bullet_pos) in bullets {
        // Move right
        bullet_pos.translation.x += time.delta().as_millis() as f32 / 1.5;

        // Check whether it's out of screen
        if bullet_pos.translation.x >= SCREEN_RESOLUTION.x + 28. / 2. {
            commands.get_entity(entity).unwrap().despawn();
            continue;
        }

        // Check whether it has collided with a zombie
        let bullet_grid_pos: GridPos = GridPos::from(*bullet_pos).round();
        let row_zombies = &land_zombies.rows[bullet_grid_pos.y as usize];
        debug!("moving bullet: {} zombies in this row", row_zombies.len());
        for zombie in row_zombies {
            if (bullet_pos.translation - zombie_pos.get(*zombie).unwrap().translation).length() < 5.
            {
                let mut health = zombie_health.get_mut(*zombie).unwrap();
                health.damage(&mut commands, 10.);
                commands.get_entity(entity).unwrap().despawn();
            }
        }
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
                commands.get_entity(ent).unwrap().despawn();
            }
        }
    }
}

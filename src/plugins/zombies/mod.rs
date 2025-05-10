use super::{
    land::{LandPlants, LAND_SIZE},
    plants::PlantCommon,
    GridPos,
};
use crate::plugins::player::PlayerCommon;
use bevy::log::{debug, info};
use bevy::{
    ecs::{component::HookContext, entity::EntityEquivalent, world::DeferredWorld},
    prelude::*,
};
use vleue_kinetoscope::{AnimatedImage, AnimatedImageController};

pub mod basic_zombie;
pub mod conehead_zombie;
pub mod create_zombie;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ZombieState {
    Walking,
    Eating,
}

#[derive(Component)]
#[require(PlayerCommon, Transform)]
#[component(on_add=zombie_on_add, on_remove=zombie_on_remove)]
pub struct ZombieCommon {
    pub walking: Handle<AnimatedImage>,
    pub eating: Handle<AnimatedImage>,
    pub state: ZombieState,
}

impl ZombieCommon {
    pub fn new(walking: Handle<AnimatedImage>, eating: Handle<AnimatedImage>) -> Self {
        ZombieCommon {
            walking,
            eating,
            state: ZombieState::Walking,
        }
    }
}

/// Add the zombie to LandZombies
fn zombie_on_add<'w>(mut world: DeferredWorld<'w>, context: HookContext) {
    let transform = world.get::<Transform>(context.entity).unwrap();
    let grid_pos: GridPos = (*transform).into();
    let mut land_zombie = world.get_resource_mut::<LandZombies>().unwrap();
    land_zombie.add_zombie(context.entity, grid_pos.round().y as usize);
}

/// Remove the zombie from LandZombies
fn zombie_on_remove<'w>(mut world: DeferredWorld<'w>, context: HookContext) {
    let transform = world.get::<Transform>(context.entity).unwrap();
    let grid_pos: GridPos = (*transform).into();
    let mut land_zombie = world.get_resource_mut::<LandZombies>().unwrap();
    land_zombie.remove_zombie(context.entity, grid_pos.round().y as usize);
}

/// Move zombies forward
fn move_zombies(
    mut commands: Commands,
    time: Res<Time>,
    zombies: Query<(&mut Transform, &mut ZombieCommon)>,
    mut health: Query<&mut PlayerCommon, With<PlantCommon>>,
    land_plants: Res<LandPlants>,
) {
    for (mut position, mut common) in zombies {
        let grid_pos: GridPos = (*position).into();
        let new_state = if let Some(target) = land_plants.get(grid_pos) {
            // We've got a plant here, eat it
            let mut player = health.get_mut(*target).unwrap();
            player.damage(&mut commands, time.delta().as_millis() as f32 / 100.);
            ZombieState::Eating
        } else {
            // No plant here, move forward
            position.translation.x -= time.delta().as_millis() as f32 / 100.;
            ZombieState::Walking
        };

        if new_state != common.state {
            common.state = new_state;
        }
    }
}

/// Update zombies' animation based on state (eating or walking)
fn update_zombie_animation(
    mut commands: Commands,
    changed_zombies: Query<(Entity, &ZombieCommon), Changed<ZombieCommon>>,
) {
    for (entity, common) in changed_zombies {
        commands.entity(entity).remove::<AnimatedImageController>();
        commands
            .entity(entity)
            .insert(AnimatedImageController::play(match common.state {
                ZombieState::Walking => common.walking.clone(),
                ZombieState::Eating => common.eating.clone(),
            }));
    }
}

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_landzombies);
        app.add_systems(Startup, create_zombie::setup);
        app.add_systems(
            Update,
            (
                create_zombie::create_zombie_randomly,
                move_zombies,
                update_zombie_animation,
            ),
        );
    }
}

/// A resource recording zombies on each row of land
#[derive(Resource)]
pub struct LandZombies {
    rows: Vec<Vec<Entity>>,
}

impl Default for LandZombies {
    fn default() -> Self {
        LandZombies {
            rows: (0..LAND_SIZE.y as i32).map(|_| vec![]).collect(),
        }
    }
}

fn setup_landzombies(mut commands: Commands) {
    commands.insert_resource(LandZombies::default());
}

impl LandZombies {
    pub fn add_zombie(&mut self, entity: Entity, row: usize) {
        self.rows[row].push(entity)
    }

    pub fn is_empty(
        &self,
        row: usize,
        transform_query: Query<&Transform, With<ZombieCommon>>,
    ) -> bool {
        // Check if the zombies are actually in land
        // (they could be out of bound)
        for zombie in &self.rows[row] {
            let transform = transform_query.get(*zombie).unwrap();
            let pos: GridPos = (*transform).into();
            if pos.in_land() {
                return false;
            }
        }

        return true;
    }

    pub fn remove_zombie(&mut self, entity: Entity, row: usize) {
        if let Some(idx) = self.rows[row].iter().position(|x| *x == entity) {
            self.rows[row].remove(idx);
        }
    }
}

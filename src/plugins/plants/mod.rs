pub mod peashooter;
pub mod sunflower;
pub mod wallnut;

use super::{land::LandPlants, GridPos};
use crate::plugins::player::PlayerCommon;
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use peashooter::{peashooter_bullet_collide, peashooter_bullet_move, peashooter_shoot};
use sunflower::{sun_go_up, sunflow_gen_sun};

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sun_go_up,
                sunflow_gen_sun,
                peashooter_shoot,
                peashooter_bullet_move,
                peashooter_bullet_collide,
            ),
        );
    }
}

#[derive(Component)]
#[component(on_add=plant_comm_on_add, on_remove=plant_comm_on_remove)]
#[require(PlayerCommon, Transform)]
pub struct PlantCommon;

fn plant_comm_on_add<'w>(mut world: DeferredWorld<'w>, context: HookContext) {
    let transform = world.get::<Transform>(context.entity).unwrap();
    let grid_pos: GridPos = (*transform).into();
    let mut map = world.resource_mut::<LandPlants>();

    if map.is_empty(grid_pos) == false {
        error!("PlantCommon added to a non-empty land tile!");
    }
    map.add(grid_pos, context.entity);
}

fn plant_comm_on_remove<'w>(mut world: DeferredWorld<'w>, context: HookContext) {
    let transform = world.get::<Transform>(context.entity).unwrap();
    let grid_pos: GridPos = (*transform).into();
    let mut map = world.resource_mut::<LandPlants>();

    if map.is_empty(grid_pos) == true {
        error!("PlantCommon removed from an empty land tile!");
    }
    map.remove(grid_pos);
}

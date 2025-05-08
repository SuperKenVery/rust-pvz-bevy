pub mod sunflower;

use super::{land::LandPlants, GridPos};
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component)]
#[component(on_add=plant_comm_on_add)]
struct PlantCommon;

fn plant_comm_on_add<'w>(mut world: DeferredWorld<'w>, context: HookContext) {
    let transform = world.get::<Transform>(context.entity).unwrap();
    let grid_pos: GridPos = (*transform).into();
    let mut map = world.resource_mut::<LandPlants>();

    if map.is_empty(grid_pos) == false {
        error!("PlantCommon added to a non-empty land tile!");
    }
    map.add(grid_pos, context.entity);
}

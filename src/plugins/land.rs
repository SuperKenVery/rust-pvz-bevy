use bevy::prelude::*;
use bevy::{
    log::{debug, info},
    platform::collections::HashMap,
};
use num::{traits::real::Real, Num, ToPrimitive};

pub const LAND_DISPLAY_OFFSET: Vec2 = Vec2::new(70.0, 0.0);

pub struct LandPlugin;
impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_land);
        // app.add_systems(Startup, GridPos::debug_offsets);
    }
}

#[derive(Component)]
struct Land;

fn add_land(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Land,
        Sprite {
            image: asset_server.load("Background1.png"),
            ..default()
        },
        Transform::from_xyz(LAND_DISPLAY_OFFSET.x, LAND_DISPLAY_OFFSET.y, -1.0),
    ));
}

pub const LAND_TILE_SIZE: Vec2 = Vec2::new(80.0, 100.0);
/// Offset from land image center to bottom-left corner of land tiles.
pub const LAND_TO_TILE_OFFSET: Vec2 = Vec2::new(-445., -275.);
/// How many tiles are there in the land.
pub const LAND_SIZE: Vec2 = Vec2::new(9., 5.);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridPos {
    pub x: f32,
    pub y: f32,
}

impl GridPos {
    pub fn new(x: impl ToPrimitive, y: impl ToPrimitive) -> Self {
        Self {
            x: x.to_f32().unwrap(),
            y: y.to_f32().unwrap(),
        }
    }

    /// Converts grid position to world position (center of tile)
    pub fn to_world(&self) -> Vec2 {
        self.clone().into()
    }

    pub fn to_world_transform(&self, z: impl ToPrimitive) -> Transform {
        let world_pos = self.to_world();
        Transform::from_xyz(world_pos.x, world_pos.y, z.to_f32().unwrap())
    }

    pub fn round(&self) -> Self {
        GridPos::new(self.x.round(), self.y.round())
    }

    fn debug_offsets(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let horizontal = meshes.add(Rectangle::new(LAND_TILE_SIZE.x, 1.));
        let vertical = meshes.add(Rectangle::new(1., LAND_TILE_SIZE.y));
        let material = materials.add(Color::srgba(1., 0., 0., 1.));

        for x in 0..LAND_SIZE.x as usize {
            for y in 0..LAND_SIZE.y as usize {
                let pos = GridPos::new(x as f32, y as f32);
                let world_pos = pos.to_world();
                commands.spawn((
                    Mesh2d(horizontal.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(world_pos.x, world_pos.y + LAND_TILE_SIZE.y / 2., 1.),
                )); // Top
                commands.spawn((
                    Mesh2d(horizontal.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(world_pos.x, world_pos.y - LAND_TILE_SIZE.y / 2., 1.),
                )); // Bottom
                commands.spawn((
                    Mesh2d(vertical.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(world_pos.x - LAND_TILE_SIZE.x / 2., world_pos.y, 1.),
                )); // Left
                commands.spawn((
                    Mesh2d(vertical.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(world_pos.x + LAND_TILE_SIZE.x / 2., world_pos.y, 1.),
                )); // Right
            }
        }
    }
}

impl Into<Vec2> for GridPos {
    /// Converts grid position to world position (center of tile)
    fn into(self) -> Vec2 {
        let bottom_left = LAND_DISPLAY_OFFSET + LAND_TO_TILE_OFFSET;
        let bottom_left_tile_center = bottom_left + LAND_TILE_SIZE / 2.;
        let offset = LAND_TILE_SIZE * Vec2::new(self.x, self.y);

        bottom_left_tile_center + offset
    }
}

impl From<Transform> for GridPos {
    /// Converts world position (transform.translation) to grid pos
    fn from(value: Transform) -> Self {
        let pos = value.translation.xy();

        pos.into()
    }
}

impl From<Vec2> for GridPos {
    fn from(value: Vec2) -> Self {
        let first: Vec2 = GridPos::new(0, 0).into();
        let distance = value - first;
        let grid_dist = distance / LAND_TILE_SIZE;

        Self::new(grid_dist.x, grid_dist.y)
    }
}

/// Map index storing plants on each tile
#[derive(Resource)]
pub struct LandPlants {
    pub tiles: HashMap<(i32, i32), Option<Entity>>,
}

impl LandPlants {
    pub fn add(&mut self, pos: GridPos, entity: Entity) {
        debug!("Adding plant to land tile {pos:#?}");
        self.tiles.insert(pos.into(), Some(entity));
    }

    pub fn is_empty(&self, pos: GridPos) -> bool {
        let key: (i32, i32) = pos.into();
        // debug!("At {pos:?} we have {:#?} ", self.tiles.get(&key));
        !self.tiles.get(&key).is_some()
    }

    pub fn remove(&mut self, pos: GridPos) {
        let key: (i32, i32) = pos.into();
        self.tiles.get(&key).take();
    }
}

impl Default for LandPlants {
    fn default() -> Self {
        let mut map = HashMap::new();

        for x in 0..LAND_SIZE.x as i32 {
            for y in 0..LAND_SIZE.y as i32 {
                map.insert((x, y), None);
            }
        }

        Self { tiles: map }
    }
}

impl From<GridPos> for (i32, i32) {
    fn from(value: GridPos) -> Self {
        let pos = value.round();
        (pos.x as i32, pos.y as i32)
    }
}

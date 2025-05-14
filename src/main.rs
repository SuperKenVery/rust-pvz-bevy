use bevy::log::DEFAULT_FILTER;
use bevy::window::WindowResolution;
use bevy::{log::LogPlugin, prelude::*};
use vleue_kinetoscope::AnimatedImagePlugin;
mod plugins;
use plugins::{plants, toolbar, GridPos, PlayerTextureResources};

pub const SCREEN_RESOLUTION: Vec2 = Vec2::new(800., 600.);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Plant vs Zombies".into(),
                        // resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.0),
                        resolution: SCREEN_RESOLUTION.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "warn,Plant_vs_Zombies=trace".into(),
                    ..default()
                }),
        )
        .add_plugins(AnimatedImagePlugin)
        .add_plugins(plugins::land::LandPlugin)
        .add_plugins(plugins::zombies::ZombiePlugin)
        .add_plugins(plugins::plants::PlantPlugin)
        .add_plugins(toolbar::ToolbarPlugin)
        .add_systems(Startup, setup)
        .add_systems(PreStartup, PlayerTextureResources::setup)
        .add_systems(Startup, debug_setup)
        .add_systems(PostUpdate, remove_dying)
        .add_observer(plugins::player::dead_cleaner)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn debug_setup(mut commands: Commands, textures: Res<PlayerTextureResources>) {}

/// A component that marks an entity as dying
///
/// It will be despawned at PostUpdate
#[derive(Component)]
pub struct Dying;

fn remove_dying(mut commands: Commands, dying_entities: Query<Entity, With<Dying>>) {
    for entity in dying_entities {
        commands.entity(entity).despawn();
    }
}

use crate::{Dying, GameState};
use bevy::log::info;
use bevy::prelude::*;

pub struct StartScreen;

#[derive(Component)]
struct StartScreenComponent;

impl Plugin for StartScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::WaitForStart), setup_start_menu);
    }
}

fn setup_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        StartScreenComponent,
        Sprite::from_image(asset_server.load("savedgames.png")),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands
        .spawn((
            StartScreenComponent,
            Sprite::from_image(asset_server.load("pause_background.png")),
            Pickable::default(),
            Transform::from_scale(Vec3::ONE * 0.2),
        ))
        .with_child((Text2d::new("Start"), Transform::from_scale(Vec3::ONE * 5.0)))
        .observe(start_clicked);
}

fn start_clicked(
    _trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    start_screen_components: Query<Entity, With<StartScreenComponent>>,
) {
    next_state.set(GameState::Running);

    for ent in start_screen_components {
        commands.entity(ent).despawn();
    }
}

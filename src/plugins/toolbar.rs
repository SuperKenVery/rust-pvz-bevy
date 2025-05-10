use super::{land::LandPlants, PlayerTextureResources, TOOLBAR_Z};
use crate::plugins::{
    plants::{peashooter::Peashooter, sunflower::Sunflower},
    FLOATING_Z, FLYING_Z,
};
use bevy::{ecs::system::IntoObserverSystem, text::TextBounds};
use bevy::{prelude::*, text::cosmic_text::ttf_parser::Style};
use std::dbg;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PreStartup, ToolbarTextureResource::setup);
        app.add_systems(
            Update,
            (
                follow_mouse,
                sun_changed.run_if(resource_exists_and_changed::<SunCount>),
            ),
        );
    }
}

#[derive(Resource)]
struct ToolbarTextureResource {
    pub counter: Handle<Image>,
    pub sunflower_card: Handle<Image>,
    pub wallnut_card: Handle<Image>,
    pub peashooter_card: Handle<Image>,
    pub cherrybomb_card: Handle<Image>,
}

impl ToolbarTextureResource {
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(ToolbarTextureResource {
            counter: asset_server.load("Counter.png"),
            sunflower_card: asset_server.load("SunflowerCard.PNG"),
            wallnut_card: asset_server.load("WallNutCard.png"),
            peashooter_card: asset_server.load("PeashooterCard.PNG"),
            cherrybomb_card: asset_server.load("CherryBombCard.png"),
        })
    }
}

/// Marker component for the sun counter
#[derive(Component)]
pub struct SunCounter;

/// Marker component for a button (a plant) in the toolbar
#[derive(Component)]
pub struct ToolbarPlant {
    pub price: i32,
}

/// Any entity with this component will be positionsed
/// where the mouse pointer is
#[derive(Component)]
#[require(Transform)]
struct FollowMouse;

/// The **Resource** representing how many suns we
/// currently have. Used to buy plants.
#[derive(Resource)]
pub struct SunCount(pub i32);

fn setup(mut commands: Commands, textures: Res<ToolbarTextureResource>, sun_count: Res<SunCount>) {
    let counter_transform = Transform::from_xyz(-400. + 163. / 2., 300. - 48. / 2., TOOLBAR_Z);
    commands.spawn((
        SunCounter,
        Sprite::from_image(textures.counter.clone()),
        counter_transform.clone(),
        Text2d::new(format!("{}", sun_count.0)),
        TextColor::BLACK,
    ));

    let left = -400. + 163.;
    const WIDTH: f32 = 110.;
    let mut x = left + WIDTH / 2.;

    add_toolbar_item(
        &mut commands,
        &mut x,
        textures.sunflower_card.clone(),
        50,
        |mouse_pos: Vec2, commands: &mut Commands, textures: Res<PlayerTextureResources>| {
            info!("Planting a sunflower at {mouse_pos}");
            Sunflower::create(mouse_pos.into(), commands, textures);
        },
    );

    add_toolbar_item(
        &mut commands,
        &mut x,
        textures.peashooter_card.clone(),
        100,
        |mouse_pos: Vec2, commands: &mut Commands, textures: Res<PlayerTextureResources>| {
            Peashooter::create(mouse_pos.into(), commands, textures);
        },
    );
}

fn add_toolbar_item(
    commands: &mut Commands,
    x: &mut f32,
    card_texture: Handle<Image>,
    price: i32,
    plant_fn: impl Fn(Vec2, &mut Commands, Res<PlayerTextureResources>) -> ()
        + Sync
        + Send
        + 'static
        + Clone,
) {
    const HEIGHT: f32 = 70.;
    const WIDTH: f32 = 110.;
    let y = 300. - HEIGHT / 2.;

    commands
        .spawn((
            ToolbarPlant { price },
            Sprite {
                image: card_texture.clone(),
                color: Color::linear_rgb(0.5, 0.5, 0.5),
                ..default()
            },
            Transform::from_xyz(*x, y, TOOLBAR_Z),
            Pickable::default(),
        ))
        .observe(tb_gen_observer(card_texture.clone(), plant_fn));
    *x += WIDTH;
}

/// # Toolbar generate observer
/// Generates an observer for a toolbar button (e.g. a sunflower).
///
/// What it does:
/// - Creates a floating card when clicked
/// - When clicked again
///     - If empty, plant the plant
///     - Remove the floating card
///
/// ## Arguments
/// - float_image: The image to show in the floating widget that follows
/// the mouse when toolbar button (e.g. sunflower) is clicked
/// - plant_fn: A closure that actually puts the plant onto the land.
/// Only called when it's empty there.
fn tb_gen_observer(
    float_image: Handle<Image>,
    plant_fn: impl Fn(Vec2, &mut Commands, Res<PlayerTextureResources>) -> ()
        + Sync
        + Send
        + 'static
        + Clone,
) -> impl Fn(
    Trigger<Pointer<Click>>,
    Commands,
    Query<&ToolbarPlant>,
    Single<(&Camera, &GlobalTransform)>,
    Res<SunCount>,
) {
    // The observer for toolbar click
    move |trigger: Trigger<Pointer<Click>>,
          mut commands: Commands,
          plant: Query<&ToolbarPlant>,
          camera: Single<(&Camera, &GlobalTransform)>,
          sun_count: Res<SunCount>| {
        let event = trigger.event();
        let mouse_pos_raw = event.pointer_location.clone();
        let (camera, camera_transform) = *camera;
        let mouse_pos = camera
            .viewport_to_world(camera_transform, mouse_pos_raw.position)
            .unwrap()
            .origin
            .truncate();

        let toolbar_plant = plant.get(trigger.target()).unwrap();
        let price = toolbar_plant.price;
        if sun_count.0 < price {
            return;
        }

        let cloned_plant_fn = plant_fn.clone();
        // Spawn the floating widget
        commands
            .spawn((
                Sprite::from_image(float_image.clone()),
                Transform::from_xyz(mouse_pos.x, mouse_pos.y, FLOATING_Z),
                Pickable::default(),
                FollowMouse,
            ))
            .observe(
                // Observer for the floating widget
                move |trigger: Trigger<Pointer<Click>>,
                      mut commands: Commands,
                      textures: Res<PlayerTextureResources>,
                      camera: Single<(&Camera, &GlobalTransform)>,
                      map: Res<LandPlants>,
                      mut sun_count: ResMut<SunCount>| {
                    let event = trigger.event();
                    let mouse_pos_raw = event.pointer_location.clone();
                    let (camera, camera_transform) = *camera;
                    let mouse_pos = camera
                        .viewport_to_world(camera_transform, mouse_pos_raw.position)
                        .unwrap()
                        .origin
                        .truncate();

                    info!("Clicked again, planting");
                    if map.is_empty(mouse_pos.into()) {
                        cloned_plant_fn(mouse_pos, &mut commands, textures);
                        sun_count.0 -= price;
                    } else {
                        warn!("Not planting because it's not empty")
                    }
                    if let Ok(mut entity) = commands.get_entity(trigger.target()) {
                        entity.despawn();
                    }
                },
            );
    }
}

fn follow_mouse(
    followers: Query<&mut Transform, With<FollowMouse>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if followers.is_empty() {
        return;
    }
    let (camera, camera_transform) = *camera;
    let Some(mouse_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    for mut pos in followers {
        pos.translation.x = mouse_pos.x;
        pos.translation.y = mouse_pos.y;
    }
}

fn sun_changed(
    mut counter: Single<&mut Text2d, With<SunCounter>>,
    sun_count: Res<SunCount>,
    toolbar_plants: Query<(&mut Sprite, &ToolbarPlant)>,
) {
    let current_suns = sun_count.0;
    counter.0 = format!("{}", current_suns);

    for (mut button, plant) in toolbar_plants {
        if current_suns >= plant.price {
            // Make it not grey
            button.color = Color::WHITE;
        } else {
            // Make it grey
            button.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}

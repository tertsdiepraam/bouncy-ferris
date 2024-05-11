//! Renders a 2D scene containing a single, moving sprite.

use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<FerrisMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, sprite_movement))
        .run();
}

#[derive(Component)]
struct Bouncy {
    x: i8,
    y: i8,
    img: u8,
    imgs: Vec<Handle<Image>>,
    scale: f32,
    moving: bool,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FerrisMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });

    let imgs = vec![
        asset_server.load("ferris.png"),
        asset_server.load("corro.png"),
    ];

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(FerrisMaterial {
                rotation: 0,
                color_texture: imgs[0].clone(),
                total_colors: 5,
            }),
            ..default()
        },
        Bouncy {
            x: 1,
            y: 1,
            scale: 0.2,
            img: 0,
            imgs,
            moving: false,
        },
    ));
}

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<FerrisMaterial>>,
    mut sprite_position: Query<(&mut Bouncy, &Handle<FerrisMaterial>)>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for ev in mouse_wheel_events.read() {
        for (mut bouncy, _) in &mut sprite_position {
            bouncy.scale = (bouncy.scale + (ev.y / 30.0)).clamp(0.1, 0.5);
        }
    }

    if keys.just_pressed(KeyCode::KeyQ) {
        for (mut bouncy, _mat) in &mut sprite_position {
            bouncy.moving = !bouncy.moving;
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        for (mut bouncy, mat) in &mut sprite_position {
            bouncy.img = (bouncy.img + 1) % bouncy.imgs.len() as u8;
            let Some(mat) = materials.get_mut(mat) else {
                continue;
            };

            mat.color_texture = bouncy.imgs[bouncy.img as usize].clone();
        }
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    windows: Query<&Window>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<FerrisMaterial>>,
    mut sprite_position: Query<(&mut Bouncy, &mut Transform, &Handle<FerrisMaterial>)>,
) {
    let window = windows.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let mut rng = thread_rng();

    for (mut bouncy, mut transform, mat) in &mut sprite_position {
        if !bouncy.moving {
            continue;
        }

        let Some(mat) = materials.get_mut(mat) else {
            continue;
        };

        let Some(img) = images.get(&mat.color_texture) else {
            continue;
        };

        let img_size = img.size_f32();

        let scale = width.min(height) * bouncy.scale;
        let x_size = scale * 1.0f32.min(img_size.x / img_size.y);
        let y_size = scale * 1.0f32.min(img_size.y / img_size.x);

        transform.scale = Vec3::new(x_size, y_size, 1.0);

        let x_range = (width - x_size) / 2.0;
        let y_range = (height - y_size) / 2.0;

        transform.translation.x += 150.0 * time.delta_seconds() * bouncy.x as f32;
        transform.translation.y += 150.0 * time.delta_seconds() * bouncy.y as f32;

        let total_colors = mat.total_colors;

        if transform.translation.x > x_range {
            bouncy.x = -1;
            transform.translation.x = x_range;
            mat.rotation = (mat.rotation + rng.gen_range(1..total_colors - 1)) % total_colors;
        }

        if transform.translation.x < -x_range {
            bouncy.x = 1;
            transform.translation.x = -x_range;
            mat.rotation = (mat.rotation + rng.gen_range(1..total_colors - 1)) % total_colors;
        }

        if transform.translation.y > y_range {
            bouncy.y = -1;
            transform.translation.y = y_range;
            mat.rotation = (mat.rotation + rng.gen_range(1..total_colors - 1)) % total_colors;
        }

        if transform.translation.y < -y_range {
            bouncy.y = 1;
            transform.translation.y = -y_range;
            mat.rotation = (mat.rotation + rng.gen_range(1..total_colors - 1)) % total_colors;
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FerrisMaterial {
    #[uniform(0)]
    rotation: u32,
    #[uniform(1)]
    total_colors: u32,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
}

impl Material2d for FerrisMaterial {
    fn fragment_shader() -> ShaderRef {
        "ferris.wgsl".into()
    }
}

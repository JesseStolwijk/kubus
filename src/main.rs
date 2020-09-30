extern crate amethyst;

use amethyst::{Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, start_logger, StateData, StateEvent, Trans};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::controls::{FlyControlBundle, FlyControlTag, HideCursor};
use amethyst::core::{Transform, TransformBundle};
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::input::{InputBundle, is_key_down, is_mouse_button_down, StringBindings, VirtualKeyCode};
use amethyst::renderer::{Camera, Material, MaterialDefaults, Mesh, RenderingBundle, RenderToWindow};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::types::DefaultBackend;
use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
        initialize_light(state_data.world);
        initialize_square(state_data.world);
    }


    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
}

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .build();
}

fn initialize_square(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Plane(Some((100, 100)))
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                ..material_defaults
            },
            (),
        )
    },
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}

fn initialize_sphere(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                ..material_defaults
            },
            (),
        )
    },
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let entity = world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .build();

    world
        .write_storage::<FlyControlTag>()
        .insert(entity, Default::default())
        .expect("Unable to attach FlyControlTag to camera");
}

fn main() -> amethyst::Result<()> {
    start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    let display_config = DisplayConfig {
        title: "Amethyst".to_string(),
        dimensions: Some((1024, 768)),
        ..Default::default()
    };

    let key_bindings_path = config_dir.join("key_bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
                .with_sensitivity(0.1, 0.1),
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.529, 0.808, 0.98, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}

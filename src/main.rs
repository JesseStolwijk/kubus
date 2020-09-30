extern crate amethyst;

use amethyst::{Application, GameDataBuilder, SimpleState, start_logger, StateData, GameData};
use amethyst::renderer::{RenderingBundle, RenderToWindow, Camera, Mesh, MaterialDefaults, Material};
use amethyst::renderer::types::DefaultBackend;
use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::core::ecs::{World, WorldExt, Builder};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::renderer::rendy::mesh::{Position, Normal, Tangent, TexCoord};
use amethyst::renderer::shape::Shape;
use amethyst::core::{TransformBundle, Transform};
use amethyst::renderer::light::{PointLight, Light};
use amethyst::renderer::palette::rgb::Rgb;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
        initialize_light(state_data.world);
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

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .build();
}

fn main() -> amethyst::Result<()> {
    start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let display_config = DisplayConfig {
        title: "Amethyst".to_string(),
        dimensions: Some((1024, 768)),
        ..Default::default()
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
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

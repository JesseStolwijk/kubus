use amethyst::core::ecs::{World, WorldExt, Builder};
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::controls::FlyControlTag;

pub fn initialize_camera(world: &mut World) {
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
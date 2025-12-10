use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    radius: f32,
}

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

pub fn create_ball(
    radius: f32,
    color: Color,
    transform: Transform,
    world: &mut World,
) -> BallBundle {
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let ball = Ball { radius };
    let mesh = meshes.add(Circle::new(ball.radius));

    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
    let material = materials.add(color);

    BallBundle {
        ball,
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform,
    }
}

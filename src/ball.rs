use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub velocity: Vec3,
}

#[derive(Component)]
struct Special;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

#[derive(Bundle)]
pub struct SpecialBallBundle {
    ball: Ball,
    special: Special,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}



pub fn create_ball(
    radius: f32,
    color: Color,
    transform: Transform,
    vel: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> BallBundle {

    let ball = Ball { radius, velocity: Vec3::new(vel.x, vel.y, 0.) };
    let mesh = meshes.add(Circle::new(ball.radius));
    let material = materials.add(color);

    BallBundle {
        ball,
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform,
    }
}

pub fn create_special_ball(
    radius: f32,
    color: Color,
    transform: Transform,
    vel: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> SpecialBallBundle {

    let ball = Ball { radius, velocity: Vec3::new(vel.x, vel.y, 0.) };
    let mesh = meshes.add(Circle::new(ball.radius));
    let material = materials.add(color);

    SpecialBallBundle {
        ball,
        special: Special,
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform,
    }
}

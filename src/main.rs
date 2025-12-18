mod ball;
mod experiment;
mod profiler;

use std::f32::consts::PI;
use std::time::Duration;
use rand::prelude::*;
use rand::rng;

use bevy::prelude::*;
use bevy::math::ops::*;
use bevy::color::palettes::basic::*;
use clap::Parser;

use ball::*;
use experiment::*;
use profiler::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print step execution times and draw gizmos
    #[arg(short = 'D', long, default_value_t = true)]
    debug: bool,
    /// The starting step size
    #[arg(short, long, default_value_t = 50)]
    first: usize,
    /// Step increment size
    #[arg(short, long, default_value_t = 50)]
    step: usize,
    /// How many steps to run
    #[arg(short, long, default_value_t = 5)]
    number: usize,
    /// Maximum time for each step
    #[arg(short, long, default_value_t = 10.)]
    duration: f32,
    /// Min number of calculations that should be done for each step
    #[arg(short, long, default_value_t = 200)]
    min: usize,
}

fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProfilerPlugin)
        .add_plugins(ExperimentPlugin {
            first: args.first,
            step: args.step,
            number_of_steps: args.number,
            step_duration: Duration::from_secs_f32(args.duration),
            min_calcs_per_step: args.min,
            variations: 4,
            debug: args.debug,
        })
        .add_systems(Startup, (
            setup, add_ball,
        ))
        .add_systems(
            Update, (
                clear_balls.run_if(on_message::<ExperimentProgress>).before(add_ball),
                add_ball.run_if(on_message::<ExperimentProgress>),
                move_balls,
            ),
        )
        .add_systems(PostUpdate, process_experiment_progress)
        .run();
}

fn setup(
    mut commands: Commands,
    mut profiler: ResMut<Profiler>,
    exp_params: Res<ExperimentParameters>,
) {
    commands.spawn(Camera2d);
    // let algorithms: Vec<String> = vec![
    //     "None".to_string(),
    //     "PairDetection".to_string(),
    //     "PairBoundingBox".to_string(),
    //     "QuadTree".to_string()
    // ];
    // let samples = exp_params.relevant_samples();
    //
    // let index = profiler.create_table("Collision", algorithms, samples.clone());
    // commands.insert_resource(CollisionTableIndex(index));
    //
    // let qt_index = profiler.create_table(
    //     "Quad Tree",
    //     vec!["Build time".to_string(), "Traversal time".to_string()],
    //     samples.clone(),
    // );
    // commands.insert_resource(QuadTreeTableIndex(qt_index));
}

fn clear_balls(balls: Query<Entity, With<Ball>>, mut commands: Commands) {
    println!("{}", "clear");
    for ball in balls.iter() {
        commands.entity(ball).despawn();
    }
}

fn random_on_circle(rng: &mut ThreadRng) -> Vec2 {
    let angle = rng.random::<f32>() * 2. * PI;
    Vec2::new(cos(angle), sin(angle))
}

fn add_ball(
    mut commands: Commands,
    window: Single<&Window>,
    exp_params: Res<ExperimentParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("{}", "add");

    let size = exp_params.current_sample_size();
    let mut rng = rng();
    let radius = 15.;
    let min_speed = 100.;
    let max_speed = 300.;
    let height = window.height() - radius * 2.;
    let width = window.width() - radius * 2.;
    let miny = radius - window.height() / 2.;
    let minx = radius - window.width() / 2.;

    let mut balls: Vec<BallBundle> = Vec::with_capacity(size);

    for _i in 0..size {
        let random_x = minx + width * rng.random::<f32>();
        let random_y = miny + height * rng.random::<f32>();
        let random_speed = min_speed + (max_speed - min_speed) * rng.random::<f32>();
        let random_velocity = random_on_circle(&mut rng) * random_speed;

        balls.push(create_ball(
            radius,
            Color::from(GRAY),
            Transform::from_xyz(random_x, random_y, 0.),
            random_velocity,
            &mut meshes,
            &mut materials,
        ));
    }

    commands.spawn_batch(balls);

    let random_x = minx + width * rng.random::<f32>();
    let random_y = miny + height * rng.random::<f32>();
    let random_speed = min_speed + (max_speed - min_speed) * rng.random::<f32>();
    let random_velocity = random_on_circle(&mut rng) * random_speed;
    
    let special_ball = create_special_ball(
        radius,
        Color::from(BLACK),
        Transform::from_xyz(random_x, random_y, 0.),
        random_velocity,
        &mut meshes, 
        &mut materials,
    );
    
    commands.spawn(special_ball);
}

fn check_in_bounds(rect: Rect, pos: Vec2, radius: f32) -> usize {
    if pos.x > rect.max.x - radius { return 1; }
    if pos.y > rect.max.y - radius { return 2; }
    if pos.x < rect.min.x + radius { return 3; }
    if pos.y < rect.min.y + radius { return 4; }
    0
}

fn move_balls(balls: Query<(&mut Transform, &mut Ball)>, window: Single<&Window>, time: Res<Time>) {
    for (mut transform, mut ball) in balls {
        transform.translation += ball.velocity * time.delta_secs();

        let window_rect = Rect::from_center_size(Vec2::ZERO, window.size());
        let trunc_pos = transform.translation.truncate();

        match check_in_bounds(window_rect, trunc_pos, ball.radius) {
            1 => ball.velocity = ball.velocity.reflect(Vec3::NEG_X),
            2 => ball.velocity = ball.velocity.reflect(Vec3::NEG_Y),
            3 => ball.velocity = ball.velocity.reflect(Vec3::X),
            4 => ball.velocity = ball.velocity.reflect(Vec3::Y),
            _ => {},
        }


        let r = Rect::from_center_half_size( // window size adjusted for ball radius
            window_rect.center(),
            window_rect.half_size() - ball.radius
        );
        let max_3 = Vec3::new(r.max.x, r.max.y, 0.);
        let min_3 = Vec3::new(r.min.x, r.min.y, 0.);

        transform.translation = transform.translation.clamp(min_3, max_3);
    }
}

fn process_experiment_progress(mut progress: MessageReader<ExperimentProgress>) {
    progress.read();
}

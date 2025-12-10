mod ball;
mod experiment;
mod profiler;

use std::time::Duration;
use bevy::prelude::*;
use clap::Parser;
use profiler::*;
use experiment::*;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print step execution times and draw gizmos
    #[arg(short = 'D', long, default_value_t = false)]
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
        .run();
}


fn add_ball(exp_params: Res<ExperimentParameters>) {

    


}


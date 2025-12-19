use std::time::{Duration, Instant};
use bevy::prelude::*;





pub struct ExperimentPlugin {
    pub first: usize,
    pub step: usize,
    pub variations: usize,
    pub number_of_steps: usize,
    pub step_duration: Duration,
    pub min_calcs_per_step: usize,
    pub pick_number: usize,
    pub debug: bool,
}

impl Plugin for ExperimentPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ExperimentProgress>();
        app.insert_resource(ExperimentParameters::new(
            self.first,
            self.step,
            self.variations,
            self.number_of_steps,
            self.step_duration,
            self.min_calcs_per_step,
            self.pick_number,
            self.debug,
        ));
        app.add_systems(Update, progress_experiment);
        app.add_systems(PreStartup, record_start_up_instant);
    }
}

#[derive(Resource)]
pub struct ExperimentParameters {
    pub sample_sizes: [usize; 100],
    pub sample_sizes_as_str: Vec<String>,
    pub sample_index: usize,
    pub number_samples: usize,
    sample_duration: Duration,
    current_sample_progress: Duration,
    min_calcs_per_sample: usize,
    current_calcs: usize,
    pub variation_index: usize,
    pub number_variations: usize,
    pub pick_number: usize,
    pub debug: bool,
}

impl ExperimentParameters {
    fn new(
        first: usize,
        step: usize,
        number_variations: usize,
        number_samples: usize,
        sample_duration: Duration,
        min_calcs_per_sample: usize,
        pick_number: usize,
        debug: bool,
    ) -> ExperimentParameters {
        let sample_sizes = generate_sample_sizes(first, step);
        let mut sample_sizes_as_str: Vec<String> = Vec::with_capacity(100);
        for sample_size in sample_sizes.iter() {
            sample_sizes_as_str.push(sample_size.to_string());
        }

        ExperimentParameters {
            sample_sizes,
            sample_sizes_as_str,
            number_samples,
            sample_index: 0,
            number_variations,
            variation_index: 0,
            sample_duration,
            current_sample_progress: Duration::from_secs(0),
            min_calcs_per_sample,
            current_calcs: 0,
            pick_number,
            debug,
        }
    }

    pub fn current_sample_size(&self) -> usize {
        self.sample_sizes[self.sample_index]
    }

    #[allow(unused)]
    pub fn current_sample_size_str(&self) -> String {
        self.sample_sizes_as_str[self.sample_index].clone()
    }

    /// return false if there is no next variation
    pub fn next_variation(&mut self) -> bool {
        self.variation_index += 1;
        self.variation_index < self.number_variations
    }

    /// return false if there is no next sample
    pub fn next_sample(&mut self) -> bool {
        self.sample_index += 1;
        self.sample_index < self.number_samples
    }

    pub fn relevant_samples(&self) -> Vec<String> {
        Vec::from(&self.sample_sizes_as_str[..self.number_samples])
    }
}

fn generate_sample_sizes(first: usize, step: usize) -> [usize; 100] {
    let mut array: [usize; 100] = [0; 100];

    #[allow(clippy::needless_range_loop)]
    for i in 0..100 {
        array[i] = first + (step * i);
    }

    array
}


/// * `0`: previous sample size index
/// * `1`: previous variation index
/// * `2`: last sample
#[derive(Message)]
pub struct ExperimentProgress(pub usize, pub usize, pub bool);

fn progress_experiment(
    mut parameters: ResMut<ExperimentParameters>,
    mut writer: MessageWriter<ExperimentProgress>,
    time: Res<Time>,
) {
    parameters.current_calcs += 1;
    parameters.current_sample_progress += Duration::from_secs_f32(time.delta_secs());
    let duration_over = parameters.current_sample_progress >= parameters.sample_duration;
    let min_calcs_reached = parameters.current_calcs >= parameters.min_calcs_per_sample;
    if !duration_over && !min_calcs_reached { return; }

    let prev_sample_size_index = parameters.sample_index;
    let prev_variation_index = parameters.variation_index;

    let last_sample = !parameters.next_sample(); // increment sample index
    let mut last_variation = false;
    if last_sample {
        last_variation = !parameters.next_variation(); // increment variation index
        parameters.sample_index = 0; // reset sample index
    }

    parameters.current_sample_progress = Duration::from_secs(0);
    parameters.current_calcs = 0;
    writer.write(ExperimentProgress(
        prev_sample_size_index,
        prev_variation_index,
        last_sample && last_variation,
    ));
}

#[derive(Resource)]
pub struct StartupInstant(pub Instant);

fn record_start_up_instant(mut commands: Commands) {
    commands.insert_resource(StartupInstant(Instant::now()));
}

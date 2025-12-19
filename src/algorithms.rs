use crate::ball::{Ball, Special};
use crate::experiment::ExperimentParameters;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;
use crate::MarkBallMessage;

struct BallData {
    entity: Entity,
    distance: f32,
}

impl Clone for BallData {
    fn clone(&self) -> Self {
        BallData {
            entity: self.entity,
            distance: self.distance,
        }
    }
}

impl Copy for BallData {}

pub fn merge_sort_naive(
    balls: Query<(Entity, &Transform, &Ball)>,
    special: Single<&Transform, With<Special>>,
    exp_params: &Res<ExperimentParameters>,
    mut writer: MessageWriter<MarkBallMessage>,
) -> u128 {

    let mut unsorted_ball_list: VecDeque<BallData> =
        VecDeque::with_capacity(exp_params.current_sample_size());

    for ball in balls {
        unsorted_ball_list.push_back(BallData {
            entity: ball.0,
            distance: ball.1.translation.distance(special.translation),
        })
    }

    let start = Instant::now();
    let sorted_balls = merge_alloc(unsorted_ball_list);
    let elapsed = start.elapsed().as_nanos();


    for i in 0..exp_params.pick_number {
        writer.write(MarkBallMessage(sorted_balls[i].entity));
    }

    elapsed
}

fn merge_alloc(unsorted: VecDeque<BallData>) -> VecDeque<BallData> {
    let length = unsorted.len();

    if length > 1 {
        let (half1, half2) = split_queue_alloc(unsorted);

        let mut half1 = merge_alloc(half1);
        let mut half2 = merge_alloc(half2);

        let mut merged: VecDeque<BallData> = VecDeque::with_capacity(length);
        let mut index = 0;

        while index < length {
            let e1 = half1.get(0).unwrap();
            let e2 = half2.get(0).unwrap();

            if e1.distance < e2.distance {
                merged.push_back(half1.pop_front().unwrap());
            } else {
                merged.push_back(half2.pop_front().unwrap());
            }   

            index += 1;
        }

        return merged;
    }

    // if length is 1 or zero the array is sorted
    unsorted
}

fn split_queue_alloc(mut queue: VecDeque<BallData>) -> (VecDeque<BallData>, VecDeque<BallData>) {
    let half_len = queue.len() / 2;

    let mut half1: VecDeque<BallData> = VecDeque::with_capacity(half_len);
    let mut half2: VecDeque<BallData> = VecDeque::with_capacity(half_len);

    for _i in 0..half_len {
        half1.push_back(queue.pop_front().unwrap());
    }

    for _i in half_len..queue.len() {
        half2.push_back(queue.pop_front().unwrap());
    }

    (half1, half2)
}

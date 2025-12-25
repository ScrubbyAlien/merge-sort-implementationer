use std::cmp::min;
use crate::MarkBallMessage;
use crate::ball::{Ball, Special};
use crate::experiment::ExperimentParameters;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;
use bevy::log::tracing_subscriber::fmt::writer::EitherWriter::B;

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
    balls: Query<(Entity, &Transform, &Ball), Without<Special>>,
    special: Single<&Transform, With<Special>>,
    exp_params: &Res<ExperimentParameters>,
    mut writer: MessageWriter<MarkBallMessage>,
) -> u128 {
    let mut unsorted_ball_list: VecDeque<BallData> =
        VecDeque::with_capacity(exp_params.current_sample_size());

    for ball in balls {
        unsorted_ball_list.push_back(BallData {
            entity: ball.0,
            distance: ball.1.translation.distance_squared(special.translation),
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
        let mut half2 = merge_alloc(half2); // if length is odd, half2 will have one more element

        let mut merged: VecDeque<BallData> = VecDeque::with_capacity(length);

        for _i in 0..length {
            merged.push_back(get_smallest(&mut half1, &mut half2));
        }

        return merged;
    }

    // if length is 1 or zero the array is sorted
    unsorted
}

fn split_queue_alloc(mut queue: VecDeque<BallData>) -> (VecDeque<BallData>, VecDeque<BallData>) {
    let len = queue.len();
    let half_len = queue.len() / 2;

    let mut half1: VecDeque<BallData> = VecDeque::with_capacity(half_len);
    let mut half2: VecDeque<BallData> = VecDeque::with_capacity(half_len);

    for _i in 0..half_len {
        half1.push_back(queue.pop_front().unwrap());
    }

    for _i in half_len..len {
        half2.push_back(queue.pop_front().unwrap());
    }

    (half1, half2)
}

fn get_smallest(half1: &mut VecDeque<BallData>, half2: &mut VecDeque<BallData>) -> BallData {
    debug_assert!(!half1.is_empty() || !half2.is_empty());

    if half1.is_empty() {
        half2.pop_front().unwrap()
    } else if half2.is_empty() {
        half1.pop_front().unwrap()
    } else {
        let e1 = half1.get(0).unwrap();
        let e2 = half2.get(0).unwrap();

        if e1.distance < e2.distance {
            half1.pop_front().unwrap()
        } else {
            half2.pop_front().unwrap()
        }
    }
}

pub fn merge_sort_memory(
    balls: Query<(Entity, &Transform, &Ball), Without<Special>>,
    special: Single<&Transform, With<Special>>,
    exp_params: &Res<ExperimentParameters>,
    mut writer: MessageWriter<MarkBallMessage>,
) -> u128 {
    // https://www.geeksforgeeks.org/dsa/in-place-merge-sort/

    let mut ball_list: Vec<BallData> =
        Vec::with_capacity(exp_params.current_sample_size());

    for ball in balls {
        ball_list.push(BallData {
            entity: ball.0,
            distance: ball.1.translation.distance_squared(special.translation),
        })
    }
    
    let mut temp = vec![BallData {entity: Entity::PLACEHOLDER, distance: 0.}; balls.iter().len()];
    
    let start = Instant::now();
    merge_sort(&mut ball_list[..], &mut temp[..]);
    let elapsed = start.elapsed().as_nanos();

    for i in 0..exp_params.pick_number {
        writer.write(MarkBallMessage(ball_list[i].entity));
    }
    
    elapsed
}

// start index inclusive, end index exclusive
fn merge_sort(
    unsorted: &mut [BallData],
    temp: &mut [BallData],
) {
    if unsorted.len() > 1 {
        let half_way = unsorted.len() / 2;

        merge_sort(&mut unsorted[..half_way], temp);
        merge_sort(&mut unsorted[half_way..], temp);

        merge(unsorted, temp, half_way);

    }


}

fn merge(unsorted: &mut [BallData], temp: &mut [BallData], half_way: usize) {
    debug_assert!(unsorted.len() > 0);
    debug_assert!(temp.len() >= unsorted.len());
    
    let mut start1 = 0;
    let mut start2 = half_way;
    let length = unsorted.len();

    for index in 0..length {
        let e1 = unsorted.get(start1);
        let e2 = unsorted.get(start2);

        if e1.is_none() {
            temp[index] = *e2.unwrap();
            start2 += 1;
        } else if e2.is_none() {
            temp[index] = *e1.unwrap();
            start1 += 1;
        } else {
            if e1.unwrap().distance < e2.unwrap().distance {
                temp[index] = *e1.unwrap();
                start1 += 1;
            } else {
                temp[index] = *e2.unwrap();
                start2 += 1;
            }
        }
    }
    
    for index in 0..length {
        unsorted[index] = temp[index];
    }
}

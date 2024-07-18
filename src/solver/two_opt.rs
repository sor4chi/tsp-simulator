use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    draw::plot_tour,
    util::{distance, total_distance, TimeKeeper},
};

use super::{AnnealingConfig, Report};

pub fn two_opt_tsp(config: AnnealingConfig, seed: u64, tour: Vec<(f64, f64)>) -> Report {
    let keeper = TimeKeeper::new();
    let n = tour.len();
    let mut order = (0..n).collect::<Vec<_>>();

    let mut rng = StdRng::seed_from_u64(seed);
    let mut temp = config.first_temp;
    let initial_distance = total_distance(&tour);

    let mut best_order = order.clone();
    let mut best_distance = initial_distance;
    let mut dist = initial_distance;
    let mut each_dist = vec![];
    for i in 0..n {
        let from = order[i];
        let to = order[(i + 1) % order.len()];
        each_dist.push(distance(tour[from], tour[to]));
    }

    let mut iteration = 0;
    while !keeper.is_time_up(config.tl) {
        iteration += 1;
        let i = rng.gen_range(0..n - 1);
        let j = rng.gen_range(i + 1..n);

        let mut new_order = order.clone();
        let mut new_dist = dist;

        let new_from_dist = distance(tour[order[(i - 1) % n]], tour[order[j]]);
        let new_to_dist = distance(tour[order[i]], tour[order[(j + 1) % n]]);

        new_dist -= each_dist[(i - 1) % n];
        new_dist -= each_dist[j];
        new_dist += new_from_dist;
        new_dist += new_to_dist;

        new_order[i..=j].reverse();

        if new_dist < dist || rng.gen::<f64>() < (-(new_dist - dist) / temp).exp() {
            order = new_order;
            dist = new_dist;
            each_dist[(i - 1) % n] = new_from_dist;
            each_dist[j] = new_to_dist;
            each_dist[i..j].reverse();
            if dist < best_distance {
                best_order.clone_from(&order);
                best_distance = dist;
            }
        }

        temp = config.first_temp
            + (config.end_temp - config.first_temp) * keeper.elapsed() / config.tl as f64;

        if iteration % config.draw_interval == 0 {
            let filename = format!("2-opt-sa/iters/{}.svg", iteration);
            plot_tour(
                &order.iter().map(|&i| tour[i]).collect::<Vec<_>>(),
                &filename,
            );
        }

        if iteration % config.report_interval == 0 {
            println!(
                "Iteration: {}\tBest: {:.2}\tCurrent: {:.2}\tTemp: {:.2}",
                iteration, best_distance, dist, temp
            );
        }
    }
    Report {
        best_tour: best_order.iter().map(|&i| tour[i]).collect(),
        initial_distance,
        best_distance,
        iteration,
    }
}

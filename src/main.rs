mod draw;
mod solver;
mod util;

use draw::plot_tour;
use rand::prelude::*;
use solver::{two_opt::two_opt_tsp, AnnealingConfig};

fn generate_cities(num_cities: usize, seed: u64) -> Vec<(f64, f64)> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..num_cities)
        .map(|_| (rng.gen_range(0.0..1000.0), rng.gen_range(0.0..1000.0)))
        .collect()
}

fn main() {
    // === Parameters ===
    let num_cities = 3000;
    let seed = 42;
    let config = AnnealingConfig {
        first_temp: 1e1,
        end_temp: 1e-4,
        tl: 10,
        draw_interval: 100000000,
        report_interval: 10000,
    };
    let per_clean = true;
    // ==================

    if per_clean {
        std::fs::remove_dir_all("output").unwrap();
    }

    let cities = generate_cities(num_cities, seed);

    plot_tour(&cities, "2-opt-sa/initial.svg");

    let optimized_tour = two_opt_tsp(config, seed, cities);

    plot_tour(&optimized_tour.best_tour, "2-opt-sa/optimized.svg");

    optimized_tour.print("Two-Opt + Simulated Annealing");
}

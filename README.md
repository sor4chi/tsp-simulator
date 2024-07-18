# TSP Simulator

This is a simple simulator for the Traveling Salesman Problem (TSP) using the following algorithms:

- 2-Opt (Simulated Annealing)
- Comming Soon...

You can configure the parameters of the simulation in the `src/main.rs` file.

```rs
// === Parameters ===
let num_cities = 3000; // The number of cities
let seed = 42; // Random seed for generating cities
let config = AnnealingConfig { // Configuration for the 2-Opt algorithm
    first_temp: 1e1, // Initial temperature
    end_temp: 1e-4, // Final temperature
    tl: 10, // Time limit
    draw_interval: 100000000, // Interval for generating SVG files
    report_interval: 10000, // Interval for printing the current state
};
let per_clean = true; // Perform a cleaning step after each iteration
// ==================
```

To run the simulation, execute the following command:

```sh
cargo run --release
```

## Showcase

- 2-Opt (Simulated Annealing)
- Number of cities: 3000
- Seed: 42
- Time limit: 10s
- Temperature: 1e1 -> 1e-4

<div>
    <img src="./output/2-opt-sa/initial.svg" alt="Initial Statement" width="45%">
    <img src="./output/2-opt-sa/optimized.svg" alt="Optimized Statement" width="45%">
</div>

pub mod two_opt;

pub struct AnnealingConfig {
    pub first_temp: f64,
    pub end_temp: f64,
    pub tl: usize,
    pub draw_interval: usize,
    pub report_interval: usize,
}

pub struct Report {
    pub best_tour: Vec<(f64, f64)>,
    initial_distance: f64,
    best_distance: f64,
    iteration: usize,
}

impl Report {
    pub fn print(&self, title: &str) {
        let title = format!("=== {} ({} cities) ===", title, self.best_tour.len());
        println!("{}", title);
        println!("Initial dist:\t{}", self.initial_distance);
        println!("Optimized dist:\t{}", self.best_distance);
        println!("Iteration:\t{}", self.iteration);
        println!(
            "Improvement:\t{:.2}%",
            100.0 * (self.initial_distance - self.best_distance) / self.initial_distance
        );
        println!("{}", "=".repeat(title.len()));
    }
}

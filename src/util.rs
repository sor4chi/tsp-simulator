use std::time::Instant;

pub fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub fn total_distance(tour: &[(f64, f64)]) -> f64 {
    let mut dist = 0.0;
    for i in 0..tour.len() {
        dist += distance(tour[i], tour[(i + 1) % tour.len()]);
    }
    dist
}

pub struct TimeKeeper {
    start: Instant,
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> f64 {
        let elapsed = self.start.elapsed();
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
    }

    pub fn is_time_up(&self, tl: usize) -> bool {
        self.elapsed() > tl as f64
    }
}

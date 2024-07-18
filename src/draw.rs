use std::{fs, path};

struct ColorGenerator {
    current: f32,
    length: usize,
}

impl ColorGenerator {
    fn new(length: usize) -> Self {
        ColorGenerator {
            current: 0.0,
            length,
        }
    }

    fn next(&mut self) -> String {
        let hue = self.current;
        self.current += 1.0 / self.length as f32;
        format!("hsl({}, 100%, 50%)", hue * 360.0)
    }
}

pub fn plot_tour(tour: &[(f64, f64)], filename: &str) {
    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 1000, 1000))
        .set("style", "background-color: #000");
    let mut color_generator = ColorGenerator::new(tour.len());

    for i in 0..tour.len() {
        let a = tour[i];
        let b = tour[(i + 1) % tour.len()];
        document = document.add(
            svg::node::element::Path::new()
                .set("fill", "none")
                .set("stroke", color_generator.next())
                .set("stroke-width", 1)
                .set("d", format!("M {} {} L {} {}", a.0, a.1, b.0, b.1)),
        );
    }

    let mut path = path::PathBuf::from("output");
    path.push(filename);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    svg::save(path, &document).unwrap();
}

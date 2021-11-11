extern crate rand;
use macroquad::prelude::*;
use rust_grid::Grid;
use std::time::Instant;

fn window_conf() -> Conf {
    Conf {
        window_title: "Triangles".to_owned(),
        window_height: 512,
        window_width: 512,
        ..Default::default()
    }
}

struct TriangleGrid {
    data: Grid<Color>,
}

impl TriangleGrid {
    fn new(width: usize, height: usize) -> Self {
        let mut data: Grid<Color> = Grid::new(width, height, Color { ..Default::default() });

        Self::randomise_colors(&mut data);

        Self {
            data,
        }
    }

    fn randomise_colors(grid: &mut Grid<Color>) {
        let mut rng = rand::thread_rng();
        let (width, height) = grid.size();

        for x in 0..width {
            for y in 0..height {
                use rand::Rng;

                grid[(x, y)] = Color {
                    r: rng.gen_range(0.0..1.0),
                    g: rng.gen_range(0.0..1.0),
                    b: rng.gen_range(0.0..1.0),
                    a: 1.0,
                };
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = TriangleGrid::new(20, 10);
    let mut last_time = Instant::now();

    loop {
        if Instant::now() - last_time > std::time::Duration::from_millis(500) {
            last_time = Instant::now();
            TriangleGrid::randomise_colors(&mut grid.data);
        }

        clear_background(BLACK);

        let size = 50.;
        let (width, height) = grid.data.size();

        for x in 0..width {
            for y in 0..height {
                let (xc, yc) = (x as f32, y as f32);

                if x % 2 == 0 {
                    draw_triangle(
                        vec2((xc / 2. + 0.5) * size, yc * size),
                        vec2((xc / 2. + 1.) * size, (yc + 1.) * size),
                        vec2(xc / 2. * size, (yc + 1.) * size),
                        grid.data[(x, y)],
                    );
                } else {
                    draw_triangle(
                        vec2((xc / 2. + 0.5) * size, (yc + 1.) * size),
                        vec2((xc / 2. + 1.) * size, yc * size),
                        vec2(xc / 2. * size, yc * size),
                        grid.data[(x, y)],
                    );
                };
            }
        }

        next_frame().await
    }
}

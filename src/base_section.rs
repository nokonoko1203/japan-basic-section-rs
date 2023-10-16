extern crate geo;

use crate::origin_coords::{Origin, PlaneRectangularCoordinateSystem};
use geo::{LineString, Polygon};

#[derive(Debug)]
pub struct BaseSection {
    number: i32,
    system_name: PlaneRectangularCoordinateSystem,
    origin: Origin,
    bbox: [i32; 4],
    top_left: (i32, i32),
    x_grid: i32,
    y_grid: i32,
    grid_size: (f64, f64),
    grid: Vec<GridCell>,
}

#[derive(Debug)]
pub struct GridCell {
    pub index: String,
    pub polygon: Polygon<f64>,
}

impl BaseSection {
    pub fn new(number: i32, system_name: PlaneRectangularCoordinateSystem, origin: Origin) -> Self {
        let bbox = [-160000, -300000, 160000, 300000];
        let top_left = (bbox[0], bbox[3]);
        let x_grid = 8;
        let y_grid = 20;
        let grid_size = (
            (bbox[2] - bbox[0]) as f64 / x_grid as f64,
            (bbox[3] - bbox[1]) as f64 / y_grid as f64,
        );
        let grid = Self::_get_grid(number, top_left, grid_size, x_grid, y_grid);
        Self {
            number,
            system_name,
            origin,
            bbox,
            top_left,
            x_grid,
            y_grid,
            grid_size,
            grid,
        }
    }

    fn _get_grid(
        number: i32,
        top_left: (i32, i32),
        grid_size: (f64, f64),
        x_grid: i32,
        y_grid: i32,
    ) -> Vec<GridCell> {
        let mut grid = Vec::new();
        for j in 0..y_grid {
            for i in 0..x_grid {
                let index = format!(
                    "{:02}{}{}",
                    number,
                    (b'A' + j as u8) as char,
                    (b'A' + i as u8) as char
                );

                let exterior = LineString::from(vec![
                    (
                        top_left.0 as f64 + grid_size.0 * i as f64,
                        top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                    (
                        top_left.0 as f64 + grid_size.0 * (i + 1) as f64,
                        top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                    (
                        top_left.0 as f64 + grid_size.0 * (i + 1) as f64,
                        top_left.1 as f64 - grid_size.1 * (j + 1) as f64,
                    ),
                    (
                        top_left.0 as f64 + grid_size.0 * i as f64,
                        top_left.1 as f64 - grid_size.1 * (j + 1) as f64,
                    ),
                    (
                        top_left.0 as f64 + grid_size.0 * i as f64,
                        top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                ]);

                let polygon = Polygon::new(exterior, vec![]);
                grid.push(GridCell { index, polygon });
            }
        }
        grid
    }
}

extern crate geo;

use crate::origin_coords::{Origin, PlaneRectangularCoordinateSystem};
use geo::{coord, polygon};
use geo_types::Polygon;

#[derive(Debug)]
pub struct BaseSection {
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
    pub fn new(system_name: PlaneRectangularCoordinateSystem) -> Self {
        let system_name = system_name;
        let origin = system_name.origin();
        let bbox = [-160000, -300000, 160000, 300000];
        let top_left = (bbox[0], bbox[3]);
        let x_grid = 8;
        let y_grid = 20;
        let grid_size = (
            (bbox[2] - bbox[0]) as f64 / x_grid as f64,
            (bbox[3] - bbox[1]) as f64 / y_grid as f64,
        );
        let grid = Self::get_grid(system_name, top_left, grid_size, x_grid, y_grid);
        Self {
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

    fn get_grid(
        system_name: PlaneRectangularCoordinateSystem,
        top_left: (i32, i32),
        grid_size: (f64, f64),
        x_grid: i32,
        y_grid: i32,
    ) -> Vec<GridCell> {
        let mut grid = Vec::new();

        for j in 0..y_grid {
            for i in 0..x_grid {
                let index = format!(
                    "{:?}{}{}",
                    system_name,
                    (b'A' + j as u8) as char,
                    (b'A' + i as u8) as char
                );

                let polygon = polygon![
                    coord!(
                        x: top_left.0 as f64 + grid_size.0 * i as f64,
                        y: top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                    coord!(
                        x: top_left.0 as f64 + grid_size.0 * (i + 1) as f64,
                        y: top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                    coord!(
                        x: top_left.0 as f64 + grid_size.0 * (i + 1) as f64,
                        y: top_left.1 as f64 - grid_size.1 * (j + 1) as f64,
                    ),
                    coord!(
                        x: top_left.0 as f64 + grid_size.0 * i as f64,
                        y: top_left.1 as f64 - grid_size.1 * (j + 1) as f64,
                    ),
                    coord!(
                        x: top_left.0 as f64 + grid_size.0 * i as f64,
                        y: top_left.1 as f64 - grid_size.1 * j as f64,
                    ),
                ];
                grid.push(GridCell { index, polygon });
            }
        }
        grid
    }
}

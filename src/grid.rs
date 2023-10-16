use crate::base_section::{BaseSection, GridCell};
use crate::origin_coords::PlaneRectangularCoordinateSystem;
use geo::BoundingRect;
use geo::{coord, polygon};

#[derive(Debug)]
pub struct Grid {
    system_number: PlaneRectangularCoordinateSystem,
    level: u32,
    base: BaseSection,
}

impl Grid {
    pub fn new(
        system_number: PlaneRectangularCoordinateSystem,
        level: u32,
    ) -> Result<Self, String> {
        if [50000, 5000, 2500, 1000, 500].contains(&level) {
            Ok(Self {
                system_number,
                level,
                base: BaseSection::new(system_number.clone()),
            })
        } else {
            Err(format!(
                "Invalid level: {} (must be one of 50000, 5000, 2500, 1000, 500)",
                level
            ))
        }
    }

    fn get_grid_size(&self, level: u32) -> (u8, u8) {
        match level {
            50000 => (10, 10),
            5000 => (10, 10),
            2500 => (2, 2),
            1000 => (5, 5),
            500 => (10, 10),
            _ => panic!("Invalid level: {}", level),
        }
    }

    fn make_number_name(index: &str, j: u8, i: u8, level: u32) -> String {
        let result = match level {
            50000 => format!("{}{}{}", index, j, i),
            5000 => format!("{}{}{}", index, j, i),
            2500 => match (j, i) {
                (0, 0) => format!("{}{}", index, 1),
                (0, 1) => format!("{}{}", index, 2),
                (1, 0) => format!("{}{}", index, 3),
                (1, 1) => format!("{}{}", index, 4),
                _ => panic!("Invalid j, i: {}, {}", j, i),
            },
            1000 => format!("{}{}{}", index, j, (b'A' + i as u8) as char),
            500 => format!("{}{}{}", index, j, i),
            _ => panic!("Invalid level: {}", level),
        };
        result
    }

    pub fn make_grid(&self) -> Vec<GridCell> {
        let mut grid = Vec::new();

        let (x_grid, y_grid) = Self::get_grid_size(&self, self.level);

        for cell in self.base.grid.iter() {
            let polygon = &cell.polygon;
            let bounds = polygon.bounding_rect().unwrap();

            let x_min = bounds.min().x;
            let y_min = bounds.min().y;
            let x_max = bounds.max().x;
            let y_max = bounds.max().y;

            let x_size = (x_max - x_min) / x_grid as f64;
            let y_size = (y_max - y_min) / y_grid as f64;

            let top_left = (x_min, y_max);

            for j in 0..y_grid {
                for i in 0..x_grid {
                    let number = Self::make_number_name(&cell.index, j, i, self.level);

                    let polygon = polygon![
                        coord!(
                            x: top_left.0 + x_size * i as f64,
                            y: top_left.1 - y_size * j as f64,
                        ),
                        coord!(
                            x: top_left.0 + x_size * (i + 1) as f64,
                            y: top_left.1 - y_size * j as f64,
                        ),
                        coord!(
                            x: top_left.0 + x_size * (i + 1) as f64,
                            y: top_left.1 - y_size * (j + 1) as f64,
                        ),
                        coord!(
                            x: top_left.0 + x_size * i as f64,
                            y: top_left.1 - y_size * (j + 1) as f64,
                        ),
                        coord!(
                            x: top_left.0 + x_size * i as f64,
                            y: top_left.1 - y_size * j as f64,
                        ),
                    ];

                    grid.push(GridCell {
                        index: number,
                        polygon,
                    });
                }
            }
        }

        grid
    }
}

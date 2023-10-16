use std::fmt::format;

use crate::origin_coords::PlaneRectangularCoordinateSystem;

#[derive(Debug)]
pub struct Grid {
    system_number: PlaneRectangularCoordinateSystem,
    level: u32,
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
            })
        } else {
            Err(format!(
                "Invalid level: {} (must be one of 50000, 5000, 2500, 1000, 500)",
                level
            ))
        }
    }

    fn get_grid_size(level: u32) -> (u8, u8) {
        match level {
            50000 => (10, 10),
            5000 => (10, 10),
            2500 => (2, 2),
            1000 => (5, 5),
            500 => (10, 10),
            _ => panic!("Invalid level: {}", level),
        }
    }

    fn make_number_name(index: &str, j: u32, i: u32, level: u32) -> String {
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
}

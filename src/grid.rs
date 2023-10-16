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
}

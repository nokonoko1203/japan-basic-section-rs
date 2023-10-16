mod base_section;
mod grid;
mod origin_coords;

use grid::Grid;

fn main() {
    let zone = origin_coords::PlaneRectangularCoordinateSystem::FIRST;
    let origin = zone.origin();
    let epsg = zone.epsg();
    println!("Zone: {:?}", zone);
    println!("Origin: {:?}", origin);
    println!("EPSG: {:?}", epsg);

    let gdf = match Grid::new(zone, 5000) {
        Ok(grid) => grid,
        Err(e) => panic!("Error: {}", e),
    };

    let result = gdf.make_grid();
    // 最初の要素だけ表示
    println!("{:?}", result[0]);
}

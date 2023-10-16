mod origin_coords;

fn main() {
    let zone = origin_coords::PlaneRectangularCoordinateSystem::FIRST;
    let origin = zone.origin();
    let epsg = zone.epsg();
    println!("Zone: {:?}", zone);
    println!("Origin: {:?}", origin);
    println!("EPSG: {:?}", epsg);
}

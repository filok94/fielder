pub mod to_kml;
pub mod to_shapefile;

fn main() {
    to_shapefile::write_shapefile(String::from("data/map.geojson"));
}

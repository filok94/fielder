use std::{fs, path::PathBuf};

use geo::Geometry;
use serde::Deserialize;
use shapefile::{
    self,
    dbase::{self, dbase_record, TableWriterBuilder},
    Polygon, Writer,
};
use uuid::Uuid;

#[derive(Deserialize)]
struct GeoJsonRecord {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Geometry,
    sf_name: String,
    sf_ext: String,
    f_name: String,
    f_ext: String,
    f_area: f32,
}

dbase_record!(
    #[derive(Debug)]
    struct FieldRecord {
        sf_name: String,
        sf_ext: String,
        f_name: String,
        f_ext: String,
        f_area: f32,
    }
);

impl FieldRecord {
    fn from_geo_record(geo_record: GeoJsonRecord) -> FieldRecord {
        FieldRecord {
            sf_name: geo_record.sf_name,
            sf_ext: geo_record.sf_ext,
            f_name: geo_record.f_name,
            f_ext: geo_record.f_ext,
            f_area: geo_record.f_area,
        }
    }
}

pub fn write_shapefile(json_file_path: PathBuf) {
    let mut shapefile_name = Uuid::new_v4().to_string();
    shapefile_name.push_str(".shp");

    let wkt_data = fs::read_to_string(json_file_path).expect("Не прочитал файл");
    let fields: Vec<GeoJsonRecord> =
        geojson::de::deserialize_feature_collection_str_to_vec(&wkt_data)
            .expect("Не смог собрать поля");
    let table_builder = TableWriterBuilder::new()
        .add_character_field("sf_name".try_into().unwrap(), 255)
        .add_character_field("sf_ext".try_into().unwrap(), 255)
        .add_character_field("f_name".try_into().unwrap(), 255)
        .add_character_field("f_ext".try_into().unwrap(), 255)
        .add_float_field("f_area".try_into().unwrap(), 4, 4);
    let mut writer = Writer::from_path(shapefile_name, table_builder)
        .expect("не смог собрать имя файла или таблицу");

    for field in fields {
        let shape_geometry = shapefile::Shape::try_from(field.geometry.clone())
            .expect("не получилось достать шейп из контура");

        let polygon =
            Polygon::try_from(shape_geometry).expect("не получилось достать полигон из шейпа");
        let record = FieldRecord::from_geo_record(field);
        writer
            .write_shape_and_record(&polygon, &record)
            .expect("не получилось добавить запись в таблицу");
    }
}

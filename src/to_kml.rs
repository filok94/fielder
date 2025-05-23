use geo::Geometry;
use kml::{self, types::Element, Kml};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};
pub fn json_with_wkt_to_kml() {
    #[derive(Deserialize)]
    struct Field {
        name: String,
        area_etalon_hectare: f32,
        superfield_name: String,
        #[serde(deserialize_with = "wkt::deserialize_wkt")]
        contour: Geometry,
    }
    fn get_fields_from_json() -> Result<String, io::Error> {
        fs::read_to_string("data/j.json")
    }

    let fields_json = get_fields_from_json().expect("Ошибка десериализации");
    let fields: Vec<Field> = serde_json::from_str(&fields_json).expect("Не смог собрать поля");

    let mut vector_of_placemarks = Vec::new();
    for field in fields {
        let kml_geometry = kml::types::Geometry::from(field.contour.clone());
        let sf_name_tag = Element {
            name: String::from("sf_name"),
            content: Some(field.superfield_name.clone()),
            attrs: HashMap::new(),
            children: Vec::new(),
        };
        let f_name_tag = Element {
            name: String::from("f_name"),
            content: Some(field.name),
            attrs: HashMap::new(),
            children: Vec::new(),
        };
        let area_tag = Element {
            name: String::from("area"),
            content: Some(field.area_etalon_hectare.to_string()),
            attrs: HashMap::new(),
            children: Vec::new(),
        };
        let kml_placemark = Kml::Placemark(kml::types::Placemark {
            name: Some(field.superfield_name),
            description: None,
            geometry: Some(kml_geometry),
            style_url: None,
            attrs: HashMap::new(),
            children: vec![sf_name_tag, f_name_tag, area_tag],
        });
        vector_of_placemarks.push(kml_placemark);
    }

    let document = Kml::KmlDocument(kml::types::KmlDocument {
        version: kml::KmlVersion::V23,
        attrs: HashMap::new(),
        elements: vector_of_placemarks,
    });

    let mut buf = Vec::new();
    kml::KmlWriter::from_writer(&mut buf)
        .write(&document)
        .expect("ошибка в записи файла");
    fs::File::create("foot.kml")
        .expect("не получилось создать файл")
        .write_all(&buf)
        .expect("не получилось записать файл");
}

use std::{fs, path::PathBuf};

use chrono::{self, Duration, NaiveDateTime};
use geo::{LineString, Simplify};
use serde::{Serialize, Serializer};
use uuid::Uuid;
use wkt::TryFromWkt;

#[derive(Serialize)]
struct FullJson {
    device_id: Uuid,
    track: Vec<TrackPoint>,
}

#[derive(Serialize)]
struct TrackPoint {
    lat: f64,
    lon: f64,
    #[serde(serialize_with = "serialize")]
    timestamp: NaiveDateTime,
}
const FORMAT: &str = "%Y-%m-%dT%H:%M:%S.%f";
const MAX_POINTS: usize = 10000;
pub fn generate_json_from_linestring(
    begin_time_as_string: &String,
    linestring_as_string: &Option<String>,
    terminal: &Uuid,
    path_to_linestring: &Option<PathBuf>,
    simplifying_ratio: &Option<f64>,
    seconds_between_points: &Option<i64>,
) {
    let startdatetime = chrono::NaiveDateTime::parse_from_str(begin_time_as_string, FORMAT)
        .expect("Не смог распарсить время");
    let wkt_linestring: LineString<f64>;
    if let Some(v) = linestring_as_string {
        wkt_linestring =
            LineString::try_from_wkt_str(v).expect("Не смог распарсить wkt из переданной строки");
    } else if let Some(v) = path_to_linestring {
        let wkt_string_from_file = fs::read_to_string(v).expect("Не смог достать файл");
        wkt_linestring = LineString::try_from_wkt_str(&wkt_string_from_file)
            .expect("Не смог распарсить wkt из переданного файла");
    } else {
        panic!("Не передан ни один формат linestring, передай или путь до файла или сам linestring")
    }
    let mut ratio: &f64 = &0.0;
    if let Some(r) = simplifying_ratio {
        ratio = r
    };
    let seconds: &i64;
    if let Some(s) = seconds_between_points {
        seconds = &s
    } else {
        seconds = &(30 as i64)
    };
    let mut points: Vec<TrackPoint> = vec![];
    for (index, p) in wkt_linestring.simplify(ratio).points().enumerate() {
        let point_time = startdatetime + Duration::seconds(seconds * index as i64);
        points.push(TrackPoint {
            lat: p.y(),
            lon: p.x(),
            timestamp: point_time,
        });
    }
    if points.len() > MAX_POINTS {
        panic!(
            "❌ Количество точек {}, разререшно меньше {}",
            points.len(),
            MAX_POINTS
        )
    };

    println!("количество точек: {}", points.len());
    let final_json = FullJson {
        device_id: terminal.to_owned(),
        track: points,
    };
    let json =
        serde_json::to_string_pretty(&final_json).expect("Не получилось сделать жсон строку");
    fs::write("new_json.json", json).expect("Не получилось создать файл");
}

fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

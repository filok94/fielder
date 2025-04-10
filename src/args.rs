use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, long_about = None, display_name="abobus")]
pub struct Cli {
    /// принимает файл, в котором будут находиться записи для работы
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    ///  шейпфайл из геожсона
    Shape,
    /// кмл из жсона
    Kml,
    /// трек для мониторинга
    Tracker(AddArgsToTracker),
}

#[derive(Args)]
pub struct AddArgsToTracker {
    /// Дата и время первой точки
    #[arg(short = 'b', long = "begin")]
    pub begin: String,
    /// Трэк из точек в формате Linestring WKT
    #[arg(short = 'l', long = "ln")]
    pub linestring: Option<String>,
    /// Гуид терминала
    #[arg(short = 't', long = "terminal")]
    pub terminal: Uuid,
    /// Путь до файла, в котором лежит linestring в WKT формате
    #[arg(short = 'p', long = "linepath")]
    pub linepath: Option<PathBuf>,
    /// На сколько упростить геометрию. Используется float 0.0001
    #[arg(short = 's', long = "simple")]
    pub simplifying_ration: Option<f64>,
    /// Секунд между точками
    #[arg(short = 'c', long = "seconds")]
    pub seconds: Option<i64>,
}

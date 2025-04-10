use args::Commands;
use clap::Parser;
use monitoring::generate_json_from_linestring;
use to_kml::json_with_wkt_to_kml;
use to_shapefile::write_shapefile;

pub mod args;
pub mod monitoring;
pub mod to_kml;
pub mod to_shapefile;

fn main() {
    let cli = args::Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Shape) => {
            if let Some(i) = cli.config {
                write_shapefile(i);
            }
        }
        Some(Commands::Kml) => {
            json_with_wkt_to_kml();
        }
        Some(Commands::Tracker(args)) => {
            generate_json_from_linestring(
                &args.begin,
                &args.linestring,
                &args.terminal,
                &args.linepath,
                &args.simplifying_ration,
                &args.seconds,
            );
        }
        None => {}
    }
}

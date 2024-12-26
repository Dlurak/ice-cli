mod consts;
mod series;
mod trip;

use clap::{Parser, Subcommand};
use consts::UNFETCHABLE_ERROR;
use iceportal::ICEPortal;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Default)]
enum Command {
    #[default]
    Status,
    Trip,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let cmd = args.command.unwrap_or(Command::default());

    match cmd {
        Command::Status => {
            let response = ICEPortal::fetch_status().await.expect(UNFETCHABLE_ERROR);
            let series = series::Series::new(response.series);
            let speed = response.speed;
            println!(
                "{}(BR {})\n{speed}km/h",
                series.name().unwrap(),
                String::from(&series)
            );
        }
        Command::Trip => {
            trip::handle_trip().await;
        }
    }
}

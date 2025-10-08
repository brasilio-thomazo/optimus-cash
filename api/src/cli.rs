use clap::Parser;

use crate::{app, db, http};

#[derive(clap::Subcommand)]
enum Commands {
    Server,
    Migrate,
    Seed,
}

#[derive(clap::Parser)]
#[command(name = "api")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub async fn run(pool: &db::Pool) -> Result<(), app::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server => http::run(pool.clone())
            .await
            .map_err(app::Error::io_error)?,
        Commands::Migrate => db::run_migration(&pool)
            .await
            .map_err(app::Error::sqlx_error)?,
        Commands::Seed => db::run_seed(&pool.write)
            .await
            .map_err(app::Error::sqlx_error)?,
    }
    Ok(())
}

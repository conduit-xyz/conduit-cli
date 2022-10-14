// CLI
use clap::{CommandFactory, Parser};
use clap_complete::generate;

/*
use std::fs::File
use dirs
use config::Config
*/

use exfac::{
    api::ExFac,
    handler,
    opts::{Opts, Subcommands},
    utils,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // dotenv::dotenv()?;
    handler::install()?;
    utils::subscriber();
    utils::enable_paint();

    let mut opts = Opts::parse();
    if opts.api.api_key.is_empty() && !matches!(opts.sub, Subcommands::Login(_)) {
        opts.api.api_key = match std::fs::read_to_string(exfac::config_dir().join("api-key")) {
            Ok(key) => key,
            Err(_) => eyre::bail!("No API Key found. Either login via `conduit login` or provide `--api-key` (or set via env var `API_KEY`)")
        };
    }
    tracing::debug!(?opts);
    let exfac = ExFac::new(opts.api);

    /*
    // Now get the default organization
    let mut home_dir = dirs::home_dir()
    .unwrap();

    home_dir::push(".conduit/config".to_owned());
    let mut config_file_path = home_dir.into_os_string().into_string().unwrap();

    let mut fileExists = File::open(config_file_path);
    if fileExists.is_err() {
        File::create(config_file_path)?;
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file_path));

    let mut defaultOrganization = settings.get_string("organization".to_owned());
    if defaultOrganization.is_err() {
        // Fetch and save the default organization
        let user = Subcommands::User::UserArgs.run(exfac).await?
        // Pick the first org as the default org
        defaultOrganization = user.organizations[0].organization
        let file = File::open(config_file_path)?

    }
    */

    match opts.sub {
        Subcommands::Completions { shell } => {
            generate(shell, &mut Opts::command(), "exfac", &mut std::io::stdout())
        }
        Subcommands::Network(args) => args.run(exfac).await?,
        Subcommands::User(args) => args.run(exfac).await?,
        Subcommands::JobTemplate(args) => args.run(exfac).await?,
        Subcommands::Job(args) => args.run(exfac).await?,
        Subcommands::Login(args) => args.run().await?,
    }

    Ok(())
}

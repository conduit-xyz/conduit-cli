use clap::Parser;

#[derive(Clone, Debug, Parser)]
/// Parameters for auth'ing and connecting to the ExFac API.
pub struct ExFacOpts {
    // TODO: Remove default value.
    #[clap(long, short, default_value = "b11bcdda-c67b-4fb1-b920-641dc1b52643")]
    /// Your ExFac API key.
    pub api_key: String,
    #[clap(long, short, default_value = "http://localhost:8080")]
    /// The URL pointing to the ExFac API.
    pub url: String,
}

impl ExFacOpts {
    // Returns the network slug.
    pub fn network(&self) -> String {
        format!("{}/v1/testnet", self.url)
    }

    // Returns the user slug.
    pub fn user(&self) -> String {
        format!("{}/v1/user", self.url)
    }
}

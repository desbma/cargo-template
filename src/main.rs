//! placeholder

use anyhow::Context as _;
use clap::Parser as _;

mod cl;
mod config;

fn main() -> anyhow::Result<()> {
    // Init logger
    simple_logger::SimpleLogger::new()
        .with_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .env()
        .init()
        .context("Failed to setup logger")?;

    // Parse CL args
    let _cl_args = cl::Args::parse();

    // Parse config
    let _cfg = config::parse()?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn a_test() {
        let v = true;
        assert!(v);
    }
}

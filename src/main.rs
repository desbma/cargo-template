//! {{project_description}}

#![cfg_attr(
    feature = "gen-man-pages",
    expect(dead_code, unused_crate_dependencies, unused_imports)
)]

use anyhow::Context as _;
use clap::Parser as _;

mod cl;
mod config;

#[cfg(feature = "gen-man-pages")]
fn main() -> anyhow::Result<()> {
    use clap::CommandFactory as _;
    let cmd = cl::Args::command();
    let output = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Missing output dir argument"))?;
    clap_mangen::generate_to(cmd, output)?;
    Ok(())
}

#[cfg(not(feature = "gen-man-pages"))]
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

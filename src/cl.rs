//! Command line interface

use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct Args {}

use archiver::{archiver_info, ArchiverOpts};
use clap::Parser;
use cli::*;
use cli::{RunMode, ToteError};
use extractor::{extractor_info, ExtractorOpts};

mod archiver;
mod cli;
mod extractor;
mod format;
mod verboser;

fn main() {
    let _opts = Cli0pts::parse();
}


#[cfg(test)]
mod tests {
    use super::*;
    use cli::RunMode;
    use std::path::PathBuf;

    #[test]
    fn test_run() {
        let opts = CliOpts::parse_from(&[
            "totebag_test",
            "-o",
            "test.zip",
            "src",
            "LICENSE",
            "README.md",
            "Cargo.toml",
        ]);
        assert_eq!(opts.mode, RunMode::Auto);
        assert_eq!(opts.output, Some(PathBuf::from("test.zip")));
        assert_eq!(opts.args.len(), 4);
        assert_eq!(
            opts.args,
            vec![
                PathBuf::from("src"),
                PathBuf::from("LICENSE"),
                PathBuf::from("README.md"),
                PathBuf::from("Cargo.toml")
            ]
        );
    }
}

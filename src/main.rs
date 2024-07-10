use archiver::{archiver_info, ArchiverOpts};
use clap::Parser;
use cli::*;
use cli::{RunMode, RError};
use extractor::{extractor_info, ExtractorOpts};

mod archiver;
mod cli;
mod extractor;
mod format;
mod verboser;

fn perform(mut opts: CliOpts) -> Result<()> {
    match opts.run_mode() {
        Ok(RunMode::Archive) => return perform_archive(opts),
        Ok(RunMode::Extract) => return perform_extract(opts),
        Ok(RunMode::List) => return perform_list(opts),
        Ok(RunMode::Auto) => {
            return Err(R_Error::Unknown(
                "cannot distinguish archiving and extracting".to_string(),
            ))
        }
        Err(e) => {
            return Err(e);
        }
    };
}

fn perform_extract(opts: CliOpts) -> Result<()> {
    let args = opts.args.clone();
    let extract_opts = ExtractorOpts::new(&opts);
    for arg in args.iter() {
        let extractor = extractor::create_extractor(arg).unwrap();
        let target = arg.to_path_buf();
        extract_opts
            .v
            .verbose(extractor_info(&extractor, &target, &extract_opts));
        extractor.perform(target, &extract_opts)?;
    }
    Ok(())
}

fn perform_list(opts: CliOpts) -> Result<()> {
    let args = opts.args.clone();
    for arg in args.iter() {
        if !arg.exists() {
            return Err(R_Error::FileNotFound(arg.to_path_buf()));
        }
        let extractor = extractor::create_extractor(&arg).unwrap();
        if args.len() > 1 {
            println!("========== {:?} ========== \n", arg);
        }
        let files = extractor.list_archives(arg.to_path_buf()).unwrap();
        for file in files.iter() {
            println!("{}", file);
        }
    }
    Ok(())
}

fn perform_archive(opts: CliOpts) -> Result<()> {
    let inout = ArchiverOpts::new(&opts);
    match archiver::create_archiver(&opts.output.unwrap()) {
        Ok(archiver) => {
            inout.v.verbose(archiver_info(&archiver, &inout));
            archiver.perform(&inout)
        }
        Err(e) => Err(e),
    } 
}

fn main() -> Result<()> {
    match perform(CliOpts::parse()) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                R_Error::NoArgumentsGiven => {
                    println!("No arguments given. Use --help for usage.")
                }
                R_Error::FileNotFound(p) => println!("{}: file not found", p.to_str().unwrap()),
                R_Error::FileExists(p) => {
                    println!("{}: file already exists", p.to_str().unwrap())
                }
                R_Error::IO(e) => println!("IO error: {}", e),
                R_Error::IOError(e) => println!("IO error: {}", e),
                R_Error::Archiver(s) => println!("Archive error: {}", s),
                R_Error::ArchiverError(s) => println!("Archive error: {}", s),
                R_Error::UnknownFormat(f) => println!("{}: unknown format", f),
                R_Error::UnsupportedFormat(f) => println!("{}: unsupported format", f),
                R_Error::Fatal(e) => println!("Error: {}", e),
                R_Error::Unknown(s) => println!("Unknown error: {}", s),
            }
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli::RunMode;
    use std::path::PathBuf;

    #[test]
    fn test_run() {
        let opts = CliOpts::parse_from(&[
            "RUM_test",
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
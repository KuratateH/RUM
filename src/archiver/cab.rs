// use crate::format::Format;
// use crate::archiver::{Archiver, ArchiverOpts};
// use crate::cli::{RError, Result};
// use std::fs::File;
// use std::io::BufWriter;
// use cab::{CabinetBuilder, CompressionType};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::cli::{Result, RError};
use crate::archiver::ArchiverOpts;





pub(super) struct CABArchiver {}

impl Archiver for CABArchiver {
    fn perform(&self, opts: &ArchiverOpts) -> Result<()> {
        let dest = &opts.dest;
        let targets = &opts.targets;

        // Open destination file
        let file = File::create(dest).map_err(RError::IOError)?;
        let writer = BufWriter::new(file);

        let mut builder = CabinetBuilder::new();

        for target in targets {
            let target_path = target.as_path();
            if target_path.is_file() {
                let mut file = File::open(target_path).map_err(RError::IOError)?;
                builder.add_folder(CompressionType::None).add_file(target_path.to_str().unwrap());
            } else if target_path.is_dir() && opts.recursive {
                for entry in std::fs::read_dir(target_path).map_err(RError::IOError)? {
                    let entry = entry.map_err(RError::IOError)?;
                    let path = entry.path();
                    if path.is_file() {
                        let mut file = File::open(&path).map_err(RError::IOError)?;
                       
                        builder.add_folder(CompressionType::None).add_file(path.to_str().unwrap().to_string());
                    }
                }
            }
        }

        builder.build(writer).map_err(RError::IOError)?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::CAB
    }
}



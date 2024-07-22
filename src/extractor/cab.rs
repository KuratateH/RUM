use std::fs::File;
//use std::io::{BufReader, BufWriter, Read, Write};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use cab::Cabinet;
use crate::cli::{Result, RError};
use crate::extractor::{Extractor, ExtractorOpts};
use crate::format::Format;
//use std::io;

pub struct CABExtractor {}

impl Extractor for CABExtractor {
    fn list_archives(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        let file = File::open(archive_file).map_err(RError::IOError)?;
        let cab = Cabinet::new(BufReader::new(file)).map_err(|_| RError::ExtractError("Failed to read CAB file".into()))?;

        let mut files = Vec::new();
        for folder in cab.folder_entries() {
            for file_entry in folder.file_entries() {
                files.push(file_entry.name().to_string());
            }
        }

        Ok(files)
    }

    fn perform(&self, archive_file: PathBuf, opts: &ExtractorOpts) -> Result<()> {
        let file = File::open(&archive_file).map_err(RError::IOError)?;
        let mut cab = Cabinet::new(BufReader::new(file)).map_err(|_| RError::ExtractError("Failed to read CAB file".into()))?;

        let dest = &opts.dest;

        // Extract file names first
        let file_names: Vec<String> = cab.folder_entries()
            .flat_map(|folder| folder.file_entries().map(|file_entry| file_entry.name().to_string()))
            .collect();

        for file_name in file_names {
            let output_path = dest.join(&file_name);

            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent).map_err(RError::IOError)?;
            }

            let mut reader = cab.read_file(&file_name).map_err(|err| RError::IOError(err))?;
            let mut writer = File::create(&output_path).map_err(RError::IOError)?;
            io::copy(&mut reader, &mut writer).map_err(RError::IOError)?;
        }

        // let cab_file = File::open(&archive_file).unwrap();
        // let mut cabinet = cab::Cabinet::new(cab_file).unwrap();
        // let dest = &opts.dest;
        
        // for folder in cabinet.folder_entries() {
        //     for file in folder.file_entries() {
        //         let file_name = file.name();
        //         let output_path = dest.join(file_name);

        //         let mut reader = cabinet.read_file(file_name).unwrap();
        //         let mut writer = File::create(output_path).unwrap();
        //         io::copy(&mut reader, &mut writer).unwrap();
        //     }
        // }
        
        // let file = File::open(&archive_file).map_err(RError::IOError)?;
        // let mut cab = Cabinet::new(BufReader::new(file)).map_err(|_| RError::ExtractError("Failed to read CAB file".into()))?;

        // let dest = &opts.dest;

        // for folder in cab.folder_entries() {
        //     for file_entry in folder.file_entries() {
        //         let file_name = file_entry.name();
        //         let output_path = dest.join(file_name);

        //         if let Some(parent) = output_path.parent() {
        //             std::fs::create_dir_all(parent).map_err(RError::IOError)?;
        //         }

        //         // `Cabinet` is re-borrowed mutably within a separate scope
        //         {
        //             let mut output_file = File::create(&output_path).map_err(RError::IOError)?;
        //             let mut buffer = Vec::new();
        //             let mut reader = cab.read_file(file_entry.name()).map_err(|err| RError::IOError(err))?;
        //             reader.read_to_end(&mut buffer).map_err(|err| RError::IOError(err))?;
        //             output_file.write_all(&buffer).map_err(RError::IOError)?;
        //         }
        //     }
        // }

        Ok(())
    }

    fn format(&self) -> Format {
        Format::CAB
    }
}

// RError に From<std::io::Error> を実装する
impl From<std::io::Error> for RError {
    fn from(error: std::io::Error) -> Self {
        RError::IOError(error)
    }
}

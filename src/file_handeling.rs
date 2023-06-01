use std::io::Write;
use std::fs::read;
use tokio::fs;
use aws_sdk_lambda as lambda;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Python,
    Go,
    Nodejs
}


pub fn convert_contents_to_blob(filename: &str) -> Result<lambda::primitives::Blob, Box<dyn std::error::Error>> {
    let file_contents = read(filename)?;
    let blob = lambda::primitives::Blob::new(file_contents);
    Ok(blob)
}


pub(crate) fn file_detection(filename: &str) -> FileType {
    match filename {
        filename if filename.contains(".py") => FileType::Python,
        filename if filename.contains(".go") => FileType::Go,
        filename if filename.contains(".js") => FileType::Nodejs,
        _ => panic!("Sadly we do not support that filetype yet.")
    }
}

pub fn zip_file(filename:&str, file_type: FileType) -> std::io::Result<()> {
    let mut zip = zip::ZipWriter::new(std::fs::File::create("deployment.zip")?);
    let file_contents = std::fs::read_to_string(filename)?;
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(filename, options)?;

    zip.write_all(file_contents.as_bytes())?;
    zip.finish()?;
    Ok(())
}

pub fn remove_file(file_type: FileType){

    match file_type{
        FileType::Python=>{

        }
        _ => {panic!("File not recognised")}
    }
}


#[test]
fn test_file_detection() {
    assert_eq!(file_detection("test.py"), FileType::Python);
    assert_eq!(file_detection("test.go"), FileType::Go);
    assert_eq!(file_detection("test.js"), FileType::Nodejs);

}

#[test]
#[should_panic]
fn test_file_detection_panic() {
    file_detection("test.rs");
    panic!("This program should panic as the filetype is not supported yet.");
}

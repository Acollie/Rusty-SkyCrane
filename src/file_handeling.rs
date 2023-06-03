use std::io::Write;
use std::fs::{read};
use tokio::fs;
use aws_sdk_lambda as lambda;
use std::process::Command;

#[derive(Debug, PartialEq, Clone, Copy)]
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

pub fn zip_file(filename:&str,file_type:FileType) -> std::io::Result<()> {
    return match file_type {
        FileType::Go => {
            Command::new("zsh").arg("-c").arg("GOOS=linux GOARCH=amd64 go build -o main ").arg(filename).output().expect("failed to execute process");
            Command::new("zsh").arg("-c").arg("zip deployment.zip main").output().expect("Failed to zip program");
            Ok(())
        }
        FileType::Nodejs => {
            Command::new("zsh").arg("-c").arg("zip deployment.zip index.js").output().expect("Failed to zip program");
            Ok(())
        }

        FileType::Python => {
            let mut zip = zip::ZipWriter::new(std::fs::File::create("deployment.zip")?);
            let file_contents = std::fs::read_to_string(filename)?;
            let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
            zip.start_file(filename, options)?;

            zip.write_all(file_contents.as_bytes())?;
            zip.finish()?;
            Ok(())
        }
    }
}

pub fn post_deployment_cleanup(file_type: FileType)->std::io::Result<()>{

    match file_type {
        FileType::Python=>{
            fs::remove_file("deployment.zip");
            Ok(())
        }
        FileType::Go=>{
            fs::remove_file("deployment.zip");
            Ok(())
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
}
#[test]
#[should_panic]
fn test_remove_unsupported_file(){
    post_deployment_cleanup(FileType::Nodejs).expect( "This should fail");
}

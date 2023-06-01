mod file_handeling;
mod upload_handeling;
mod handle_role;
mod lambda_handeling;

use std::collections::HashSet;
use aws_sdk_lambda as lambda;
use crate::file_handeling::{FileType, post_deployment_cleanup};
use aws_sdk_lambda::types::Runtime;
use aws_sdk_lambda::types::FunctionCode;
use crate::lambda::types::FullDocument::Default;
use crate::file_handeling::convert_contents_to_blob;
use std::fs::read;
use crate::lambda_handeling::get_lambda_names;

fn resolve_runtime(file_type: FileType) -> lambda::types::Runtime {
    match file_type {
        FileType::Python => lambda::types::Runtime::Python38,
        FileType::Go => lambda::types::Runtime::Go1x,
        FileType::Nodejs => lambda::types::Runtime::Nodejs18x,
    }
}


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{

    let args = std::env::args().collect::<Vec<String>>();
    let config = aws_config::load_from_env().await;
    let client = lambda::Client::new(&config);



    let filename = &args[1];
    let function_name = &args[2];
    let service_role = &args[3];
    let file_type = file_handeling::file_detection(filename);


    post_deployment_cleanup(file_type).unwrap();

    Ok(())
}

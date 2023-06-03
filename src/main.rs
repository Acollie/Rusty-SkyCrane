mod file_handeling;
mod upload_handeling;
mod handle_role;
mod lambda_handeling;

use aws_sdk_lambda as lambda;
use crate::file_handeling::{FileType, post_deployment_cleanup};
use crate::upload_handeling::{lambda_upload};

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
    let functions_names = lambda_handeling::get_lambda_names(&client).await;

    lambda_upload(&functions_names, service_role, &client, filename, file_type, function_name).await.expect("TODO: panic message");

    post_deployment_cleanup(file_type).unwrap();

    Ok(())
}

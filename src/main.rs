mod reading_file;
mod upload_handeling;
mod handle_role;

use std::collections::HashSet;
use aws_sdk_lambda as lambda;
use crate::reading_file::FileType;
use aws_sdk_lambda::types::Runtime;
use aws_sdk_lambda::types::FunctionCode;
use crate::lambda::types::FullDocument::Default;
use crate::reading_file::convert_contents_to_blob;
use std::fs::read;

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

    let resp = client.list_functions().send().await?;


    let functions = resp.functions().unwrap_or_default();
    let mut functions_name:HashSet<String> = HashSet::new();

    for function in functions {
        functions_name.insert(function.function_name().unwrap().to_string());
    }

    let filename = &args[1];
    let function_name = &args[2];
    let service_role = &args[3];
    let file_type = reading_file::file_detection(filename);

    let file_contents = read("deployment.zip")?;
    let blob = lambda::primitives::Blob::new(file_contents);
    let function_code = FunctionCode::builder()
        .zip_file(blob)
        .build();

    let runtime = resolve_runtime(file_type);
    if functions_name.contains(function_name){
        client.update_function_code().function_name(function_name)
            .zip_file(convert_contents_to_blob("deployment.zip").unwrap()).send()
            .await?;
    }else {
        client.create_function()
            .function_name(function_name)
            .handler(filename.to_owned() + ".handler")
            .runtime(runtime)
            .role(service_role)
            .code(function_code)
            .send()
            .await?;
    }
    let file_type = reading_file::file_detection(filename);

    reading_file::zip_file(filename, file_type);
    Ok(())
}

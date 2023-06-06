use std::collections::HashSet;
use std::fs::read;
use aws_sdk_lambda::types::{FunctionCode};
use crate::file_handling::{convert_contents_to_blob, FileType};
use aws_sdk_lambda as lambda;
use aws_sdk_lambda::Client;
use crate::{file_handling, resolve_runtime};

pub async fn lambda_upload(lambda_functions:&HashSet<String>,service_role:&str,client:&Client,filename:&str,file_type:FileType,function_name:&str)->Result<(),()>{
    file_handling::zip_file(filename, file_type).unwrap();
    let file_contents = read("deployment.zip").unwrap();
    let blob = lambda::primitives::Blob::new(file_contents);
    let function_code = FunctionCode::builder()
        .zip_file(blob)
        .build();

    file_handling::zip_file(filename, file_type).unwrap();
    let file_without_extension = filename.split(".").next().unwrap();


    let runtime = resolve_runtime(file_type);
    if cfg!(test) {
        return Ok(())
    }
    if lambda_functions.contains(function_name){
        client.update_function_code().function_name(function_name)
            .zip_file(convert_contents_to_blob("deployment.zip").unwrap()).send()
            .await.unwrap();
    }else {
        client.create_function()
            .function_name(function_name)
            .handler(file_without_extension.to_owned() + ".handler")
            .runtime(runtime)
            .role(service_role)
            .code(function_code)
            .send()
            .await.unwrap();
    }
    Ok(())
}

#[tokio::test]
async fn test_lambda_upload(){
    let mut lambda_functions = HashSet::new();
    lambda_functions.insert("test".to_string());
    let config = aws_config::load_from_env().await;
    let client = lambda::Client::new(&config);
    let result = lambda_upload(&lambda_functions,"test",&client,"test.py",FileType::Python,"test").await;
    assert_eq!(result.is_ok(),true);
}
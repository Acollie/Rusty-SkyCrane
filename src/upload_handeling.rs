use std::fs::read;
use aws_sdk_lambda::types::FunctionCode;
use crate::{file_handeling, resolve_runtime};
use crate::file_handeling::convert_contents_to_blob;

pub async fn python_uploading(){

    let file_contents = read("deployment.zip")?;
    let blob = lambda::primitives::Blob::new(file_contents);
    let function_code = FunctionCode::builder()
        .zip_file(blob)
        .build();

    file_handeling::zip_file(filename);
    let runtime = resolve_runtime(file_type);
    if functions_name.await.contains(function_name){
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
}
mod file_handling;
mod upload_handling;
mod handle_role;
mod lambda_handling;

use aws_sdk_lambda as lambda;
use crate::file_handling::{FileType, post_deployment_cleanup};
use crate::upload_handling::{lambda_upload};

fn resolve_runtime(file_type: FileType) -> lambda::types::Runtime {
    match file_type {
        FileType::Python => lambda::types::Runtime::Python38,
        FileType::Go => lambda::types::Runtime::Go1x,
        FileType::Nodejs => lambda::types::Runtime::Nodejs18x,
    }
}
fn handle_inputs(args:Vec<String>)->(String,String,String){
    let filename = &args[1];
    let function_name = &args[2];
    let service_role = &args[3];
    return (filename.to_string(),function_name.to_string(),service_role.to_string())
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{

    let (filename,function_name,service_role) = handle_inputs(std::env::args().collect());
    let config = aws_config::load_from_env().await;
    let client = lambda::Client::new(&config);



    let file_type = file_handling::file_detection(filename.as_str());
    let functions_names = lambda_handling::get_lambda_names(&client).await;

    lambda_upload(&functions_names, service_role.as_str(), &client, filename.as_str(), file_type, function_name.as_str()).await.expect("TODO: panic message");

    post_deployment_cleanup(file_type).unwrap();

    Ok(())
}

#[test]
fn test_handle_inputs(){
    let args = vec!["initial".to_string(),"filename".to_string(),"function_name".to_string(),"service_role".to_string()];
    let (filename,function_name,service_role) = handle_inputs(args);
    assert_eq!(filename,"filename");
    assert_eq!(function_name,"function_name");
    assert_eq!(service_role,"service_role");
}

#[test]
#[should_panic]
fn test_handle_inputs_panic(){
    let args = vec!["initial".to_string(),"test".to_string()];
    let (filename,function_name,service_role) = handle_inputs(args);
    assert_eq!(filename,"test");
    assert_eq!(function_name,"test");
    assert_eq!(service_role,"test");
}

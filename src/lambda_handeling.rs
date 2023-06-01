use std::collections::HashSet;
use aws_sdk_lambda::Client;

pub async fn get_lambda_names(client:&Client)->HashSet<String>{

    let resp = client.list_functions().send().await.unwrap();
    let functions = resp.functions().unwrap_or_default();
    let mut functions_names:HashSet<String> = HashSet::new();

    for function in functions {
        functions_names.insert(function.function_name().unwrap().to_string());
    }
    return functions_names
}
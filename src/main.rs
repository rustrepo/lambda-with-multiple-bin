use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::env;
use dotenv::dotenv;
mod vector;
mod vector2;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code examples in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    dotenv().ok();
   
    let table_name = env::var("TEST")?;
    println!("Message from Env is = {}", table_name);

    vector::demonstrate_vector_operations();
    vector::iterating_vector();
    vector2::struct_vector();

    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
        
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

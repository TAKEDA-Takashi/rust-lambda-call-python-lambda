use lamedh_runtime::{Context, Error};
use log::{debug, info};
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use serde::Deserialize;
use serde_json::Value;
use std::env;

#[derive(Debug, Deserialize)]
struct TestData {
    name: String,
    code: u32,
    tags: Option<String>,
    lang: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "rust_lambda_call_python_lambda=debug");
    env_logger::init();

    lamedh_runtime::run(lamedh_runtime::handler_fn(handler)).await?;

    Ok(())
}

async fn handler(_: Value, _: Context) -> Result<(), Error> {
    debug!("handler start");

    let lambda_client = LambdaClient::new(Default::default());

    let res = lambda_client
        .invoke(InvocationRequest {
            function_name: "s3-select-sample-python".into(),
            ..Default::default()
        })
        .await?;

    let text = unwrap_string_quote(String::from_utf8(res.payload.unwrap().to_vec())?);

    debug!("{:?}", text);

    for line in text.lines() {
        let d: TestData = serde_json::from_str(line)?;
        info!("{:?}", d);
    }

    Ok(())
}

fn unwrap_string_quote(s: String) -> String {
    let mut s: String = s.replace("\\\"", "\"").replace("\\n", "\n");
    s.remove(0);
    s.remove(s.len() - 1);
    s
}

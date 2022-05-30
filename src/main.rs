use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos/1?userdId=1")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", todos);
    Ok(())
}

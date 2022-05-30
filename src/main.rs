use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userdId=1")
        .send()
        .await?
        .json()
        .await?;

    let sent_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&todos[4])
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", sent_todo);
    println!("{:#?}", &todos[4]);

    Ok(())
}

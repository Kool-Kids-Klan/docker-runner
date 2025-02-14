mod docker_api;
use docker_api::client::{get_docker_client, list_images, pull_image};

#[tokio::main]
async fn main() {
    let docker_client = get_docker_client().unwrap();
    let ping_res = docker_client.ping().await.unwrap();
    println!("{}", ping_res);
    let images = list_images(&docker_client).await.unwrap();
    for image in images {
        println!("-> {:?}", image);
    }

    let pull_logs = pull_image(&docker_client, "nginx", "latest").await.unwrap();
    for log in pull_logs {
        println!("-> {:?}", log);
    }
}

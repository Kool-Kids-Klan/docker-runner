mod docker_api;
use docker_api::client::{get_docker_client, list_images, pull_image, run_container};

#[tokio::main]
async fn main() {
    let docker_client = get_docker_client().unwrap();
    let ping_res = docker_client.ping().await.unwrap();
    println!("{}", ping_res);
    let images = list_images(&docker_client).await.unwrap();
    for image in images {
        println!("-> {:?}", image);
    }

    let image_name = "ubuntu";
    let pull_logs = pull_image(&docker_client, image_name, "latest")
        .await
        .unwrap();
    for log in pull_logs {
        println!("-> {:?}", log);
    }

    let container_name = "my-contianer";
    let created_container = run_container(&docker_client, container_name, image_name)
        .await
        .unwrap();
    println!("-> {:?}", created_container);
}

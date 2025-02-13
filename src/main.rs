use bollard::Docker;

#[tokio::main]
async fn main() {
    let docker_client = Docker::connect_with_socket_defaults().unwrap();
    let ping_res = docker_client.ping().await.unwrap();
    println!("{}", ping_res);
}

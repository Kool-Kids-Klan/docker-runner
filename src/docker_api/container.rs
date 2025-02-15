use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        WaitContainerOptions,
    },
    errors::Error,
    secret::ContainerCreateResponse,
    Docker,
};
use futures_util::StreamExt;

pub async fn run_container(
    client: &Docker,
    container_name: &str,
    image_name: &str,
) -> Result<ContainerCreateResponse, Error> {
    let options = Some(CreateContainerOptions {
        name: container_name,
        platform: None,
    });

    let config = Config {
        image: Some(image_name),
        cmd: Some(vec!["sleep", "10"]),
        ..Default::default()
    };

    let created_container = client.create_container(options, config).await?;

    client
        .start_container(container_name, None::<StartContainerOptions<String>>)
        .await?;

    let options = Some(WaitContainerOptions {
        condition: "not-running",
    });
    let mut wait_stream = client.wait_container(container_name, options);
    while let Some(log) = wait_stream.next().await {
        println!("-> Waiting for container to finish: {:?}", log?);
    }

    client
        .remove_container(container_name, None::<RemoveContainerOptions>)
        .await?;
    return Ok(created_container);
}

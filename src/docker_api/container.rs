use bollard::{
    container::{
        AttachContainerOptions, Config, CreateContainerOptions, RemoveContainerOptions,
        StartContainerOptions, WaitContainerOptions,
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
        ..Default::default()
    };

    let created_container = client.create_container(options, config).await?;

    client
        .start_container(container_name, None::<StartContainerOptions<String>>)
        .await?;

    read_output(client, container_name).await?;

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

pub async fn read_output(client: &Docker, container_name: &str) -> Result<(), Error> {
    let attach_options = Some(AttachContainerOptions::<String> {
        stdout: Some(true),
        logs: Some(true),
        stream: Some(true),
        ..Default::default()
    });

    let mut attached_container = client
        .attach_container(container_name, attach_options)
        .await?;

    while let Some(log) = attached_container.output.next().await {
        println!("-> {:?}", log?);
    }

    return Ok(());
}

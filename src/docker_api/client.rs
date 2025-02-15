use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        WaitContainerOptions,
    },
    errors::Error,
    image::{CreateImageOptions, ListImagesOptions},
    secret::{ContainerCreateResponse, CreateImageInfo, ImageSummary},
    Docker,
};
use futures_util::{StreamExt, TryStreamExt};

pub fn get_docker_client() -> Result<Docker, Error> {
    return Docker::connect_with_socket_defaults();
}

pub async fn list_images(client: &Docker) -> Result<Vec<ImageSummary>, Error> {
    return client
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await;
}

pub async fn pull_image(
    client: &Docker,
    name: &str,
    tag: &str,
) -> Result<Vec<CreateImageInfo>, Error> {
    return client
        .create_image(
            Some(CreateImageOptions {
                from_image: name,
                tag,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect()
        .await;
}

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

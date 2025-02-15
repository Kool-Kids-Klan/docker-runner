use bollard::{
    errors::Error,
    image::{CreateImageOptions, ListImagesOptions},
    secret::{CreateImageInfo, ImageSummary},
    Docker,
};
use futures_util::TryStreamExt;

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

use bollard::{errors::Error, image::ListImagesOptions, secret::ImageSummary, Docker};

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

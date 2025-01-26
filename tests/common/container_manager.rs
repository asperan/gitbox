use hierrorchy::error_leaf;
use which::which;

#[error_leaf("Failed to find 'podman' or 'docker' in path")]
pub struct ContainerManagerNotFoundError {}

pub fn container_manager() -> Result<String, ContainerManagerNotFoundError> {
    Ok(match which("podman") {
        Ok(value) => value,
        Err(_) => match which("docker") {
            Ok(value) => value,
            Err(_) => return Err(ContainerManagerNotFoundError {}),
        },
    }
    .to_str()
    .expect("A binary path is a valid string")
    .to_owned())
}

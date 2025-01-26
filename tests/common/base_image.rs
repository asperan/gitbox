use std::error::Error;
use std::io::Error as IoError;

use hierrorchy::{error_leaf, error_node};
use indoc::formatdoc;

use super::container_manager::{container_manager, ContainerManagerNotFoundError};

#[error_leaf(format!("image build failed, stderr dump:\n{}", self.stderr))]
pub struct BuildImageCommandError {
    stderr: String,
}

error_node! {
    pub type DockerfileWriteError<IoError> = "failed to write dockerfile"
}

error_node! {
    pub type BaseImageBuildError<
        ContainerManagerNotFoundError,
        DockerfileWriteError,
        BuildImageCommandError
    > = "failed to build base image"
}

pub fn build_base_image(
    msrv: &str,
    bin_name: &str,
    required_packages: &[&str],
) -> Result<String, BaseImageBuildError> {
    let dockerfile_content = formatdoc! { "
        FROM docker.io/library/rust:{msrv}-slim AS builder
        WORKDIR /project
        COPY ./ ./
        RUN cargo build

        FROM docker.io/library/debian:12.9-slim AS runner
        {}
        COPY --from=builder /project/target/debug/{bin_name} /usr/local/bin/{bin_name}
    ",
    if required_packages.is_empty() {
        String::new()
    } else {
        format!("RUN apt-get update && apt-get install -y --no-install-recommends {}", required_packages.join(" "))
    }};
    let package_name = std::env::var("CARGO_PKG_NAME").expect("Package name is present and UTF-8");
    let image_name = format!("{package_name}-integration-tests-base:{msrv}");
    let tmp_dockerfile_path =
        std::env::temp_dir().join(format!("Dockerfile.{package_name}_integration_tests"));
    std::fs::write(&tmp_dockerfile_path, dockerfile_content)
        .map_err(DockerfileWriteError::from)?;
    let result = std::process::Command::new(&container_manager()?)
        .args([
            "build",
            "-t",
            &image_name,
            "-f",
            tmp_dockerfile_path
                .to_str()
                .expect("Temp dockerfile path exists"),
            ".",
        ])
        .output()
        .expect("Building the image does not fail");
    if !result.status.success() {
        return Err(BuildImageCommandError {
            stderr: std::str::from_utf8(&result.stderr)
                .expect("result stderr is a valid string")
                .to_owned(),
        }
        .into());
    }
    Ok(image_name)
}

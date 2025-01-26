use std::fs::Permissions;
use std::io::Error as IoError;
use std::os::unix::fs::PermissionsExt;
use std::{env::temp_dir, error::Error, fs};

use hierrorchy::{error_leaf, error_node};

use super::container_manager::{container_manager, ContainerManagerNotFoundError};

const PACKAGE_NAME: &str = std::env!("CARGO_PKG_NAME");
const INIT_PHASE_DELIMITER: &str = "===== INIT PHASE TERMINATED =====";

#[error_leaf(format!("test {} failed - stderr:\n{}\n----- END STDERR -----", self.test_name, self.stderr))]
pub struct IntegrationTestFailedError {
    test_name: String,
    stderr: String,
}

#[error_leaf(format!("test {} failed - expected output '{}' but actually it was '{}'", self.test_name, self.expected, self.actual))]
pub struct IntegrationTestAssertionError {
    test_name: String,
    expected: String,
    actual: String,
}

error_node! {
    pub type IntegrationTestRunError<IoError, ContainerManagerNotFoundError IntegrationTestFailedError,IntegrationTestAssertionError> = "integration test failed"
}

#[derive(Debug)]
pub struct IntegrationTest {
    name: String,
    base_image: String,
    init_commands: String,
    run_commands: String,
    expected_output: String,
}

impl IntegrationTest {
    pub fn new(
        name: &str,
        base_image: &str,
        init_commands: &str,
        run_commands: &str,
        expected_output: &str,
    ) -> Self {
        IntegrationTest {
            name: name.to_owned(),
            base_image: base_image.to_owned(),
            init_commands: init_commands.to_owned(),
            run_commands: run_commands.to_owned(),
            expected_output: expected_output.to_owned(),
        }
    }

    pub fn run(&self) -> Result<(), IntegrationTestRunError> {
        let container_name = format!("{}_integration-test_{}", PACKAGE_NAME, &self.name);
        let test_script_content = format!(
            "#!/bin/bash\n{}\necho '{}'\n{}",
            &self.init_commands, INIT_PHASE_DELIMITER, &self.run_commands
        );
        let test_script_path =
            temp_dir().join(&format!("test-script_{}_{}", PACKAGE_NAME, &self.name));
        fs::write(&test_script_path, test_script_content)?;
        fs::File::open(&test_script_path)?.set_permissions(Permissions::from_mode(0o755))?;
        let result = std::process::Command::new(&container_manager()?)
            .args([
                "run",
                "--rm",
                "-v",
                &format!(
                    "{}:/test_script",
                    &test_script_path
                        .to_str()
                        .expect("test script path is correct")
                ),
                "--name",
                &container_name,
                &self.base_image,
                "/test_script",
            ])
            .output()?;
        if result.status.success() {
            let output = std::str::from_utf8(&result.stdout).expect("stdout is a valid rust string").to_owned();
            let run_output = output.lines().skip_while(|it| it != &INIT_PHASE_DELIMITER).skip(1).map(|it| it.to_owned()).reduce(|acc, it| acc + "\n" + &it).unwrap_or_else(|| String::new());
            if run_output != self.expected_output {
                Err(IntegrationTestAssertionError { test_name: self.name.clone(), expected: self.expected_output.clone(), actual: run_output }.into())
            } else {
                Ok(())
            }
        } else {
            Err(IntegrationTestFailedError {
                test_name: self.name.clone(),
                stderr: std::str::from_utf8(&result.stderr)
                    .expect("Error is a valid rust string")
                    .to_owned(),
            }
            .into())
        }
    }
}

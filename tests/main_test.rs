use dogana::{
    dogana_images::DEBIAN_IMAGE,
    dogana_test::{DoganaTest, DoganaTestResult},
};

#[test]
fn test_gb_init() -> DoganaTestResult {
    DoganaTest::builder()
        .set_base_image(&DEBIAN_IMAGE)
        .set_init_commands(&["mkdir /tmp/workdir && cd /tmp/workdir", "git config --global user.name \"tester\" && git config --global user.email \"tester@example.org\""])
        .set_run_commands(&["gb init", "git log --pretty=%s"])
        .set_expected_output("Repository initialized successfully\nchore(init): initialize empty repository")
        .build()
        .run()
}

#[test]
fn test_describe_prerelease_in_empty_repository() -> DoganaTestResult {
    DoganaTest::builder()
        .set_base_image(&DEBIAN_IMAGE)
        .set_init_commands(&["mkdir /tmp/workdir && cd /tmp/workdir", "git config --global user.name \"tester\" && git config --global user.email \"tester@example.org\"", "gb init"])
        .set_run_commands(&["gb describe --prerelease", "git tag --list"])
        .set_expected_output("0.1.0-1")
        .build()
        .run()
}

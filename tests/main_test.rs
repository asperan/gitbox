use common::integration_test::IntegrationTest;

mod common;

const MSRV: &str = "1.83";
const BIN_NAME: &str = "gb";

#[test]
fn setup() {
    match common::base_image::build_base_image(MSRV, BIN_NAME, &["git"]) {
        Ok(image_name) => {
            let mut all_ok: bool = true;
            if let Err(e) = IntegrationTest::new(
                "gb-init",
                &image_name,
                "mkdir /tmp/workdir && cd /tmp/workdir\ngit config --global user.name \"tester\" && git config --global user.email \"tester@example.org\"",
                "gb init\ngit log --pretty=%s",
                "Repository initialized successfully\nchore(init): initialize empty repository",
            )
            .run()
            {
                eprintln!("{}", e);
                all_ok = false;
            }
            if let Err(e) = IntegrationTest::new(
                "gb-describe-prerelease-in-empty-repo",
                &image_name,
                "mkdir /tmp/workdir && cd /tmp/workdir\ngit config --global user.name \"tester\" && git config --global user.email \"tester@example.org\"\ngb init",
                "gb describe --prerelease\ngit tag --list",
                "0.1.0-1",
            )
            .run()
            {
                eprintln!("{}", e);
                all_ok = false;
            }
            assert!(all_ok);
        }
        Err(e) => panic!("Failed to build base image: {}", e),
    }
}

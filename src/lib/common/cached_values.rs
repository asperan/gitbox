use super::git::git_dir;

pub struct CachedValues {}

impl CachedValues {
    pub fn git_dir() -> &'static String {
        unsafe {
            match &GIT_DIR {
                Some(value) => &value,
                None => {
                    GIT_DIR = Some(git_dir());
                    GIT_DIR.as_ref().unwrap()
                },
            }
        }
    }
}

static mut GIT_DIR: Option<String> = None;

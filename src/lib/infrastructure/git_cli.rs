use std::{process::Command, str::FromStr};

use crate::{
    application::retriever::{
        commit_retriever::CommitRetriever, gitinfo_retriever::GitInfoRetriever,
        version_retriever::VersionRetriever,
    },
    domain::{semantic_version::SemanticVersion, type_aliases::AnyError},
};

use super::error::{command_execution_error::CommandExecutionError, generic_cli_error::CliError};

pub struct GitCli {}

impl GitCli {
    pub fn new() -> GitCli {
        GitCli {}
    }

    fn run_git_command<'a>(
        &self,
        args: impl Iterator<Item = &'a str> + Clone,
    ) -> Result<String, AnyError> {
        let execution_output = Command::new("git")
            .args(args.clone())
            .output()
            .map_err(|e| {
                CommandExecutionError::new(
                    &format!(
                        "{} {}",
                        "git",
                        args.fold(String::new(), |acc, x| acc.to_owned() + " " + x)
                    ),
                    Box::new(e),
                )
            })?;
        if execution_output.status.success() {
            Ok(std::str::from_utf8(&execution_output.stdout)?.to_string())
        } else {
            Err(Box::new(CliError::new(std::str::from_utf8(
                &execution_output.stderr,
            )?)))
        }
    }

    fn split_and_clean_commits(&self, list: String) -> Vec<String> {
        list.split("\n")
            .filter(|it| !it.is_empty())
            .map(|it| it.to_string())
            .collect()
    }
}

impl CommitRetriever for GitCli {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
        let log_list =
            self.run_git_command(vec!["log", "--pretty=format:%s", "--all"].into_iter())?;
        Ok(Box::new(self.split_and_clean_commits(log_list).into_iter()))
    }

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
        let mut args = vec!["log", "--pretty=format:%s"];
        let mut _s = String::new();
        if let Some(value) = version {
            _s = format!("^{}", value);
            args.push(&_s);
            args.push("HEAD");
        }
        let log_list = self.run_git_command(args.into_iter())?;
        Ok(Box::new(self.split_and_clean_commits(log_list).into_iter()))
    }
}

impl VersionRetriever for GitCli {
    fn last_version(&self) -> Result<Option<String>, AnyError> {
        let output = self.run_git_command(vec!["describe", "--tags", "--abbrev=0"].into_iter());
        match output {
            Ok(v) => Ok(Some(SemanticVersion::from_str(v.trim())?.to_string())),
            Err(e) => Err(e),
        }
    }

    fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
        let output =
            self.run_git_command(vec!["--no-pager", "tag", "--list", "--merged"].into_iter());
        match output {
            Ok(v) => {
                if v.trim().is_empty() {
                    Ok(None)
                } else {
                    Ok(v.trim()
                        .split("\n")
                        .filter_map(|it| SemanticVersion::from_str(it).ok())
                        .filter(|it| it.prerelease().is_none())
                        .max()
                        .map(|it| it.to_string()))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl GitInfoRetriever for GitCli {
    fn git_dir(&self) -> Result<String, AnyError> {
        self.run_git_command(vec!["rev-parse", "--absolute-git-dir"].into_iter())
    }
}
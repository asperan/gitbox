use std::{process::Command, str::FromStr};

use crate::{
    application::{
        manager::{
            conventional_commit_egress_manager::ConventionalCommitEgressManager, init_egress_manager::InitEgressManager,
            tag_egress_manager::TagEgressManager,
        },
        manager::{
            commit_metadata_ingress_manager::CommitMetadataIngressManager,
            bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
            full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
            gitinfo_ingress_manager::GitInfoIngressManager,
            version_ingress_manager::VersionIngressManager,
        },
    },
    domain::semantic_version::SemanticVersion,
    usecases::{metadata_spec::MetadataSpec, type_aliases::AnyError},
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
            Ok(std::str::from_utf8(&execution_output.stdout)?
                .trim()
                .to_string())
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

impl FullCommitSummaryHistoryIngressManager for GitCli {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
        let log_list =
            self.run_git_command(vec!["log", "--pretty=format:%s", "--all"].into_iter())?;
        Ok(Box::new(self.split_and_clean_commits(log_list).into_iter()))
    }
}

impl BoundedCommitSummaryIngressManager for GitCli {
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

impl VersionIngressManager for GitCli {
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

impl GitInfoIngressManager for GitCli {
    fn git_dir(&self) -> Result<String, AnyError> {
        self.run_git_command(vec!["rev-parse", "--absolute-git-dir"].into_iter())
    }
}

impl ConventionalCommitEgressManager for GitCli {
    fn create_commit(&self, commit: &str) -> Result<(), AnyError> {
        self.run_git_command(vec!["commit", "-m", commit].into_iter())
            .map(|_| ())
    }

    fn create_empty_commit(&self, commit: &str) -> Result<(), AnyError> {
        self.run_git_command(vec!["commit", "--allow-empty", "-m", commit].into_iter())
            .map(|_| ())
    }
}

impl InitEgressManager for GitCli {
    fn init_repository(&self) -> Result<(), AnyError> {
        self.run_git_command(vec!["init"].into_iter()).map(|_| ())
    }
}

impl TagEgressManager for GitCli {
    fn create_tag(
        &self,
        label: &str,
        message: &Option<String>,
        sign: bool,
    ) -> Result<(), AnyError> {
        let mut args = vec![label];
        args.push("-m");
        match message {
            Some(s) => {
                args.push(&s);
            }
            None => {
                args.push("");
            }
        }
        if sign {
            args.push("-s");
        }
        self.run_git_command(args.into_iter()).map(|_| ())
    }
}

impl CommitMetadataIngressManager for GitCli {
    fn get_metadata(&self, metadata_spec: &MetadataSpec) -> Result<String, AnyError> {
        match metadata_spec {
            MetadataSpec::Sha => {
                self.run_git_command(vec!["log", "-n", "1", "--pretty=format:%h"].into_iter())
            }
            MetadataSpec::Date => {
                self.run_git_command(vec!["log", "-n", "1", "--pretty=format:%as"].into_iter())
            }
        }
    }
}

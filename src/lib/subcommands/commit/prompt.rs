use regex::Regex;
use requestty::{prompt_one, Answer, Question};
use std::path::Path;

use crate::common::{
    cached_values::CachedValues,
    commons::{append_line, print_error_and_exit, read_lines, ensure_dir_exists},
    git::{DEFAULT_COMMIT_TYPES, EXTRA_DIR_PATH, TYPES_FILE_PATH, SCOPES_FILE_PATH},
};

pub struct Prompt {}

impl Prompt {
    pub fn ask_type() -> String {
        let available_types = Prompt::read_types();
        let raw_select = Question::raw_select("commit_type")
            .message("Choose the commit type:")
            .choices(&available_types)
            .choice("Create new type")
            .build();
        let answer = prompt_one(raw_select);
        let answer_index = match answer {
            Ok(Answer::ListItem(a)) => a.index,
            Ok(_) => panic!("Obtained a non ListItem from a raw_select"),
            Err(e) => print_error_and_exit(&e.to_string()),
        };
        if answer_index == available_types.len() {
            Prompt::ask_new_type()
        } else {
            available_types[answer_index].clone()
        }
    }

    pub fn ask_scope() -> String {
        let available_scopes = Prompt::read_scopes();
        let raw_select = Question::raw_select("scope")
            .message("Choose the scope:")
            .choices(&available_scopes)
            .choice("Create new scope")
            .choice("None")
            .build();
        let answer = prompt_one(raw_select);
        let answer_index = match answer {
            Ok(Answer::ListItem(a)) => a.index,
            Ok(_) => panic!("Obtained a non ListItem from a raw_select"),
            Err(e) => print_error_and_exit(&e.to_string()),
        };
        if answer_index == available_scopes.len() {
            Prompt::ask_new_scope()
        } else if answer_index == available_scopes.len() + 1 {
            String::from("")
        } else {
            available_scopes[answer_index].clone()
        }
    }

    pub fn ask_breaking() -> bool {
        let answer = prompt_one(
            Question::confirm("breaking")
                .message("Is this commit a breaking change?")
                .build(),
        );
        match answer {
            Ok(Answer::Bool(breaking)) => breaking,
            Ok(_) => panic!("Obtained a non Bool from a confirm"),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    }

    pub fn ask_summary() -> String {
        let answer = prompt_one(
            Question::input("summary")
                .message("Commit summary:")
                .validate(|s, _| {
                    if !s.is_empty() {
                        Ok(())
                    } else {
                        Err("The summary cannot be empty".to_owned())
                    }
                })
                .build(),
        );
        match answer {
            Ok(Answer::String(s)) => s,
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    }

    pub fn ask_body() -> String {
        let answer = prompt_one(
            Question::editor("body")
                .message("Insert the body of the commit message")
                .extension(".txt")
                .build(),
        );
        match answer {
            Ok(Answer::String(s)) => s,
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    }

    fn read_types() -> Vec<String> {
        Prompt::ensure_extra_dir_exists();
        let types_file_absolute_path =
            CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + TYPES_FILE_PATH;
        Prompt::read_values(
            &types_file_absolute_path,
            DEFAULT_COMMIT_TYPES.iter().map(|s| s.to_string()).collect(),
        )
    }

    fn read_scopes() -> Vec<String> {
        Prompt::ensure_extra_dir_exists();
        let scopes_file_absolute_path =
            CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + SCOPES_FILE_PATH;
        Prompt::read_values(&scopes_file_absolute_path, vec![])
    }

    fn ensure_extra_dir_exists() {
        let extra_dir_absolute_path = CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH;
        ensure_dir_exists(&extra_dir_absolute_path);
    }

    fn read_values(path: &str, default_values: Vec<String>) -> Vec<String> {
        let file_absolute_path = Path::new(path);
        match file_absolute_path.try_exists() {
            Ok(exists) => {
                if !exists {
                    let content = if default_values.is_empty() {
                        String::from("")
                    } else {
                        default_values.join("\n")
                    };
                    let write_result = std::fs::write(file_absolute_path, content);
                    if let Err(e) = write_result { eprintln!("Failed to write '{}': {}", path, e) }
                    default_values
                } else {
                    match read_lines(file_absolute_path) {
                        Ok(lines) => lines,
                        Err(e) => {
                            eprintln!(
                                "Failed to read file '{}': {}",
                                file_absolute_path.display(),
                                e
                            );
                            default_values
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: the file '{}' existence cannot be verified ({}). Fallback to only default types", path, e);
                default_values
            }
        }
    }

    fn ask_new_type() -> String {
        let type_regex = Regex::new(r"^[a-z]+$").unwrap();
        Prompt::ask_new_input(
            "type",
            type_regex,
            &(CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + TYPES_FILE_PATH),
            &|s| format!("\n{}", s),
        )
    }

    fn ask_new_scope() -> String {
        let scope_regex = Regex::new(r"^[a-z -]+$").unwrap();
        Prompt::ask_new_input(
            "scope",
            scope_regex,
            &(CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + SCOPES_FILE_PATH),
            &|s| format!("{}\n", s),
        )
    }

    fn ask_new_input(
        what: &str,
        valid_regex: Regex,
        path_to_append: &str,
        format_content: &dyn Fn(&str) -> String,
    ) -> String {
        let new_value = prompt_one(
            Question::input(format!("new-{}", what))
                .message(format!("New {}: ", what))
                .validate(|s, _| {
                    if valid_regex.is_match(s) {
                        Ok(())
                    } else {
                        Err(format!("{} not valid", what))
                    }
                })
                .build(),
        );
        match new_value {
            Ok(Answer::String(s)) => {
                match append_line(path_to_append, &format_content(&s)) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Failed to update {}s file: {}", what, e),
                }
                s
            }
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    }
}

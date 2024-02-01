use std::rc::Rc;

use regex::Regex;
use requestty::{prompt_one, Answer, Question};

use crate::usecases::type_aliases::AnyError;

use super::{
    gitextra_append_manager::GitExtraAppendManager, gitextra_read_manager::GitExtraReadManager,
};

pub struct PromptManager {
    gitextra_read_manager: Rc<dyn GitExtraReadManager>,
    gitextra_append_manager: Rc<dyn GitExtraAppendManager>,
}

impl PromptManager {
    pub fn new(
        gitextra_read_manager: Rc<dyn GitExtraReadManager>,
        gitextra_append_manager: Rc<dyn GitExtraAppendManager>,
    ) -> PromptManager {
        PromptManager {
            gitextra_read_manager,
            gitextra_append_manager,
        }
    }

    pub fn ask_type(&self) -> Result<String, AnyError> {
        let available_types = self.gitextra_read_manager.get_types()?;
        let raw_select = Question::raw_select("commit_type")
            .message("Choose the commit type:")
            .choices(&available_types)
            .choice("Create new type")
            .build();
        let answer = prompt_one(raw_select);
        let answer_index = match answer {
            Ok(Answer::ListItem(a)) => a.index,
            Ok(_) => panic!("Obtained a non ListItem from a raw_select"),
            Err(e) => return Err(Box::new(e)),
        };
        if answer_index == available_types.len() {
            self.ask_new_type()
        } else {
            Ok(available_types[answer_index].clone())
        }
    }

    pub fn ask_scope(&self) -> Result<Option<String>, AnyError> {
        let available_scopes = self.gitextra_read_manager.get_scopes()?;
        let raw_select = Question::raw_select("scope")
            .message("Choose the scope:")
            .choice("None")
            .choices(&available_scopes)
            .choice("Create new scope")
            .build();
        let answer = prompt_one(raw_select);
        let answer_index = match answer {
            Ok(Answer::ListItem(a)) => a.index,
            Ok(_) => panic!("Obtained a non ListItem from a raw_select"),
            Err(e) => return Err(Box::new(e)),
        };
        Ok(if answer_index == available_scopes.len() + 1 {
            Some(self.ask_new_scope()?)
        } else if answer_index == 0 {
            None
        } else {
            Some(available_scopes[answer_index - 1].clone())
        })
    }

    pub fn ask_breaking(&self) -> Result<bool, AnyError> {
        let answer = prompt_one(
            Question::confirm("breaking")
                .message("Is this commit a breaking change?")
                .build(),
        );
        match answer {
            Ok(Answer::Bool(breaking)) => Ok(breaking),
            Ok(_) => panic!("Obtained a non Bool from a confirm"),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn ask_summary(&self) -> Result<String, AnyError> {
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
            Ok(Answer::String(s)) => Ok(s),
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn ask_body(&self) -> Result<Option<String>, AnyError> {
        let answer = prompt_one(
            Question::editor("body")
                .message("Insert the body of the commit message")
                .extension(".txt")
                .build(),
        );
        match answer {
            Ok(Answer::String(s)) => Ok(if s.is_empty() { None } else { Some(s) }),
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn ask_new_type(&self) -> Result<String, AnyError> {
        let type_regex = Regex::new(r"^[a-z]+$").unwrap();
        let new_type = self.ask_new_input("type", type_regex)?;
        self.gitextra_append_manager.append_type(&new_type)?;
        Ok(new_type)
    }

    fn ask_new_scope(&self) -> Result<String, AnyError> {
        let scope_regex = Regex::new(r"^[a-z -]+$").unwrap();
        let new_scope = self.ask_new_input("scope", scope_regex)?;
        self.gitextra_append_manager.append_scope(&new_scope)?;
        Ok(new_scope)
    }

    fn ask_new_input(&self, what: &str, valid_regex: Regex) -> Result<String, AnyError> {
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
            Ok(Answer::String(s)) => Ok(s),
            Ok(_) => panic!("Obtained a non String from an input"),
            Err(e) => Err(Box::new(e)),
        }
    }
}

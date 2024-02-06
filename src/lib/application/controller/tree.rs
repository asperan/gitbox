use std::rc::Rc;

use regex::Regex;

use crate::{
    application::manager::{
        git_tree_ingress_manager::GitTreeIngressManager,
        message_egress_manager::MessageEgressManager,
    },
    usecases::type_aliases::AnyError,
};

use super::exit_code::ControllerExitCode;

const TREE_FORMAT: &str = "%C(bold blue)%h%C(reset)§%C(dim normal)(%cr)%C(reset)§%C(auto)%d%C(reset)§§%n§§§       %C(normal)%an%C(reset)%C(dim normal): %s%C(reset)";
const TIME_MINIMUM_PADDING: usize = 2;

pub struct TreeController {
    git_tree_ingress_manager: Rc<dyn GitTreeIngressManager>,
    message_egress_manager: Rc<dyn MessageEgressManager>,
}

impl TreeController {
    pub fn new(
        git_tree_ingress_manager: Rc<dyn GitTreeIngressManager>,
        message_egress_manager: Rc<dyn MessageEgressManager>,
    ) -> Self {
        TreeController {
            git_tree_ingress_manager,
            message_egress_manager,
        }
    }

    pub fn commit_tree(&self) -> ControllerExitCode {
        match self.run() {
            Ok(_) => ControllerExitCode::Ok,
            Err(e) => {
                self.message_egress_manager
                    .error(&format!("Failed to print commit tree: {}", e));
                ControllerExitCode::Error(1)
            }
        }
    }

    fn run(&self) -> Result<(), AnyError> {
        let binding = self.git_tree_ingress_manager.commit_tree(TREE_FORMAT)?;
        let lines: Box<[Box<[&str]>]> = binding
            .iter()
            .map(|it| it.split('§').collect::<Box<[&str]>>())
            .collect();
        let time_color_length = {
            let time_regex = Regex::new("\\([a-z0-9 ,]+\\)").unwrap();
            let first_line_time = &lines[0][1];
            let captures = time_regex
                .captures(&first_line_time)
                .expect("The first line always contains time reference");
            first_line_time.len() - captures.get(0).expect("the groups should be there").len()
        };
        let time_padding = {
            lines
                .iter()
                .filter_map(|it| {
                    if it.len() > 1 && !it[1].is_empty() {
                        Some(it[1].len() - time_color_length)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0usize)
        };
        lines
            .iter()
            .map(|line| {
                let left_padding = TIME_MINIMUM_PADDING
                    + time_padding
                    + if line.len() > 1 && !line[1].is_empty() {
                        time_color_length
                    } else {
                        0
                    };
                self.format_line(line, left_padding)
            })
            .for_each(|it| self.message_egress_manager.output(&it));
        Ok(())
    }

    #[inline(always)]
    fn format_line(&self, line: &Box<[&str]>, left_padding: usize) -> Box<str> {
        let line_len = line.len();
        format!(
            "{date:>width$} {tree_mark} {pointers} {commit_text}",
            date = if line_len < 2 { "" } else { line[1] },
            width = left_padding,
            tree_mark = line[0],
            pointers = if line_len < 3 { "" } else { line[2] },
            commit_text = if line_len < 4 { "" } else { line[3] },
        )
        .into()
    }
}

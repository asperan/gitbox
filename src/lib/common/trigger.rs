mod change_nodes;

use self::change_nodes::{
    BreakingNode, FirstAndValue, FirstOrValue, ScopeNode, SecondAndValue, SecondOrValue,
};
use crate::common::commons::print_error_and_exit;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use self::change_nodes::{
    AndStatement, ArrayNode, BasicStatement, InNode, ObjectNode, OrStatement, PriorityStatement,
    Start, TypeNode, Visitable,
};

#[derive(Debug)]
pub struct Trigger {
    start_node: Start,
}

impl Trigger {
    fn new(start_node: Start) -> Trigger {
        Trigger { start_node }
    }

    pub fn from(dsl: &str) -> Trigger {
        ChangeTriggerParser::run(dsl)
    }

    pub fn accept(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        self.start_node.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug, Parser)]
#[grammar = "lib/common/trigger/trigger-grammar.pest"]
struct ChangeTriggerParser {}

impl ChangeTriggerParser {
    fn run(dsl: &str) -> Trigger {
        let parse_result = Self::parse(Rule::START, dsl);
        #[cfg(debug_assertions)]
        dbg!(&parse_result);
        Trigger::new(match parse_result {
            Ok(mut v) => ChangeTriggerParser::parse_start(v.next().unwrap()),
            Err(e) => print_error_and_exit(&ChangeTriggerParser::format_rules(e).to_string()),
        })
    }

    fn format_rules(error: Error<Rule>) -> Error<Rule> {
        error.renamed_rules(|rule| {
            match rule {
                Rule::BREAKING_STMT => "'breaking'",
                Rule::SCOPE_OBJECT => "'scope'",
                Rule::TYPE_OBJECT => "'type'",
                Rule::ARRAY_STMT => "'type/scope IN [ _ ]'",
                Rule::WHITESPACE => "whitespace",
                Rule::PAR_STMT => "statement with parenthesis '( _ )'",
                Rule::LITERAL => "literal",
                Rule::OBJECT => "'type'/'scope'",
                Rule::ARRAY => "'[ _ ]'",
                Rule::STMT => "'breaking'/'type/scope IN [ _ ]'",
                Rule::OR_STMT => "OR statement '(_ OR _)'",
                Rule::AND_STMT => "AND statement '(_ AND _)'",
                Rule::EOI => "End Of Input",
                Rule::START => "Main statement",
            }
            .to_string()
        })
    }

    fn parse_start(token: Pair<Rule>) -> Start {
        match &token.as_rule() {
            Rule::AND_STMT => Start::And(Self::parse_and(token.into_inner())),
            Rule::OR_STMT => Start::Or(Self::parse_or(token.into_inner())),
            Rule::STMT => Start::Basic(Self::parse_basic(token.into_inner().next().unwrap())),
            _ => unreachable!(),
        }
    }

    fn parse_and(mut tokens: Pairs<Rule>) -> AndStatement {
        let lhs = tokens.next().unwrap();
        let left_node = match lhs.as_rule() {
            Rule::PAR_STMT => FirstAndValue::Priority(Box::new(Self::parse_priority(
                lhs.into_inner().next().unwrap(),
            ))),
            Rule::STMT => FirstAndValue::Basic(Self::parse_basic(lhs.into_inner().next().unwrap())),
            _ => unreachable!(),
        };
        let rhs = tokens.next().unwrap();
        let right_node = match rhs.as_rule() {
            Rule::AND_STMT => SecondAndValue::And(Box::new(Self::parse_and(rhs.into_inner()))),
            Rule::PAR_STMT => SecondAndValue::Priority(Box::new(Self::parse_priority(
                rhs.into_inner().next().unwrap(),
            ))),
            Rule::STMT => {
                SecondAndValue::Basic(Self::parse_basic(rhs.into_inner().next().unwrap()))
            }
            _ => unreachable!(),
        };
        AndStatement {
            left: left_node,
            right: right_node,
        }
    }

    fn parse_priority(token: Pair<Rule>) -> PriorityStatement {
        match token.as_rule() {
            Rule::OR_STMT => PriorityStatement {
                internal_node: Self::parse_or(token.into_inner()),
            },
            _ => unreachable!(),
        }
    }

    fn parse_or(mut tokens: Pairs<Rule>) -> OrStatement {
        let lhs = tokens.next().unwrap();
        let left_node = match lhs.as_rule() {
            Rule::AND_STMT => FirstOrValue::And(Self::parse_and(lhs.into_inner())),
            Rule::STMT => FirstOrValue::Basic(Self::parse_basic(lhs.into_inner().next().unwrap())),
            _ => unreachable!(),
        };
        let rhs = tokens.next().unwrap();
        let right_node = match rhs.as_rule() {
            Rule::AND_STMT => SecondOrValue::And(Self::parse_and(rhs.into_inner())),
            Rule::OR_STMT => SecondOrValue::Or(Box::new(Self::parse_or(rhs.into_inner()))),
            Rule::STMT => SecondOrValue::Basic(Self::parse_basic(rhs.into_inner().next().unwrap())),
            _ => unreachable!(),
        };
        OrStatement {
            left: left_node,
            right: right_node,
        }
    }

    fn parse_basic(token: Pair<Rule>) -> BasicStatement {
        match &token.as_rule() {
            Rule::BREAKING_STMT => BasicStatement::Breaking(BreakingNode {}),
            Rule::ARRAY_STMT => BasicStatement::In(Self::parse_in(token.into_inner())),
            _ => unreachable!(),
        }
    }

    fn parse_in(mut tokens: Pairs<Rule>) -> InNode {
        let object_node = Self::parse_object(tokens.next().unwrap());
        let array_node = Self::parse_array(tokens.next().unwrap());
        InNode {
            object: object_node,
            array: array_node,
        }
    }

    fn parse_object(token: Pair<Rule>) -> ObjectNode {
        let inner_token = token.into_inner().next().unwrap();
        match inner_token.as_rule() {
            Rule::TYPE_OBJECT => ObjectNode::Type(TypeNode {}),
            Rule::SCOPE_OBJECT => ObjectNode::Scope(ScopeNode {}),
            _ => unreachable!(),
        }
    }

    fn parse_array(token: Pair<Rule>) -> ArrayNode {
        ArrayNode {
            values: token.into_inner().map(|t| t.as_str().to_string()).collect(),
        }
    }
}

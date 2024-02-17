/*
A Trigger can be seen as a function which takes the type, the optional scope and the breaking-ness
of a [CommitSummary] and outputs a boolean value, i.e. whether the trigger has been activated by the
commit (or, the commit is accepted by the Trigger).

Triggers can be used for checking whether a commit procs a major, minor or patch increase; or
if it must be hidden in a changelog.
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Trigger {
    start_node: Start,
}

impl Trigger {
    pub fn new(start_node: Start) -> Trigger {
        Trigger { start_node }
    }

    pub fn accept(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        self.start_node.visit(commit_type, scope, breaking)
    }
}

trait Visitable<'a, T> {
    fn visit(&self, commit_type: &str, scope: Option<&'a str>, breaking: bool) -> T;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeNode {}
impl Visitable<'_, String> for TypeNode {
    fn visit(&self, commit_type: &str, _scope: Option<&str>, _breaking: bool) -> String {
        commit_type.to_owned()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeNode {}
impl<'a> Visitable<'a, Option<&'a str>> for ScopeNode {
    fn visit(
        &self,
        _commit_type: &str,
        scope: Option<&'a str>,
        _breaking: bool,
    ) -> Option<&'a str> {
        scope
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectNode {
    Scope(ScopeNode),
    Type(TypeNode),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BreakingNode {}
impl Visitable<'_, bool> for BreakingNode {
    fn visit(&self, _commit_type: &str, _scope: Option<&str>, breaking: bool) -> bool {
        breaking
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiteralNode {
    value: String,
}
impl Visitable<'_, String> for LiteralNode {
    fn visit(
        &self,
        _commit_type: &str,
        _scope: Option<&str>,
        _breaking: bool,
    ) -> std::string::String {
        self.value.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayNode {
    pub values: Vec<String>,
}
impl Visitable<'_, Vec<String>> for ArrayNode {
    fn visit(&self, _commit_type: &str, _scope: Option<&str>, _breaking: bool) -> Vec<String> {
        self.values.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InNode {
    pub object: ObjectNode,
    pub array: ArrayNode,
}
impl Visitable<'_, bool> for InNode {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match &self.object {
            ObjectNode::Type(type_node) => self
                .array
                .visit(commit_type, scope, breaking)
                .contains(&type_node.visit(commit_type, scope, breaking)),
            ObjectNode::Scope(scope_node) => match scope_node.visit(commit_type, scope, breaking) {
                Some(this_scope) => self
                    .array
                    .visit(commit_type, scope, breaking)
                    .contains(&this_scope.to_owned()),
                None => false,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BasicStatement {
    In(InNode),
    Breaking(BreakingNode),
}

impl Visitable<'_, bool> for BasicStatement {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            BasicStatement::In(n) => n.visit(commit_type, scope, breaking),
            BasicStatement::Breaking(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FirstAndValue {
    Basic(BasicStatement),
    Priority(Box<PriorityStatement>),
}

impl Visitable<'_, bool> for FirstAndValue {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            FirstAndValue::Basic(node) => node.visit(commit_type, scope, breaking),
            FirstAndValue::Priority(node) => node.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SecondAndValue {
    Basic(BasicStatement),
    Priority(Box<PriorityStatement>),
    And(Box<AndStatement>),
}

impl Visitable<'_, bool> for SecondAndValue {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Priority(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AndStatement {
    pub left: FirstAndValue,
    pub right: SecondAndValue,
}

impl Visitable<'_, bool> for AndStatement {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking)
            && self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FirstOrValue {
    And(AndStatement),
    Basic(BasicStatement),
}

impl Visitable<'_, bool> for FirstOrValue {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SecondOrValue {
    And(AndStatement),
    Basic(BasicStatement),
    Or(Box<OrStatement>),
}

impl Visitable<'_, bool> for SecondOrValue {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
            Self::Or(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrStatement {
    pub left: FirstOrValue,
    pub right: SecondOrValue,
}

impl Visitable<'_, bool> for OrStatement {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking)
            || self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PriorityStatement {
    pub internal_node: OrStatement,
}

impl Visitable<'_, bool> for PriorityStatement {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        self.internal_node.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Start {
    And(AndStatement),
    Or(OrStatement),
    Basic(BasicStatement),
}

impl Visitable<'_, bool> for Start {
    fn visit(&self, commit_type: &str, scope: Option<&str>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Or(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::trigger::{
        AndStatement, BasicStatement, FirstAndValue, FirstOrValue, OrStatement, PriorityStatement,
        SecondAndValue, SecondOrValue, Start,
    };

    use super::{
        ArrayNode, BreakingNode, InNode, LiteralNode, ObjectNode, ScopeNode, TypeNode, Visitable,
    };

    type TestValue = (&'static str, Option<&'static str>, bool);

    const TEST_VALUES1: TestValue = ("type", Some("scope"), true);
    const TEST_VALUES2: TestValue = ("type", None, true);
    const TEST_VALUES3: TestValue = ("type", None, false);

    #[test]
    fn type_node() {
        let n = TypeNode {};
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            TEST_VALUES1.0
        );
    }

    #[test]
    fn scope_node_empty() {
        let n = ScopeNode {};
        assert_eq!(
            n.visit(TEST_VALUES2.0, TEST_VALUES2.1, TEST_VALUES2.2),
            None
        );
    }

    #[test]
    fn scope_node() {
        let n = ScopeNode {};
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            Some("scope")
        );
    }

    #[test]
    fn breaking_node() {
        let n = BreakingNode {};
        assert!(n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2));
    }

    #[test]
    fn not_breaking_node() {
        let n = BreakingNode {};
        assert!(!n.visit(TEST_VALUES3.0, TEST_VALUES3.1, TEST_VALUES3.2));
    }

    #[test]
    fn literal_node() {
        let n = LiteralNode {
            value: "literal".to_string(),
        };
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            "literal".to_string()
        );
    }

    #[test]
    fn array_node() {
        let n = ArrayNode {
            values: vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string(),
            ],
        };
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string()
            ]
        );
    }

    // InNode
    #[test]
    fn in_node_contains_type() {
        let n = InNode {
            object: ObjectNode::Type(TypeNode {}),
            array: ArrayNode {
                values: vec!["type".to_string()],
            },
        };
        assert!(n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2));
    }

    #[test]
    fn in_node_not_contains_type() {
        let n = InNode {
            object: ObjectNode::Type(TypeNode {}),
            array: ArrayNode {
                values: vec!["test".to_string()],
            },
        };
        assert!(!n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2));
    }

    #[test]
    fn in_node_contains_scope() {
        let n = InNode {
            object: ObjectNode::Scope(ScopeNode {}),
            array: ArrayNode {
                values: vec!["scope".to_string()],
            },
        };
        assert!(n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2));
    }

    #[test]
    fn in_node_not_contains_scope() {
        let n = InNode {
            object: ObjectNode::Scope(ScopeNode {}),
            array: ArrayNode {
                values: vec!["test".to_string()],
            },
        };
        assert!(!n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2));
    }

    #[test]
    fn in_node_empty_scope() {
        let n = InNode {
            object: ObjectNode::Scope(ScopeNode {}),
            array: ArrayNode {
                values: vec!["test".to_string()],
            },
        };
        assert!(!n.visit(TEST_VALUES2.0, TEST_VALUES2.1, TEST_VALUES2.2));
    }

    // BasicStatement
    #[test]
    fn basic_statement_in_node() {
        let n = InNode {
            object: ObjectNode::Scope(ScopeNode {}),
            array: ArrayNode {
                values: vec!["test".to_string()],
            },
        };
        let s = BasicStatement::In(n.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn basic_statement_breaking_node() {
        let n = BreakingNode {};
        let s = BasicStatement::Breaking(n.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    // FirstAndValue
    #[test]
    fn first_and_value_basic() {
        let s = BasicStatement::Breaking(BreakingNode {});
        let n = FirstAndValue::Basic(s.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn first_and_value_priority() {
        let s = PriorityStatement {
            internal_node: OrStatement {
                left: FirstOrValue::And(AndStatement {
                    left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
                    right: SecondAndValue::Basic(BasicStatement::In(InNode {
                        object: ObjectNode::Type(TypeNode {}),
                        array: ArrayNode {
                            values: vec!["test".to_string()],
                        },
                    })),
                }),
                right: SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            },
        };
        let n = FirstAndValue::Priority(Box::new(s.clone()));
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // SecondAndValue
    #[test]
    fn second_and_value_basic() {
        let s = BasicStatement::Breaking(BreakingNode {});
        let n = SecondAndValue::Basic(s.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn second_and_value_priority() {
        let s = PriorityStatement {
            internal_node: OrStatement {
                left: FirstOrValue::And(AndStatement {
                    left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
                    right: SecondAndValue::Basic(BasicStatement::In(InNode {
                        object: ObjectNode::Type(TypeNode {}),
                        array: ArrayNode {
                            values: vec!["test".to_string()],
                        },
                    })),
                }),
                right: SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            },
        };
        let n = SecondAndValue::Priority(Box::new(s.clone()));
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn second_and_value_and() {
        let s = AndStatement {
            left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            right: SecondAndValue::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["test".to_string()],
                },
            })),
        };
        let v = SecondAndValue::And(Box::new(s.clone()));
        assert_eq!(
            v.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // AndStatement
    #[test]
    fn and_statement() {
        let a1 = FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {}));
        let a2 = SecondAndValue::Basic(BasicStatement::In(InNode {
            object: ObjectNode::Type(TypeNode {}),
            array: ArrayNode {
                values: vec!["test".to_string()],
            },
        }));
        let s = AndStatement {
            left: a1.clone(),
            right: a2.clone(),
        };

        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            a1.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
                && a2.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // FirstOrValue
    #[test]
    fn first_or_value_basic() {
        let s = BasicStatement::Breaking(BreakingNode {});
        let n = FirstOrValue::Basic(s.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn first_or_value_and() {
        let s = AndStatement {
            left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            right: SecondAndValue::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["test".to_string()],
                },
            })),
        };
        let v = FirstOrValue::And(s.clone());
        assert_eq!(
            v.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // SecondOrValue
    #[test]
    fn second_or_value_basic() {
        let s = BasicStatement::Breaking(BreakingNode {});
        let n = SecondOrValue::Basic(s.clone());
        assert_eq!(
            n.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    #[test]
    fn second_or_value_and() {
        let s = AndStatement {
            left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            right: SecondAndValue::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["test".to_string()],
                },
            })),
        };
        let v = SecondOrValue::And(s.clone());
        assert_eq!(
            v.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    #[test]
    fn second_or_value_or() {
        let s = OrStatement {
            left: FirstOrValue::And(AndStatement {
                left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
                right: SecondAndValue::Basic(BasicStatement::In(InNode {
                    object: ObjectNode::Type(TypeNode {}),
                    array: ArrayNode {
                        values: vec!["test".to_string()],
                    },
                })),
            }),
            right: SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {})),
        };
        let v = SecondOrValue::Or(Box::new(s.clone()));
        assert_eq!(
            v.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // OrStatement
    #[test]
    fn or_statement() {
        let o1 = FirstOrValue::And(AndStatement {
            left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            right: SecondAndValue::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["test".to_string()],
                },
            })),
        });

        let o2 = SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {}));
        let s = OrStatement {
            left: o1.clone(),
            right: o2.clone(),
        };
        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            o1.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
                || o2.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    // PriorityStatement
    #[test]
    fn priority_statement() {
        let s = OrStatement {
            left: FirstOrValue::And(AndStatement {
                left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
                right: SecondAndValue::Basic(BasicStatement::In(InNode {
                    object: ObjectNode::Type(TypeNode {}),
                    array: ArrayNode {
                        values: vec!["test".to_string()],
                    },
                })),
            }),
            right: SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {})),
        };
        let p = PriorityStatement {
            internal_node: s.clone(),
        };

        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            p.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
    // Start
    #[test]
    fn start_and() {
        let a = AndStatement {
            left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
            right: SecondAndValue::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["test".to_string()],
                },
            })),
        };
        let s = Start::And(a.clone());
        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            a.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn start_or() {
        let a = OrStatement {
            left: FirstOrValue::And(AndStatement {
                left: FirstAndValue::Basic(BasicStatement::Breaking(BreakingNode {})),
                right: SecondAndValue::Basic(BasicStatement::In(InNode {
                    object: ObjectNode::Type(TypeNode {}),
                    array: ArrayNode {
                        values: vec!["test".to_string()],
                    },
                })),
            }),
            right: SecondOrValue::Basic(BasicStatement::Breaking(BreakingNode {})),
        };
        let s = Start::Or(a.clone());
        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            a.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }

    #[test]
    fn start_basic() {
        let b = BasicStatement::Breaking(BreakingNode {});
        let s = Start::Basic(b.clone());
        assert_eq!(
            s.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2),
            b.visit(TEST_VALUES1.0, TEST_VALUES1.1, TEST_VALUES1.2)
        );
    }
}

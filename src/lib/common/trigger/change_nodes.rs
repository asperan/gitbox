pub(super) trait Visitable<'a, T> {
    fn visit(&self, commit_type: &str, scope: &'a Option<String>, breaking: bool) -> T;
}

#[derive(Debug)]
pub(super) struct TypeNode {}
impl Visitable<'_, String> for TypeNode {
    fn visit(&self, commit_type: &str, _scope: &Option<String>, _breaking: bool) -> String {
        commit_type.to_owned()
    }
}

#[derive(Debug)]
pub(super) struct ScopeNode {}
impl<'a> Visitable<'a, &'a Option<String>> for ScopeNode {
    fn visit(&self, _commit_type: &str, scope: &'a Option<String>, _breaking: bool) -> &'a Option<String> {
        scope
    }
}

#[derive(Debug)]
pub(super) enum ObjectNode {
    Scope(ScopeNode),
    Type(TypeNode),
}

#[derive(Debug)]
pub(super) struct BreakingNode {}
impl Visitable<'_, bool> for BreakingNode {
    fn visit(&self, _commit_type: &str, _scope: &Option<String>, breaking: bool) -> bool {
        breaking
    }
}

#[derive(Debug)]
pub(super) struct LiteralNode {
    value: String,
}
impl Visitable<'_, String> for LiteralNode {
    fn visit(&self, _commit_type: &str, _scope: &Option<String>, _breaking: bool) -> std::string::String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub(super) struct ArrayNode {
    pub values: Vec<String>,
}
impl Visitable<'_, Vec<String>> for ArrayNode {
    fn visit(&self, _commit_type: &str, _scope: &Option<String>, _breaking: bool) -> Vec<String> {
        self.values.clone()
    }
}

#[derive(Debug)]
pub(super) struct InNode {
    pub object: ObjectNode,
    pub array: ArrayNode,
}
impl Visitable<'_, bool> for InNode {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match &self.object {
            ObjectNode::Type(type_node) => self.array.visit(commit_type, scope, breaking).contains(&type_node.visit(commit_type, scope, breaking)),
            ObjectNode::Scope(scope_node) => match scope_node.visit(commit_type, scope, breaking) {
                Some(this_scope) => self.array.visit(commit_type, scope, breaking).contains(this_scope),
                None => false,
            },

        }
    }
}

#[derive(Debug)]
pub(super) enum BasicStatement {
    In(InNode),
    Breaking(BreakingNode),
}

impl Visitable<'_, bool> for BasicStatement {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            BasicStatement::In(n) => n.visit(commit_type, scope, breaking),
            BasicStatement::Breaking(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) enum FirstAndValue {
    Basic(BasicStatement),
    Priority(Box<PriorityStatement>),
}

impl Visitable<'_, bool> for FirstAndValue {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            FirstAndValue::Basic(node) => node.visit(commit_type, scope, breaking),
            FirstAndValue::Priority(node) => node.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) enum SecondAndValue {
    Basic(BasicStatement),
    Priority(Box<PriorityStatement>),
    And(Box<AndStatement>),
}

impl Visitable<'_, bool> for SecondAndValue {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Priority(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) struct AndStatement {
    pub left: FirstAndValue,
    pub right: SecondAndValue,
}

impl Visitable<'_, bool> for AndStatement {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking) && self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) enum FirstOrValue {
    And(AndStatement),
    Basic(BasicStatement),
}

impl Visitable<'_, bool> for FirstOrValue {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) enum SecondOrValue {
    And(AndStatement),
    Basic(BasicStatement),
    Or(Box<OrStatement>),
}

impl Visitable<'_, bool> for SecondOrValue {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
            Self::Or(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) struct OrStatement {
    pub left: FirstOrValue,
    pub right: SecondOrValue,
}

impl Visitable<'_, bool> for OrStatement {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking) || self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) struct PriorityStatement {
    pub internal_node: OrStatement,
}

impl Visitable<'_, bool> for PriorityStatement {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        self.internal_node.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) enum Start {
    And(AndStatement),
    Or(OrStatement),
    Basic(BasicStatement),
}

impl Visitable<'_, bool> for Start {
    fn visit(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Or(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
        }
    }
}


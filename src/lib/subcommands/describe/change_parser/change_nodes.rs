pub(super) trait Visitable<T> {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> T;
}

#[derive(Debug)]
pub(super) struct TypeNode {}
impl Visitable<String> for TypeNode {
    fn visit(&self, commit_type: &str, _scope: &str, _breaking: bool) -> String {
        commit_type.to_owned()
    }
}

#[derive(Debug)]
pub(super) struct ScopeNode {}
impl Visitable<String> for ScopeNode {
    fn visit(&self, _commit_type: &str, scope: &str, _breaking: bool) -> String {
        scope.to_owned()
    }
}

#[derive(Debug)]
pub(super) enum ObjectNode {
    Scope(ScopeNode),
    Type(TypeNode),
}

impl Visitable<String> for ObjectNode {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> String {
        match self {
            ObjectNode::Scope(n) => n.visit(commit_type, scope, breaking),
            ObjectNode::Type(n) => n.visit(commit_type, scope, breaking),
        }
    }
}

#[derive(Debug)]
pub(super) struct BreakingNode {}
impl Visitable<bool> for BreakingNode {
    fn visit(&self, _commit_type: &str, _scope: &str, breaking: bool) -> bool {
        breaking
    }
}

#[derive(Debug)]
pub(super) struct LiteralNode {
    value: String,
}
impl Visitable<String> for LiteralNode {
    fn visit(&self, _commit_type: &str, _scope: &str, _breaking: bool) -> std::string::String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub(super) struct ArrayNode {
    pub values: Vec<String>,
}
impl Visitable<Vec<String>> for ArrayNode {
    fn visit(&self, _commit_type: &str, _scope: &str, _breaking: bool) -> Vec<String> {
        self.values.clone()
    }
}

#[derive(Debug)]
pub(super) struct InNode {
    pub object: ObjectNode,
    pub array: ArrayNode,
}
impl Visitable<bool> for InNode {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
        self.array.visit(commit_type, scope, breaking).contains(&self.object.visit(commit_type, scope, breaking))
    }
}

#[derive(Debug)]
pub(super) enum BasicStatement {
    In(InNode),
    Breaking(BreakingNode),
}

impl Visitable<bool> for BasicStatement {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
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

impl Visitable<bool> for FirstAndValue {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
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

impl Visitable<bool> for SecondAndValue {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
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

impl Visitable<bool> for AndStatement {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking) && self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) enum FirstOrValue {
    And(AndStatement),
    Basic(BasicStatement),
}

impl Visitable<bool> for FirstOrValue {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
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

impl Visitable<bool> for SecondOrValue {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
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

impl Visitable<bool> for OrStatement {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
        self.left.visit(commit_type, scope, breaking) || self.right.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) struct PriorityStatement {
    pub internal_node: OrStatement,
}

impl Visitable<bool> for PriorityStatement {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
        self.internal_node.visit(commit_type, scope, breaking)
    }
}

#[derive(Debug)]
pub(super) enum Start {
    And(AndStatement),
    Or(OrStatement),
    Basic(BasicStatement),
}

impl Visitable<bool> for Start {
    fn visit(&self, commit_type: &str, scope: &str, breaking: bool) -> bool {
        match self {
            Self::And(n) => n.visit(commit_type, scope, breaking),
            Self::Or(n) => n.visit(commit_type, scope, breaking),
            Self::Basic(n) => n.visit(commit_type, scope, breaking),
        }
    }
}


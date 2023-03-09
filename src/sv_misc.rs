use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

pub fn identifier(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    };

    match id {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn keyword(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let kwd = match unwrap_node!(parent, Keyword) {
        Some(RefNode::Keyword(x)) => Some(x.nodes.0),

        _ => None,
    };

    match kwd {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn symbol(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let symbol = match unwrap_node!(parent, Symbol) {
        Some(RefNode::Symbol(x)) => Some(x.nodes.0),

        _ => None,
    };

    match symbol {
        Some(x) => Some(syntax_tree.get_str(&x).unwrap().to_string()),
        _ => None,
    }
}

pub fn get_string(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let mut ret: String = String::new();
    let mut skip_whitespace: bool = false;

    for node in parent.into_iter().event() {
        match node {
            NodeEvent::Enter(RefNode::WhiteSpace(_)) => skip_whitespace = true,
            NodeEvent::Leave(RefNode::WhiteSpace(_)) => skip_whitespace = false,
            NodeEvent::Enter(RefNode::Locate(x)) => {
                if !skip_whitespace {
                    ret.push_str(&syntax_tree.get_str(x).unwrap().to_string());
                }
            }

            _ => (),
        }
    }

    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

pub fn get_comment(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<Vec<String>> {
    let mut ret: Vec<String> = Vec::new();
    let mut extract_comment: bool = false;

    for node in parent.into_iter().event() {
        match node {
            NodeEvent::Enter(RefNode::Comment(_)) => extract_comment = true,
            NodeEvent::Leave(RefNode::Comment(_)) => extract_comment = false,
            NodeEvent::Enter(RefNode::Locate(x)) => {
                if extract_comment {
                    ret.push(syntax_tree.get_str(x).unwrap().to_string());
                }
            }

            _ => (),
        }
    }

    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

use crate::structures::{SvPackageDeclaration, SvParamType};
use crate::sv_misc::identifier;
use crate::sv_port::port_parameter_declaration_ansi;
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn package_declaration(
    m: RefNode,
    syntax_tree: &SyntaxTree,
    filepath: &str,
) -> SvPackageDeclaration {
    let mut ret = SvPackageDeclaration {
        identifier: package_identifier(m.clone(), syntax_tree).unwrap(),
        parameters: Vec::new(),
        filepath: String::from(filepath),
    };

    for node in m {
        match node {
            RefNode::ParameterDeclarationParam(_) | RefNode::LocalParameterDeclarationParam(_) => {
                let common_data = unwrap_node!(node.clone(), DataType, DataTypeOrImplicit);
                let a = unwrap_node!(node.clone(), ListOfParamAssignments);

                for param in a.unwrap() {
                    match param {
                        RefNode::ParamAssignment(x) => {
                            ret.parameters.push(port_parameter_declaration_ansi(
                                x,
                                syntax_tree,
                                common_data.clone(),
                                &SvParamType::LocalParam,
                            ));
                        }
                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }

    ret
}

fn package_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = unwrap_node!(node, PackageIdentifier).unwrap();
    identifier(id, &syntax_tree)
}

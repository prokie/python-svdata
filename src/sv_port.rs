use crate::structures::{
    SvDataKind, SvDataType, SvNetType, SvPackedDimension, SvParamType, SvParameter, SvPort,
    SvPortDirection, SvSignedness, SvUnpackedDimension,
};
use crate::sv_misc::{get_comment, get_string, identifier, keyword, symbol};
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn port_declaration_ansi(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
    prev_port: &Option<SvPort>,
) -> SvPort {
    let inherit = port_check_inheritance_ansi(p, prev_port);
    let ret: SvPort;

    if inherit == false {
        ret = SvPort {
            identifier: port_identifier(p, syntax_tree),
            direction: port_direction_ansi(p, prev_port),
            nettype: port_nettype_ansi(p, &port_direction_ansi(p, prev_port)),
            datakind: port_datakind_ansi(&port_nettype_ansi(p, &port_direction_ansi(p, prev_port))),
            datatype: port_datatype_ansi(p, syntax_tree),
            classid: port_classid_ansi(p, &port_datatype_ansi(p, syntax_tree), syntax_tree),
            signedness: port_signedness_ansi(p, &port_datatype_ansi(p, syntax_tree)),
            packed_dimensions: port_packeddim_ansi(RefNode::AnsiPortDeclaration(p), syntax_tree),
            unpacked_dimensions: port_unpackeddim_ansi(
                RefNode::AnsiPortDeclaration(p),
                syntax_tree,
            ),
            comment: get_comment(RefNode::AnsiPortDeclaration(p), syntax_tree),
        }
    } else {
        let prev_port = prev_port.clone().unwrap();
        ret = SvPort {
            identifier: port_identifier(p, syntax_tree),
            direction: prev_port.direction,
            nettype: prev_port.nettype,
            datakind: prev_port.datakind,
            datatype: prev_port.datatype,
            classid: prev_port.classid,
            signedness: prev_port.signedness,
            packed_dimensions: prev_port.packed_dimensions,
            unpacked_dimensions: port_unpackeddim_ansi(
                RefNode::AnsiPortDeclaration(p),
                syntax_tree,
            ),
            comment: get_comment(RefNode::AnsiPortDeclaration(p), syntax_tree),
        };
    }

    return ret;
}

pub fn port_parameter_declaration_ansi(
    p: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    common_data: Option<RefNode>,
    param_type: &SvParamType,
) -> SvParameter {
    let found_assignment = port_parameter_check_default_ansi(p);
    let (param_datatype, param_explicit_datatype) = port_parameter_datatype_ansi(
        common_data.clone(),
        p,
        syntax_tree,
        found_assignment,
        param_type,
    );
    let (param_signedness, param_explicit_signedness) = port_parameter_signedness_ansi(
        common_data.clone(),
        p,
        &param_datatype,
        found_assignment,
        param_explicit_datatype.clone(),
        syntax_tree,
    );
    let mut param_packeddim: Vec<SvPackedDimension> = Vec::new();
    match common_data {
        Some(_) => {
            param_packeddim = port_packeddim_ansi(common_data.clone().unwrap(), syntax_tree);
        }
        _ => (),
    }

    let is_param = match param_type.clone() {
        SvParamType::LocalParam => false,
        SvParamType::Parameter => true,
    };

    let ret = SvParameter {
        identifier: port_parameter_identifier_ansi(p, syntax_tree),
        paramtype: param_type.clone(),
        datatype: param_datatype.clone(),
        datatype_overridable: param_explicit_datatype.clone() && is_param,
        classid: port_parameter_classid_ansi(common_data.clone(), &param_datatype, syntax_tree),
        signedness: param_signedness.clone(),
        signedness_overridable: param_explicit_signedness && is_param,
        packed_dimensions: param_packeddim.clone(),
        unpacked_dimensions: port_unpackeddim_ansi(RefNode::ParamAssignment(p), syntax_tree),
        expression: port_parameter_value_ansi(p, syntax_tree, found_assignment),
        num_bits: port_parameter_bits_ansi(
            param_packeddim.clone(),
            p,
            &param_datatype,
            param_explicit_datatype,
            found_assignment,
            &port_parameter_value_ansi(p, syntax_tree, found_assignment),
            syntax_tree,
        ),
        comment: get_comment(RefNode::ParamAssignment(p), syntax_tree),
    };

    port_parameter_syntax_ansi(
        &ret.datatype,
        &ret.signedness,
        &ret.packed_dimensions,
        param_type,
        found_assignment,
    );

    ret
}

fn port_parameter_check_default_ansi(node: &sv_parser::ParamAssignment) -> bool {
    let expression = unwrap_node!(node, ConstantParamExpression);
    match expression {
        Some(RefNode::ConstantParamExpression(_)) => true,
        _ => false,
    }
}

fn port_parameter_syntax_ansi(
    datatype: &Option<SvDataType>,
    signedness: &Option<SvSignedness>,
    packed_dimensions: &Vec<SvPackedDimension>,
    param_type: &SvParamType,
    found_assignment: bool,
) {
    if !packed_dimensions.is_empty() {
        match datatype {
            Some(SvDataType::Integer) => {
                panic!("Cannot combine packed dimensions with an integer!")
            }
            Some(SvDataType::Real) => panic!("Cannot combine packed dimensions with a real!"),
            Some(SvDataType::String) => panic!("Cannot combine packed dimensions with a string!"),
            Some(SvDataType::Time) => panic!("Cannot combine packed dimensions with time!"),
            _ => (),
        }
    }

    match signedness {
        Some(SvSignedness::Signed) | Some(SvSignedness::Unsigned) => match datatype {
            Some(SvDataType::Real) => panic!("Reals cannot have signedness!"),
            Some(SvDataType::String) => panic!("Strings cannot have signedness!"),
            _ => (),
        },

        _ => (),
    }

    match (param_type, found_assignment) {
        (SvParamType::LocalParam, false) => panic!("Localparams must have a default value!"),
        _ => (),
    }
}

fn parameter_resolver_needed_ansi(node: &sv_parser::ParamAssignment) -> bool {
    let expression = unwrap_node!(
        node,
        ConstantFunctionCall,
        BinaryOperator,
        ConstantConcatenation,
        ConditionalExpression
    );
    match expression {
        Some(_) => true,
        _ => false,
    }
}

fn parameter_datatype_resolver_ansi(node: &sv_parser::ParamAssignment) -> SvDataType {
    let datatype = unwrap_node!(
        node,
        Number,
        TimeLiteral,
        UnbasedUnsizedLiteral,
        StringLiteral
    );
    match datatype {
        Some(RefNode::Number(sv_parser::Number::IntegralNumber(_))) => {
            let subtype = unwrap_node!(node, RealNumber);
            match subtype {
                Some(_) => SvDataType::Real,
                _ => SvDataType::Logic,
            }
        }

        Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => SvDataType::Real,
        Some(RefNode::TimeLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => SvDataType::Logic,
                _ => SvDataType::Time,
            }
        }
        Some(RefNode::UnbasedUnsizedLiteral(_)) => {
            let subtype = unwrap_node!(node, RealNumber, IntegralNumber);
            match subtype {
                Some(RefNode::RealNumber(_)) => SvDataType::Real,
                Some(RefNode::IntegralNumber(_)) => SvDataType::Logic,
                _ => SvDataType::Bit,
            }
        }
        Some(RefNode::StringLiteral(_)) => SvDataType::String,
        _ => SvDataType::Unsupported,
    }
}

fn parameter_signedness_resolver_ansi(
    node: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    syntax_tree: &SyntaxTree,
) -> Option<SvSignedness> {
    match datatype {
        Some(SvDataType::String) => return None,
        _ => (),
    }

    let mut ret: Option<SvSignedness> = Some(SvSignedness::Signed);
    for sub_node in node {
        match sub_node {
            RefNode::Number(sv_parser::Number::IntegralNumber(_)) => {
                let integral_type = unwrap_node!(sub_node, BinaryNumber, HexNumber, OctalNumber);
                match integral_type {
                    Some(RefNode::BinaryNumber(_))
                    | Some(RefNode::HexNumber(_))
                    | Some(RefNode::OctalNumber(_)) => {
                        let base = unwrap_node!(
                            integral_type.unwrap(),
                            BinaryBase,
                            HexBase,
                            OctalBase,
                            DecimalNumberBaseUnsigned
                        );

                        let base_token;
                        match base.clone() {
                            Some(_) => {
                                base_token = get_string(base.clone().unwrap(), syntax_tree).unwrap()
                            }
                            _ => {
                                ret = Some(SvSignedness::Unsupported); // If not primary literals
                                break;
                            }
                        }

                        match base {
                            Some(RefNode::BinaryBase(_)) => {
                                if base_token != "'sb" {
                                    ret = Some(SvSignedness::Unsigned);
                                    break;
                                }
                            }

                            Some(RefNode::HexBase(_)) => {
                                if base_token != "'sh" {
                                    ret = Some(SvSignedness::Unsigned);
                                    break;
                                }
                            }

                            Some(RefNode::OctalBase(_)) => {
                                if base_token != "'so" {
                                    ret = Some(SvSignedness::Unsigned);
                                    break;
                                }
                            }

                            Some(RefNode::DecimalNumberBaseUnsigned(_)) => {
                                if base_token != "'sd" {
                                    ret = Some(SvSignedness::Unsigned);
                                    break;
                                }
                            }

                            _ => unreachable!(),
                        }
                    }

                    _ => (),
                }
            }

            RefNode::Number(sv_parser::Number::RealNumber(_)) => {
                ret = None;
                break;
            }

            RefNode::TimeLiteral(_) => {
                ret = Some(SvSignedness::Unsigned);
                break;
            }

            RefNode::UnbasedUnsizedLiteral(_) => {
                ret = Some(SvSignedness::Unsigned);
                break;
            }

            RefNode::BinaryOperator(_) => {
                let symbol_token = symbol(sub_node, syntax_tree).unwrap();
                match symbol_token.as_str() {
                    "&" | "~&" | "|" | "~|" | "^" | "~^" | "<" | "<=" | ">" | ">=" | "=="
                    | "=!" => {
                        ret = Some(SvSignedness::Unsigned);
                        break;
                    }
                    _ => (),
                }
            }

            _ => (),
        }
    }

    ret
}

fn port_parameter_identifier_ansi(
    node: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
) -> String {
    let id = unwrap_node!(node, ParameterIdentifier).unwrap();
    identifier(id, &syntax_tree).unwrap()
}

fn port_parameter_value_ansi(
    node: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    found_assignment: bool,
) -> Option<String> {
    if !found_assignment {
        return None;
    } else {
        let expression = unwrap_node!(node, ConstantParamExpression);
        get_string(expression.unwrap(), syntax_tree)
    }
}

fn port_parameter_datatype_ansi(
    common_data: Option<RefNode>,
    p: &sv_parser::ParamAssignment,
    syntax_tree: &SyntaxTree,
    found_assignment: bool,
    param_type: &SvParamType,
) -> (Option<SvDataType>, bool) {
    let datatype: Option<RefNode>;
    let mut ret: (Option<SvDataType>, bool) = match param_type {
        SvParamType::Parameter => (None, true),
        SvParamType::LocalParam => (Some(SvDataType::Logic), false),
    };

    match common_data {
        Some(_) => {
            datatype = unwrap_node!(
                common_data.clone().unwrap(),
                IntegerVectorType,
                IntegerAtomType,
                NonIntegerType,
                ClassType,
                TypeReference
            );
        }
        _ => {
            datatype = None;
        }
    }

    match datatype {
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Logic(_))) => {
            (Some(SvDataType::Logic), false)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) => {
            (Some(SvDataType::Reg), false)
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) => {
            (Some(SvDataType::Bit), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) => {
            (Some(SvDataType::Byte), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) => {
            (Some(SvDataType::Shortint), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) => {
            (Some(SvDataType::Int), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) => {
            (Some(SvDataType::Longint), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) => {
            (Some(SvDataType::Integer), false)
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) => {
            (Some(SvDataType::Time), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) => {
            (Some(SvDataType::Shortreal), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) => {
            (Some(SvDataType::Realtime), false)
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) => {
            (Some(SvDataType::Real), false)
        }
        Some(RefNode::ClassType(_)) => (Some(SvDataType::Class), false),
        Some(RefNode::TypeReference(_)) => (Some(SvDataType::TypeRef), false),
        _ => {
            if common_data != None {
                match unwrap_node!(common_data.unwrap(), DataType) {
                    Some(x) => match keyword(x, syntax_tree) {
                        Some(x) => {
                            if x == "string" {
                                return (Some(SvDataType::String), false);
                            } else {
                                println!("{}", x);
                                unreachable!();
                            }
                        }

                        _ => unreachable!(),
                    },

                    _ => (),
                }
            }

            if found_assignment {
                if parameter_resolver_needed_ansi(p) {
                    match unwrap_node!(p, BinaryOperator) {
                        Some(_) => ret = (Some(parameter_datatype_resolver_ansi(p)), true),
                        _ => ret = (Some(SvDataType::Unsupported), true),
                    }
                } else {
                    let implicit_type =
                        unwrap_node!(p, Number, TimeLiteral, UnbasedUnsizedLiteral, StringLiteral);
                    match implicit_type {
                        Some(RefNode::UnbasedUnsizedLiteral(_))
                        | Some(RefNode::Number(sv_parser::Number::IntegralNumber(_))) => {
                            ret = (Some(SvDataType::Logic), true);
                        }
                        Some(RefNode::Number(sv_parser::Number::RealNumber(_))) => {
                            ret = (Some(SvDataType::Real), true);
                        }
                        Some(RefNode::TimeLiteral(_)) => ret = (Some(SvDataType::Time), true),
                        Some(RefNode::StringLiteral(_)) => ret = (Some(SvDataType::String), true),
                        _ => ret = (Some(SvDataType::Unsupported), true),
                    }
                }
            }

            ret
        }
    }
}

fn port_parameter_signedness_ansi(
    m: Option<RefNode>,
    p: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    found_assignment: bool,
    datatype_overridable: bool,
    syntax_tree: &SyntaxTree,
) -> (Option<SvSignedness>, bool) {
    let ret: (Option<SvSignedness>, bool);

    match m {
        Some(_) => {
            let signedness = unwrap_node!(m.clone().unwrap(), Signing);
            match signedness {
                Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => {
                    return (Some(SvSignedness::Signed), false)
                }
                Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => {
                    return (Some(SvSignedness::Unsigned), false)
                }
                _ => (),
            }
        }

        _ => (),
    }

    match datatype {
        Some(SvDataType::Class) | Some(SvDataType::String) | Some(SvDataType::Real) => {
            match datatype_overridable {
                true => ret = (None, true),
                false => ret = (None, false),
            }
        }

        Some(SvDataType::Shortint)
        | Some(SvDataType::Int)
        | Some(SvDataType::Longint)
        | Some(SvDataType::Byte)
        | Some(SvDataType::Integer) => ret = (Some(SvSignedness::Signed), true),

        Some(SvDataType::Logic) => {
            if !datatype_overridable || !found_assignment {
                ret = (Some(SvSignedness::Unsigned), true);
            } else {
                if parameter_resolver_needed_ansi(p) {
                    match unwrap_node!(p, BinaryOperator) {
                        Some(_) => {
                            ret = (
                                parameter_signedness_resolver_ansi(p, datatype, syntax_tree),
                                true,
                            )
                        }
                        _ => ret = (Some(SvSignedness::Unsupported), true),
                    }
                } else {
                    let integral_type = unwrap_node!(
                        p,
                        DecimalNumber,
                        BinaryNumber,
                        HexNumber,
                        OctalNumber,
                        UnbasedUnsizedLiteral
                    );

                    match integral_type {
                        Some(RefNode::UnbasedUnsizedLiteral(_)) => {
                            ret = (Some(SvSignedness::Unsigned), true)
                        }
                        Some(RefNode::DecimalNumber(sv_parser::DecimalNumber::UnsignedNumber(
                            _,
                        ))) => ret = (Some(SvSignedness::Signed), true),
                        _ => {
                            let base = unwrap_node!(
                                integral_type.unwrap(),
                                BinaryBase,
                                HexBase,
                                OctalBase,
                                DecimalBase
                            );

                            let base_token;
                            match base.clone() {
                                Some(_) => {
                                    base_token =
                                        get_string(base.clone().unwrap(), syntax_tree).unwrap()
                                }
                                _ => {
                                    return (Some(SvSignedness::Unsupported), true);
                                    // If not primary literal
                                }
                            }

                            match base {
                                Some(RefNode::BinaryBase(_)) => {
                                    ret = if base_token == "'sb" {
                                        (Some(SvSignedness::Signed), true)
                                    } else {
                                        (Some(SvSignedness::Unsigned), true)
                                    };
                                }

                                Some(RefNode::HexBase(_)) => {
                                    ret = if base_token == "'sh" {
                                        (Some(SvSignedness::Signed), true)
                                    } else {
                                        (Some(SvSignedness::Unsigned), true)
                                    };
                                }

                                Some(RefNode::OctalBase(_)) => {
                                    ret = if base_token == "'so" {
                                        (Some(SvSignedness::Signed), true)
                                    } else {
                                        (Some(SvSignedness::Unsigned), true)
                                    };
                                }

                                Some(RefNode::DecimalBase(_)) => {
                                    println!("{}", base_token);
                                    ret = if base_token == "'sd" {
                                        (Some(SvSignedness::Signed), true)
                                    } else {
                                        (Some(SvSignedness::Unsigned), false)
                                    };
                                }

                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }

        _ => match datatype {
            Some(SvDataType::Unsupported) => ret = (Some(SvSignedness::Unsupported), true),
            None => ret = (None, true),
            _ => ret = (Some(SvSignedness::Unsigned), true),
        },
    }

    ret
}

fn port_parameter_classid_ansi(
    m: Option<RefNode>,
    datatype: &Option<SvDataType>,
    syntax_tree: &SyntaxTree,
) -> Option<String> {
    match datatype {
        Some(SvDataType::Class) => {
            if let Some(id) = unwrap_node!(m.unwrap(), ClassIdentifier) {
                identifier(id, syntax_tree)
            } else {
                unreachable!()
            }
        }

        _ => None,
    }
}

fn port_parameter_bits_ansi(
    mut packed_dimensions: Vec<SvPackedDimension>,
    p: &sv_parser::ParamAssignment,
    datatype: &Option<SvDataType>,
    datatype_overridable: bool,
    found_assignment: bool,
    expression: &Option<String>,
    syntax_tree: &SyntaxTree,
) -> Option<u64> {
    if !packed_dimensions.is_empty() {
        let mut nu_bits: u64 = 0;
        packed_dimensions.reverse();

        for dim in packed_dimensions {
            let (left, right) = dim;
            let left_num: std::result::Result<i64, _> = left.as_str().parse();
            let right_num: std::result::Result<i64, _> = right.as_str().parse();

            match (left_num, right_num) {
                (Ok(left_num), Ok(right_num)) => {
                    let res: i64 = left_num - right_num;
                    if nu_bits == 0 {
                        nu_bits = res.abs() as u64 + 1;
                    } else {
                        nu_bits = nu_bits * (res.abs() as u64 + 1);
                    }
                }

                _ => return Some(404), // TODO
            }
        }

        Some(nu_bits)
    } else {
        match datatype {
            Some(SvDataType::Class) => None,

            Some(SvDataType::Bit) => Some(1),

            Some(SvDataType::Byte) => Some(8),

            Some(SvDataType::Integer) | Some(SvDataType::Int) | Some(SvDataType::Shortreal) => {
                Some(32)
            }

            Some(SvDataType::Shortint) => Some(16),

            Some(SvDataType::Longint)
            | Some(SvDataType::Time)
            | Some(SvDataType::Real)
            | Some(SvDataType::Realtime) => Some(64),

            Some(SvDataType::String) => {
                if parameter_resolver_needed_ansi(p) {
                    Some(404) // TODO
                } else {
                    if !found_assignment {
                        None
                    } else {
                        Some((expression.clone().unwrap().len() as u64 - 2) * 8)
                    }
                }
            }

            Some(SvDataType::Reg) | Some(SvDataType::Logic) => {
                if parameter_resolver_needed_ansi(p) {
                    Some(404) // TODO
                } else {
                    if !datatype_overridable {
                        Some(1)
                    } else if !found_assignment {
                        None
                    } else {
                        let fixed_size = unwrap_node!(p, Size, UnbasedUnsizedLiteral);

                        match fixed_size {
                            Some(RefNode::Size(_)) => {
                                let ret: u64;
                                ret = get_string(fixed_size.clone().unwrap(), syntax_tree)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .unwrap();
                                Some(ret)
                            }
                            Some(RefNode::UnbasedUnsizedLiteral(_)) => Some(1),
                            _ => {
                                Some(32)

                                // TODO
                            }
                        }
                    }
                }
            }

            Some(SvDataType::Unsupported) => Some(404), // TODO

            None => None,

            _ => unreachable!(),
        }
    }
}

fn port_identifier(node: &sv_parser::AnsiPortDeclaration, syntax_tree: &SyntaxTree) -> String {
    if let Some(id) = unwrap_node!(node, PortIdentifier) {
        identifier(id, syntax_tree).unwrap()
    } else {
        unreachable!()
    }
}

fn port_direction_ansi(
    node: &sv_parser::AnsiPortDeclaration,
    prev_port: &Option<SvPort>,
) -> SvPortDirection {
    let dir = unwrap_node!(node, PortDirection);
    match dir {
        Some(RefNode::PortDirection(sv_parser::PortDirection::Inout(_))) => SvPortDirection::Inout,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Input(_))) => SvPortDirection::Input,
        Some(RefNode::PortDirection(sv_parser::PortDirection::Output(_))) => {
            SvPortDirection::Output
        }
        Some(RefNode::PortDirection(sv_parser::PortDirection::Ref(_))) => SvPortDirection::Ref,
        _ => match prev_port {
            Some(_) => prev_port.clone().unwrap().direction,
            None => SvPortDirection::Inout,
        },
    }
}

fn port_datakind_ansi(nettype: &Option<SvNetType>) -> SvDataKind {
    match nettype {
        None => SvDataKind::Variable,

        Some(_) => SvDataKind::Net,
    }
}

fn port_datatype_ansi(
    node: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> SvDataType {
    let datatype = unwrap_node!(
        node,
        IntegerVectorType,
        IntegerAtomType,
        NonIntegerType,
        ClassType,
        TypeReference
    );
    match datatype {
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Logic(_))) => {
            SvDataType::Logic
        }
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Reg(_))) => SvDataType::Reg,
        Some(RefNode::IntegerVectorType(sv_parser::IntegerVectorType::Bit(_))) => SvDataType::Bit,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Byte(_))) => SvDataType::Byte,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Shortint(_))) => {
            SvDataType::Shortint
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Int(_))) => SvDataType::Int,
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Longint(_))) => {
            SvDataType::Longint
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Integer(_))) => {
            SvDataType::Integer
        }
        Some(RefNode::IntegerAtomType(sv_parser::IntegerAtomType::Time(_))) => SvDataType::Time,
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Shortreal(_))) => {
            SvDataType::Shortreal
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Realtime(_))) => {
            SvDataType::Realtime
        }
        Some(RefNode::NonIntegerType(sv_parser::NonIntegerType::Real(_))) => SvDataType::Real,
        Some(RefNode::ClassType(_)) => SvDataType::Class,
        Some(RefNode::TypeReference(_)) => SvDataType::TypeRef,
        _ => match unwrap_node!(node, DataType) {
            Some(x) => match keyword(x, syntax_tree) {
                Some(x) => {
                    if x == "string" {
                        return SvDataType::String;
                    } else {
                        println!("{}", x);
                        unreachable!();
                    }
                }

                _ => unreachable!(),
            },
            _ => return SvDataType::Logic,
        },
    }
}

fn port_nettype_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    direction: &SvPortDirection,
) -> Option<SvNetType> {
    let objecttype = unwrap_node!(m, AnsiPortDeclarationVariable, AnsiPortDeclarationNet);
    match objecttype {
        Some(RefNode::AnsiPortDeclarationVariable(_)) => {
            match unwrap_node!(m, PortDirection, DataType, Signing, PackedDimension) {
                Some(_) => None,
                _ => Some(SvNetType::Wire),
            }
        }

        Some(RefNode::AnsiPortDeclarationNet(x)) => {
            let nettype = unwrap_node!(x, NetType);

            match nettype {
                // "Var" token was not found
                Some(RefNode::NetType(sv_parser::NetType::Supply0(_))) => Some(SvNetType::Supply0),
                Some(RefNode::NetType(sv_parser::NetType::Supply1(_))) => Some(SvNetType::Supply1),
                Some(RefNode::NetType(sv_parser::NetType::Triand(_))) => Some(SvNetType::Triand),
                Some(RefNode::NetType(sv_parser::NetType::Trior(_))) => Some(SvNetType::Trior),
                Some(RefNode::NetType(sv_parser::NetType::Trireg(_))) => Some(SvNetType::Trireg),
                Some(RefNode::NetType(sv_parser::NetType::Tri0(_))) => Some(SvNetType::Tri0),
                Some(RefNode::NetType(sv_parser::NetType::Tri1(_))) => Some(SvNetType::Tri1),
                Some(RefNode::NetType(sv_parser::NetType::Tri(_))) => Some(SvNetType::Tri),
                Some(RefNode::NetType(sv_parser::NetType::Uwire(_))) => Some(SvNetType::Uwire),
                Some(RefNode::NetType(sv_parser::NetType::Wire(_))) => Some(SvNetType::Wire),
                Some(RefNode::NetType(sv_parser::NetType::Wand(_))) => Some(SvNetType::Wand),
                Some(RefNode::NetType(sv_parser::NetType::Wor(_))) => Some(SvNetType::Wor),

                _ => match direction {
                    SvPortDirection::Inout | SvPortDirection::Input => Some(SvNetType::Wire),
                    SvPortDirection::Output => match unwrap_node!(m, DataType) {
                        Some(_) => None,
                        _ => Some(SvNetType::Wire),
                    },

                    SvPortDirection::Ref => None,

                    _ => unreachable!(),
                },
            }
        }

        _ => unreachable!(),
    }
}

fn port_signedness_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    datatype: &SvDataType,
) -> Option<SvSignedness> {
    match datatype {
        SvDataType::Class | SvDataType::String | SvDataType::Real | SvDataType::Time => None,
        _ => {
            let signedness = unwrap_node!(m, Signing);
            match signedness {
                Some(RefNode::Signing(sv_parser::Signing::Signed(_))) => {
                    return Some(SvSignedness::Signed)
                }
                Some(RefNode::Signing(sv_parser::Signing::Unsigned(_))) => {
                    return Some(SvSignedness::Unsigned)
                }
                _ => (),
            }

            match datatype {
                SvDataType::Shortint
                | SvDataType::Int
                | SvDataType::Longint
                | SvDataType::Byte
                | SvDataType::Integer => Some(SvSignedness::Signed),
                _ => Some(SvSignedness::Unsigned),
            }
        }
    }
}

fn port_packeddim_ansi(m: RefNode, syntax_tree: &SyntaxTree) -> Vec<SvPackedDimension> {
    let mut ret: Vec<SvPackedDimension> = Vec::new();

    for node in m {
        match node {
            RefNode::PackedDimensionRange(x) => {
                let range = unwrap_node!(x, ConstantRange);
                match range {
                    Some(RefNode::ConstantRange(sv_parser::ConstantRange { nodes })) => {
                        let (l, _, r) = nodes;
                        let left =
                            get_string(RefNode::ConstantExpression(&l), syntax_tree).unwrap();
                        let right =
                            get_string(RefNode::ConstantExpression(&r), syntax_tree).unwrap();

                        ret.push((left, right));
                    }

                    _ => (),
                }
            }

            _ => (),
        }
    }

    ret
}

fn port_unpackeddim_ansi(m: RefNode, syntax_tree: &SyntaxTree) -> Vec<SvUnpackedDimension> {
    let mut ret: Vec<SvUnpackedDimension> = Vec::new();

    for node in m {
        match node {
            RefNode::UnpackedDimensionRange(x) => {
                let range = unwrap_node!(x, ConstantRange);
                match range {
                    Some(RefNode::ConstantRange(sv_parser::ConstantRange { nodes })) => {
                        let (l, _, r) = nodes;
                        let left = get_string(RefNode::ConstantExpression(l), syntax_tree).unwrap();
                        let right =
                            get_string(RefNode::ConstantExpression(r), syntax_tree).unwrap();

                        ret.push((left, Some(right)));
                    }

                    _ => (),
                }
            }

            RefNode::UnpackedDimensionExpression(x) => {
                let range = unwrap_node!(x, ConstantExpression).unwrap();
                let left = get_string(range, syntax_tree).unwrap();

                ret.push((left, None));
            }

            _ => (),
        }
    }

    ret
}

fn port_classid_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    datatype: &SvDataType,
    syntax_tree: &SyntaxTree,
) -> Option<String> {
    match datatype {
        SvDataType::Class => {
            if let Some(id) = unwrap_node!(m, ClassIdentifier) {
                identifier(id, syntax_tree)
            } else {
                unreachable!()
            }
        }

        _ => None,
    }
}

fn port_check_inheritance_ansi(
    m: &sv_parser::AnsiPortDeclaration,
    prev_port: &Option<SvPort>,
) -> bool {
    let datatype = unwrap_node!(
        m,
        DataType,
        Signing,
        NetType,
        VarDataType,
        PortDirection,
        PackedDimension
    );

    match prev_port {
        Some(_) => match datatype {
            Some(_) => false,
            _ => true,
        },
        None => false,
    }
}

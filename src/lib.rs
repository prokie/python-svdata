use std::{collections::HashMap, path::PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*};
use structures::SvData;
use sv_module::module_declaration_ansi;
use sv_package::package_declaration;
use sv_parser::{parse_sv, NodeEvent, RefNode, SyntaxTree};

pub mod structures;
pub mod sv_instance;
pub mod sv_misc;
pub mod sv_module;
pub mod sv_package;
pub mod sv_port;
pub mod sv_primlit;
pub mod sv_primlit_integral;

/// Reads a systemverilog file and returns an `SvData` object.
#[pyfunction]
pub fn read_sv_file(file_path: &str) -> PyResult<SvData> {
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    let mut svdata = SvData {
        modules: Vec::new(),
        packages: Vec::new(),
    };

    if let Ok((syntax_tree, _)) = parse_sv(&file_path, &defines, &includes, true, false) {
        sv_to_structure(&syntax_tree, &file_path, &mut svdata);
    } else {
        Err(PyValueError::new_err(format!(
            "Could not parse {}.",
            file_path
        )))?
    }

    Ok(svdata)
}

fn sv_to_structure(syntax_tree: &SyntaxTree, filepath: &str, svdata: &mut SvData) -> () {
    for event in syntax_tree.into_iter().event() {
        let enter_not_leave = match event {
            NodeEvent::Enter(_) => true,
            NodeEvent::Leave(_) => false,
        };
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(x) => x,
        };

        if enter_not_leave {
            match node {
                RefNode::ModuleDeclarationAnsi(_) => {
                    svdata
                        .modules
                        .push(module_declaration_ansi(node, syntax_tree, filepath).clone());
                }
                RefNode::PackageDeclaration(_) => {
                    svdata
                        .packages
                        .push(package_declaration(node, syntax_tree, filepath).clone());
                }
                _ => (),
            }
        }
    }
}

#[pymodule]
fn python_svdata(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_sv_file, m)?)?;
    m.add_class::<SvData>()?;

    Ok(())
}

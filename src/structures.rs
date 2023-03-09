use pyo3::prelude::*;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvData {
    #[pyo3(get, set)]
    pub modules: Vec<SvModuleDeclaration>,
    #[pyo3(get, set)]
    pub packages: Vec<SvPackageDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvModuleDeclaration {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub parameters: Vec<SvParameter>,
    #[pyo3(get, set)]
    pub ports: Vec<SvPort>,
    #[pyo3(get, set)]
    pub instances: Vec<SvInstance>,
    #[pyo3(get, set)]
    pub filepath: String,
    #[pyo3(get, set)]
    pub comments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvPackageDeclaration {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub parameters: Vec<SvParameter>,
    #[pyo3(get, set)]
    pub filepath: String,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvParameter {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub expression: Option<String>,
    #[pyo3(get, set)]
    pub paramtype: SvParamType,
    #[pyo3(get, set)]
    pub datatype: Option<SvDataType>,
    #[pyo3(get, set)]
    pub datatype_overridable: bool,
    #[pyo3(get, set)]
    pub classid: Option<String>,
    #[pyo3(get, set)]
    pub signedness: Option<SvSignedness>,
    #[pyo3(get, set)]
    pub signedness_overridable: bool,
    #[pyo3(get, set)]
    pub num_bits: Option<u64>,
    #[pyo3(get, set)]
    pub packed_dimensions: Vec<SvPackedDimension>,
    #[pyo3(get, set)]
    pub unpacked_dimensions: Vec<SvUnpackedDimension>,
    #[pyo3(get, set)]
    pub comment: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvParamType {
    Parameter,
    LocalParam,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvDataKind {
    Net,
    Variable,
    IMPLICIT,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvSignedness {
    Signed,
    Unsigned,
    Unsupported,
    IMPLICIT,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvDataType {
    Logic,
    Reg,
    Bit,
    Byte,
    Integer,
    Int,
    Shortint,
    Longint,
    Time,
    Real,
    Shortreal,
    Realtime,
    Array,
    Enum,
    Struct,
    Union,
    Class,
    TypeRef,
    String,
    Unsupported,
    IMPLICIT,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvNetType {
    Wire,
    Uwire,
    Tri,
    Wor,
    Wand,
    Triand,
    Trior,
    Trireg,
    Tri0,
    Tri1,
    Supply0,
    Supply1,
    IMPLICIT,
}

pub type SvPackedDimension = (String, String);
pub type SvUnpackedDimension = (String, Option<String>);

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvPort {
    pub identifier: String,
    pub direction: SvPortDirection,
    pub datakind: SvDataKind,
    pub datatype: SvDataType,
    pub classid: Option<String>,
    pub nettype: Option<SvNetType>,
    pub signedness: Option<SvSignedness>,
    pub packed_dimensions: Vec<SvPackedDimension>,
    pub unpacked_dimensions: Vec<SvUnpackedDimension>,
    pub comment: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvInstance {
    pub module_identifier: String,
    pub hierarchical_instance: String,
    pub hierarchy: Vec<String>,
    pub connections: Vec<Vec<String>>,
}

impl fmt::Display for SvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for module in &self.modules {
            write!(f, "{}", module)?;
        }
        for package in &self.packages {
            write!(f, "{}", package)?;
        }

        write!(f, "")
    }
}

impl fmt::Display for SvModuleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Module:")?;
        writeln!(f, "  Identifier: {}", self.identifier)?;
        writeln!(f, "  Filepath: {}", self.filepath)?;
        writeln!(f, "  Comments: {:?}", self.comments)?;

        for port in &self.ports {
            write!(f, "{}", port)?;
        }

        for param in &self.parameters {
            write!(f, "{}", param)?;
        }

        for instance in &self.instances {
            write!(f, "{}", instance)?;
        }

        writeln!(f, "")
    }
}

impl fmt::Display for SvInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  Instance: ")?;
        writeln!(f, "    Module identifier: {}", self.module_identifier)?;
        writeln!(
            f,
            "    Hierarchical instance: {}",
            self.hierarchical_instance
        )?;
        writeln!(f, "    Hierarchy: {:?}", self.hierarchy)?;
        writeln!(f, "    Connections: {:?}", self.connections)?;

        write!(f, "")
    }
}

impl fmt::Display for SvPackageDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Package:")?;
        writeln!(f, "  Identifier: {}", self.identifier)?;
        writeln!(f, "  Filepath: {}", self.filepath)?;

        for param in &self.parameters {
            write!(f, "{}", param)?;
        }

        writeln!(f, "")
    }
}

impl fmt::Display for SvPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  Port: ")?;
        writeln!(f, "    Identifier: {}", self.identifier)?;
        writeln!(f, "    Direction: {:?}", self.direction)?;
        writeln!(f, "    DataKind: {:?}", self.datakind)?;
        writeln!(f, "    DataType: {:?}", self.datatype)?;
        match &self.classid {
            None => {
                writeln!(f, "    ClassIdentifier: None")?;
            }
            Some(x) => {
                writeln!(f, "    ClassIdentifier: {}", x)?;
            }
        }
        match &self.nettype {
            None => {
                writeln!(f, "    NetType: None")?;
            }
            Some(x) => {
                writeln!(f, "    NetType: {:?}", x)?;
            }
        }
        match &self.signedness {
            None => {
                writeln!(f, "    Signedness: None")?;
            }
            Some(x) => {
                writeln!(f, "    Signedness: {:?}", x)?;
            }
        }

        writeln!(f, "    PackedDimensions: {:?}", self.packed_dimensions)?;
        let mut unpackeddim_display: Vec<(String, String)> = Vec::new();

        for (u, l) in &self.unpacked_dimensions {
            match l {
                Some(x) => unpackeddim_display.push((u.clone(), x.clone())),
                None => unpackeddim_display.push((u.clone(), String::from("None"))),
            }
        }
        writeln!(f, "    UnpackedDimensions: {:?}", unpackeddim_display)?;
        match &self.comment {
            None => {
                writeln!(f, "    Comment: None")?;
            }
            Some(x) => {
                writeln!(f, "    Comment: {:?}", x)?;
            }
        }

        write!(f, "")
    }
}

impl fmt::Display for SvParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  Parameter: ")?;
        writeln!(f, "    Identifier: {}", self.identifier)?;
        match &self.expression {
            None => {
                writeln!(f, "    Expression: None")?;
            }
            Some(x) => {
                writeln!(f, "    Expression: {}", x)?;
            }
        }
        writeln!(f, "    ParameterType: {:?}", self.paramtype)?;
        match &self.datatype {
            None => {
                writeln!(f, "    DataType: None")?;
            }
            Some(x) => {
                writeln!(f, "    DataType: {:?}", x)?;
            }
        }
        writeln!(
            f,
            "    DataTypeOverridable: {:?}",
            self.datatype_overridable
        )?;
        match &self.classid {
            None => {
                writeln!(f, "    ClassIdentifier: None")?;
            }
            Some(x) => {
                writeln!(f, "    ClassIdentifier: {}", x)?;
            }
        }
        match &self.signedness {
            None => {
                writeln!(f, "    Signedness: None")?;
            }
            Some(x) => {
                writeln!(f, "    Signedness: {:?}", x)?;
            }
        }
        writeln!(
            f,
            "    SignednessOverridable: {:?}",
            self.signedness_overridable
        )?;
        match &self.num_bits {
            None => {
                writeln!(f, "    NumBits: None")?;
            }
            Some(x) => {
                writeln!(f, "    NumBits: {}", x)?;
            }
        }
        writeln!(f, "    PackedDimensions: {:?}", self.packed_dimensions)?;
        let mut unpackeddim_display: Vec<(String, String)> = Vec::new();

        for (u, l) in &self.unpacked_dimensions {
            match l {
                Some(x) => unpackeddim_display.push((u.clone(), x.clone())),
                None => unpackeddim_display.push((u.clone(), String::from("None"))),
            }
        }
        writeln!(f, "    UnpackedDimensions: {:?}", unpackeddim_display)?;

        match &self.comment {
            None => {
                writeln!(f, "    Comment: None")?;
            }
            Some(x) => {
                writeln!(f, "    Comment: {:?}", x)?;
            }
        }

        write!(f, "")
    }
}

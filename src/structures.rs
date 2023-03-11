use pyo3::prelude::*;
use std::fmt;

/// This is the main data structure that is returned by the parser.
///
/// Args:
///    modules (list[SvModuleDeclaration]): A list of all the modules in the file.
///    packages (list[SvPackageDeclaration]): A list of all the packages in the file.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvData {
    #[pyo3(get, set)]
    pub modules: Vec<SvModuleDeclaration>,
    #[pyo3(get, set)]
    pub packages: Vec<SvPackageDeclaration>,
}
#[pymethods]
impl SvData {
    #[new]
    fn new() -> Self {
        SvData {
            modules: Vec::new(),
            packages: Vec::new(),
        }
    }
    fn __repr__(&self) -> String {
        self.to_string()
    }
}
/// Store the information about a module.
///
/// Args:
///
///   identifier (str): The name of the module.
///   parameters (list[SvParameter]): A list of all the parameters in the module.
///   ports (list[SvPort]): A list of all the ports in the module.
///   instances (list[SvInstance]): A list of all the instances in the module.
///   filepath (str): The path to the file that contains the module.
///   comments (list[str]): A list of all the comments in the module.
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

#[pymethods]
impl SvModuleDeclaration {
    #[new]
    fn new() -> Self {
        SvModuleDeclaration {
            identifier: String::new(),
            parameters: Vec::new(),
            ports: Vec::new(),
            instances: Vec::new(),
            filepath: String::new(),
            comments: Vec::new(),
        }
    }
    fn __repr__(&self) -> String {
        self.to_string()
    }
}

/// Store the information about a package.
///
/// Args:
///
///    identifier (str): The name of the package.
///    parameters (list[SvParameter]): A list of all the parameters in the package.
///    filepath (str): The path to the file that contains the package.
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
#[pymethods]
impl SvPackageDeclaration {
    #[new]
    fn new() -> Self {
        SvPackageDeclaration {
            identifier: String::new(),
            parameters: Vec::new(),
            filepath: String::new(),
        }
    }
    fn __repr__(&self) -> String {
        self.to_string()
    }
}

/// Store the information about a parameter.
///
/// Args:
///    identifier (str): The name of the parameter.
///    expression (str | None): The expression of the parameter.
///    paramtype (SvParamType): The type of the parameter.
///    datatype (SvDataType | None): The data type of the parameter.
///    datatype_overridable (bool): Whether the data type of the parameter is overridable.
///    classid (str | None): The class id of the parameter.
///    signedness (SvSignedness | None): The signedness of the parameter.
///    signedness_overridable (bool): Whether the signedness of the parameter is overridable.
///    num_bits (int | None): The number of bits of the parameter.
///    packed_dimensions (list[SvPackedDimension]): A list of all the packed dimensions of the parameter.
///    unpacked_dimensions (list[SvUnpackedDimension]): A list of all the unpacked dimensions of the parameter.
///    comment (list[str] | None): A list of all the comments of the parameter.
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
#[pymethods]
impl SvParameter {
    #[new]
    fn new() -> Self {
        SvParameter {
            identifier: String::new(),
            expression: None,
            paramtype: SvParamType::Parameter,
            datatype: None,
            datatype_overridable: false,
            classid: None,
            signedness: None,
            signedness_overridable: false,
            num_bits: None,
            packed_dimensions: Vec::new(),
            unpacked_dimensions: Vec::new(),
            comment: None,
        }
    }
    fn __repr__(&self) -> String {
        self.to_string()
    }
}

/// Parameter types.
///
/// Args:
///   Parameter (SvParamType): A parameter.
///   LocalParam (SvParamType): A local parameter.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvParamType {
    Parameter,
    LocalParam,
}

#[pymethods]
impl SvParamType {
    #[new]
    fn new() -> Self {
        SvParamType::Parameter
    }

    fn __repr__(&self) -> String {
        match self {
            SvParamType::Parameter => "Parameter".to_string(),
            SvParamType::LocalParam => "LocalParam".to_string(),
        }
    }
}

/// Port directions.
///
/// Args:
///    Inout (SvPortDirection): An inout port.
///    Input (SvPortDirection): An input port.
///    Output (SvPortDirection): An output port.
///    Ref (SvPortDirection): A ref port.
///    IMPLICIT (SvPortDirection): An implicit port.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[pymethods]
impl SvPortDirection {
    fn __repr__(&self) -> String {
        match self {
            SvPortDirection::Inout => "Inout".to_string(),
            SvPortDirection::Input => "Input".to_string(),
            SvPortDirection::Output => "Output".to_string(),
            SvPortDirection::Ref => "Ref".to_string(),
            SvPortDirection::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}

/// Data kinds.
///
/// Args:
///    Net (SvDataKind): A net.
///    Variable (SvDataKind): A variable.
///    IMPLICIT (SvDataKind): An implicit data kind.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvDataKind {
    Net,
    Variable,
    IMPLICIT,
}

#[pymethods]
impl SvDataKind {
    fn __repr__(&self) -> String {
        match self {
            SvDataKind::Net => "Net".to_string(),
            SvDataKind::Variable => "Variable".to_string(),
            SvDataKind::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}

/// Signedness.
///
/// Args:
///   Signed (SvSignedness): A signed value.
///   Unsigned (SvSignedness): An unsigned value.
///   Unsupported (SvSignedness): An unsupported value.
///   IMPLICIT (SvSignedness): An implicit value.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum SvSignedness {
    Signed,
    Unsigned,
    Unsupported,
    IMPLICIT,
}

#[pymethods]
impl SvSignedness {
    fn __repr__(&self) -> String {
        match self {
            SvSignedness::Signed => "Signed".to_string(),
            SvSignedness::Unsigned => "Unsigned".to_string(),
            SvSignedness::Unsupported => "Unsupported".to_string(),
            SvSignedness::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}

/// Data types.
///
/// Args:
///     Logic (SvDataType): A logic type.
///     Reg (SvDataType): A reg type.
///     Bit (SvDataType): A bit type.
///     Byte (SvDataType): A byte type.
///     Integer (SvDataType): An integer type.
///     Int (SvDataType): An int type.
///     Shortint (SvDataType): A shortint type.
///     Longint (SvDataType): A longint type.
///     Time (SvDataType): A time type.
///     Real (SvDataType): A real type.
///     Shortreal (SvDataType): A shortreal type.
///     Realtime (SvDataType): A realtime type.
///     Array (SvDataType): An array type.
///     Enum (SvDataType): An enum type.
///     Struct (SvDataType): A struct type.
///     Union (SvDataType): A union type.
///     Class (SvDataType): A class type.
///     TypeRef (SvDataType): A type reference.
///     String (SvDataType): A string type.
///     Unsupported (SvDataType): An unsupported type.
///     IMPLICIT (SvDataType): An implicit type.
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

#[pymethods]
impl SvDataType {
    fn __repr__(&self) -> String {
        match self {
            SvDataType::Logic => "Logic".to_string(),
            SvDataType::Reg => "Reg".to_string(),
            SvDataType::Bit => "Bit".to_string(),
            SvDataType::Byte => "Byte".to_string(),
            SvDataType::Integer => "Integer".to_string(),
            SvDataType::Int => "Int".to_string(),
            SvDataType::Shortint => "Shortint".to_string(),
            SvDataType::Longint => "Longint".to_string(),
            SvDataType::Time => "Time".to_string(),
            SvDataType::Real => "Real".to_string(),
            SvDataType::Shortreal => "Shortreal".to_string(),
            SvDataType::Realtime => "Realtime".to_string(),
            SvDataType::Array => "Array".to_string(),
            SvDataType::Enum => "Enum".to_string(),
            SvDataType::Struct => "Struct".to_string(),
            SvDataType::Union => "Union".to_string(),
            SvDataType::Class => "Class".to_string(),
            SvDataType::TypeRef => "TypeRef".to_string(),
            SvDataType::String => "String".to_string(),
            SvDataType::Unsupported => "Unsupported".to_string(),
            SvDataType::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}

/// Net types.
///
/// Args:
///     Wire (SvNetType): A wire.
///     Uwire (SvNetType): An uwire.
///     Tri (SvNetType): A tri.
///     Wor (SvNetType): A wor.
///     Wand (SvNetType): A wand.
///     Triand (SvNetType): A triand.
///     Trior (SvNetType): A trior.
///     Trireg (SvNetType): A trireg.
///     Tri0 (SvNetType): A tri0.
///     Tri1 (SvNetType): A tri1.
///     Supply0 (SvNetType): A supply0.
///     Supply1 (SvNetType): A supply1.
///     IMPLICIT (SvNetType): An implicit net type.
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

#[pymethods]
impl SvNetType {
    fn __repr__(&self) -> String {
        match self {
            SvNetType::Wire => "Wire".to_string(),
            SvNetType::Uwire => "Uwire".to_string(),
            SvNetType::Tri => "Tri".to_string(),
            SvNetType::Wor => "Wor".to_string(),
            SvNetType::Wand => "Wand".to_string(),
            SvNetType::Triand => "Triand".to_string(),
            SvNetType::Trior => "Trior".to_string(),
            SvNetType::Trireg => "Trireg".to_string(),
            SvNetType::Tri0 => "Tri0".to_string(),
            SvNetType::Tri1 => "Tri1".to_string(),
            SvNetType::Supply0 => "Supply0".to_string(),
            SvNetType::Supply1 => "Supply1".to_string(),
            SvNetType::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}
/// Packed dimensions.
/// The first element is the left bound, the second is the right bound.
pub type SvPackedDimension = (String, String);

/// Unpacked dimensions.
/// The first element is the left bound, the second is the right bound.
pub type SvUnpackedDimension = (String, Option<String>);

/// Ports.
///
/// Args:
///    identifier (str): The identifier of the port.
///    direction (SvPortDirection): The direction of the port.
///    datakind (SvDataKind): The data kind of the port.
///    datatype (SvDataType): The data type of the port.
///    classid (str): The class identifier of the port.
///    nettype (SvNetType): The net type of the port.
///    signedness (SvSignedness): The signedness of the port.
///    packed_dimensions (List[SvPackedDimension]): The packed dimensions of the port.
///    unpacked_dimensions (List[SvUnpackedDimension]): The unpacked dimensions of the port.
///    comment (List[str] | None): The comment of the port.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvPort {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub direction: SvPortDirection,
    #[pyo3(get, set)]
    pub datakind: SvDataKind,
    #[pyo3(get, set)]
    pub datatype: SvDataType,
    #[pyo3(get, set)]
    pub classid: Option<String>,
    #[pyo3(get, set)]
    pub nettype: Option<SvNetType>,
    #[pyo3(get, set)]
    pub signedness: Option<SvSignedness>,
    #[pyo3(get, set)]
    pub packed_dimensions: Vec<SvPackedDimension>,
    #[pyo3(get, set)]
    pub unpacked_dimensions: Vec<SvUnpackedDimension>,
    #[pyo3(get, set)]
    pub comment: Option<Vec<String>>,
}

/// Instances.
///
/// Args:
///    module_identifier (str): The module identifier of the instance.
///    hierarchical_instance (str): The hierarchical instance of the instance.
///    hierarchy (List[str]): The hierarchy of the instance.
///    connections (List[List[str]]): The connections of the instance.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvInstance {
    #[pyo3(get, set)]
    pub module_identifier: String,
    #[pyo3(get, set)]
    pub hierarchical_instance: String,
    #[pyo3(get, set)]
    pub hierarchy: Vec<String>,
    #[pyo3(get, set)]
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

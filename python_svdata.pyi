from enum import Enum

SvPackedDimension = tuple[str, str]
SvUnpackedDimension = tuple[str, str | None]

class SvParamType(Enum):
    Paramater = "Parameter"
    LocalParam = "LocalParam"

class SvPortDirection(Enum):
    Inout = "Inout"
    Input = "Input"
    Output = "Output"
    Ref = "Ref"
    IMPLICIT = "IMPLICIT"

class SvDataKind(Enum):
    Net = "Net"
    Variable = "Variable"
    IMPLICIT = "IMPLICIT"

class SvDataType(Enum):
    Logic = "Logic"
    Reg = "Reg"
    Bit = "Bit"
    Byte = "Byte"
    Integer = "Integer"
    Int = "Int"
    Shortint = "Shortint"
    Longint = "Longint"
    Time = "Time"
    Real = "Real"
    Shortreal = "Shortreal"
    Realtime = "Realtime"
    Array = "Array"
    Enum = "Enum"
    Struct = "Struct"
    Union = "Union"
    Class = "Class"
    TypeRef = "TypeRef"
    String = "String"
    Unsupported = "Unsupported"
    IMPLICIT = "IMPLICIT"

class SvSignedness(Enum):
    Signed = "Signed"
    Unsigned = "Unsigned"
    Unsupported = "Unsupported"
    IMPLICIT = "IMPLICIT"

class SvNetType(Enum):
    Wire = "Wire"
    Uwire = "Uwire"
    Tri = "Tri"
    Wor = "Wor"
    Wand = "Wand"
    Triand = "Triand"
    Trior = "Trior"
    Trireg = "Trireg"
    Tri0 = "Tri0"
    Tri1 = "Tri1"
    Supply0 = "Supply0"
    Supply1 = "Supply1"
    IMPLICIT = "IMPLICIT"

class SvInstance:
    module_identifier: str
    hierarchical_identifier: str
    hierarchy: list[str]
    connections: list[list[str]]

class SvParameter:
    identifier: str
    expression: str | None
    paramtype: SvParamType
    datatype: SvDataType | None
    datatype_overridable: bool
    classid: str | None
    signedness: SvSignedness | None
    signedness_overridable: bool
    num_bits: int | None
    packed_dimensions: list[SvPackedDimension]
    unpacked_dimensions: list[SvUnpackedDimension]
    comment: list[str]

class SvPort:
    identifier: str
    direction: SvPortDirection
    datakind: SvDataKind
    datatype: SvDataType
    classid: str | None
    nettype: SvNetType | None
    signedness: SvSignedness | None
    packed_dimensions: list[SvPackedDimension]
    unpacked_dimensions: list[SvUnpackedDimension]
    comment: list[str] | None

class SvModuleDeclaration:
    identifier: str
    parameters: list[SvParameter]
    ports: list[SvPort]
    instances: list[SvInstance]
    filepath: str
    comments: list[str]

class SvPackageDeclaration:
    identifier: str
    parameters: list[SvParameter]
    filepath: str

class SvData:
    modules: list[SvModuleDeclaration]
    packages: list[SvPackageDeclaration]

def read_sv_file(file_path: str) -> SvData: ...

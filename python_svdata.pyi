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

class SvModuleDeclaration:
    identifier: str
    parameters: list[SvParameter]
    ports: list[SvPort]
    instances: list[SvInstance]
    filepath: str
    comments: list[str]

class SvData:
    modules: list[SvModuleDeclaration]
    packages: list[SvPackageDeclaration]

def read_sv_file(file_path: str) -> SvData: ...

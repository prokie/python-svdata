from python_svdata import SvDataKind, SvDataType, SvPortDirection, read_sv_file


ansi_module_a = read_sv_file("tests/systemverilog/ansi_module.sv").modules[0]


def test_module_name() -> None:
    assert ansi_module_a.identifier == "ansi_module_a"
    assert ansi_module_a.ports[0].identifier == "a"


def test_module_ports() -> None:
    assert str(ansi_module_a.ports[0].direction) == "Input"
    assert ansi_module_a.ports[0].direction == SvPortDirection.Input

    assert str(ansi_module_a.ports[0].datakind) == "Variable"
    assert ansi_module_a.ports[0].datakind == SvDataKind.Variable

    assert str(ansi_module_a.ports[0].datatype) == "Logic"
    assert ansi_module_a.ports[0].datatype == SvDataType.Logic

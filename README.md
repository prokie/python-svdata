[![Documentation Status](https://readthedocs.org/projects/python-svdata/badge/?version=latest)](https://python-svdata.readthedocs.io/en/latest/?badge=latest)

# Python Svdata

This is a copy of the excellent repository https://github.com/davemcewan/svdata,
but ported for use in python instead of serializing.

## Installation

```bash
pip install python-svdata
```

## Usage

To parse a systemverilog file simply import the function `read_sv_file`
and specify the path to the file.

```python
from python_svdata import read_sv_file

sv_data = read_sv_file("test.sv")
```

import nox


@nox.session(venv_backend="none")
def format_check(s: nox.Session) -> None:
    s.run("isort", "--check", "python_svdata.pyi")
    s.run("black", "--check", "python_svdata.pyi")


@nox.session(venv_backend="none")
def lint(s: nox.Session) -> None:
    s.run("pflake8", "--color", "always")


@nox.session(venv_backend="none")
def type_check(s: nox.Session) -> None:
    s.run("mypy", "python_svdata.pyi", "noxfile.py")


@nox.session(venv_backend="none")
def test(s: nox.Session) -> None:
    s.run("pytest", "-s", "-v", "--cov")
    s.run("coverage", "report")
    s.run("coverage", "xml")


@nox.session(venv_backend="none")
def docs(s: nox.Session) -> None:
    s.run("sphinx-build", "-b", "html", "docs/source", "docs/build/html", "-W", "--keep-going")

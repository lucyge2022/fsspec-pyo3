from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="lucy-rust-package",
    version="0.1",
    rust_extensions=[RustExtension("lucyrust", binding=Binding.PyO3)],
    packages=["lucyrust"],
    # include any other necessary package metadata
    zip_safe=False,
)

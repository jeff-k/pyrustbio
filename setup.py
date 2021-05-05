from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyrustbio",
    version="0.2.0",
    rust_extensions=[
        RustExtension(
            "pyrustbio.pyrustbio", "Cargo.toml", binding=Binding.PyO3, debug=False
        )
    ],
    include_package_data=True,
    packages=["pyrustbio"],
    zip_safe=False,
)

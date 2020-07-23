from setuptools import setup, find_packages
setup(
    name="arrowlab",
    version="0.1",
    packages=find_packages(include=["python"]),
    # other arguments here...
    entry_points={
        "console_scripts": [
            "alab = python.cli:main"
        ]
    }
)
from setuptools import setup

with open('README.md') as f:
    long_description = f.read()

setup(
    name='dlog_python',
    version='0.1.6',
    python_requires=">=3.8.0",
    author='Lennart Voorgang',
    author_email='lennart@voorgang.dev',
    description='Python adapter for the dlog logging platform',
    long_description=long_description,
    long_description_content_type='text/markdown',
    homepage='https://github.com/lennartvrg/dlog/python',
    repository='https://github.com/lennartvrg/dlog/python',
    documentation='https://github.com/lennartvrg/dlog/python',
)

from setuptools import setup, find_packages
import os

# Read the contents of README file
this_directory = os.path.abspath(os.path.dirname(__file__))
with open(os.path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name='halt-py',
    version='0.1.0',
    description='Python bindings for Halt.rs multi-agent proxy',
    long_description=long_description,
    long_description_content_type='text/markdown',
    author='Halt.rs Team',
    author_email='halt@halt.rs',
    url='https://github.com/halt-rs/halt-py',
    packages=find_packages(),
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
        'Programming Language :: Python :: 3.12',
        'Operating System :: OS Independent',
        'Topic :: Software Development :: Libraries :: Python Modules',
    ],
    python_requires='>=3.8',
    install_requires=[
        'requests>=2.25.0',
        'websockets>=10.0',
        'pydantic>=1.8.0',
        'asyncio-mqtt>=0.11.0',
    ],
    extras_require={
        'dev': [
            'pytest>=6.0',
            'pytest-asyncio>=0.15.0',
            'black>=21.0',
            'isort>=5.0',
            'mypy>=0.900',
            'flake8>=3.9',
        ],
    },
    entry_points={
        'console_scripts': [
            'halt-py=halt.cli:main',
        ],
    },
)

from setuptools import setup, find_packages

setup(
    name='kolos',
    version='0.0.1',
    description='Kolos prototype',
    packages=find_packages(exclude=('tests', 'kernel', 'kernel.*')),
    py_modules=['kolos_cli'],
    entry_points={'console_scripts': ['kolos=kolos_cli:main']},
)

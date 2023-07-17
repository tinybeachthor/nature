from setuptools import setup

with open("requirements.txt", "r") as file:
    requires = file.readlines()

setup(name='shell',
      install_requires=requires,
      zip_safe=False,
      include_package_data=True,
      )

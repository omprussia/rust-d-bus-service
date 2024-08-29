# Example of the simple D-Bus service running on system bus

The purpose of the project is to show how to make D-Bus service that:
- works on system bus,
- starts only on request,
- available only for the root.

This example is written in Rust, contains all required configuration files and can be packed in rpm package for installation on the RPM-based OS.

You can use it as a basis for you own project.

# Build

## Prerequisites
To build this project you will need OS GNU/Linux with installed Rust in it.

If you want to build rpm package, then you will also need rpmbuild.

## Build'n'pack
To build project and make rpm package just run ./build.sh script.

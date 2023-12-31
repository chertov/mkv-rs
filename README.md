This repository contains an experimental Rust package for parsing and serializing the EBML/MKV format based on the Matroska specification.

Note: This project is currently under development and is in an experimental stage. It is not recommended for production use.

## About the Project
The EBML/MKV format is an open standard multimedia container format designed for storing audio, video, and subtitle data. This project aims to provide a Rust implementation for parsing and serializing Matroska files, leveraging the power and safety guarantees of the Rust programming language.


This repository contains two packages: mkv and mkv_codegen. Please note that mkv_codegen is a purely technical package and will not be included as a dependency in the final product. It is specifically designed for parsing the specification files, ebml_matroska.xml and matroska_tags.xml. With few exceptions, almost all structures in the mkv package are generated from these specification files. The code generated by mkv_codegen is stored in the mkv/src/gen directory.
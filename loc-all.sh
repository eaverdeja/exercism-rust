#!/bin/bash

# runs tokei excluding tests, markdown and TOML files
tokei . -e tests/ -e \*.md -e \*.toml -e \*.sh -f -s code

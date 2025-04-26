#!/bin/bash

# runs tokei excluding tests, markdown, TOML files and other assets
tokei . -e tests/ -e \*.md -e \*.toml -e \*.sh -e \*.svg -f -s code

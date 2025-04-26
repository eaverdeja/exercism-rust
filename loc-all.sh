#!/bin/bash

# runs `tokei` excluding tests, markdown, TOML files and other assets
# prints file by file, sorting by lines of code
# limits width to 100 columns
tokei . -e tests/ -e \*.md -e \*.toml -e \*.sh -e \*.svg -f -s code -c 100

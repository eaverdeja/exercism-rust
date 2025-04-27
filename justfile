default:
    just --list

# runs `tokei` excluding tests, markdown, TOML files and other assets. Prints file by file, sorting by lines of code. Limits width to 100 columns
count-loc:
    tokei . -e tests/ -e \*.md -e \*.toml -e \*.sh -e \*.svg -f -s code -c 100

# tests all projects in the workspace
test-all:
    time cargo test --workspace --test '*'

# generates a report indicating the projects with the slowest test suites
test-sorted:
    ./test-sorted.sh

# updats the workspace when a new exercise is added
update:
    ./update-workspace.sh

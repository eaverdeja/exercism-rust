#!/bin/bash

echo "Running all tests and measuring execution time per crate..."

# Capture the start time for total execution using a more reliable approach
TIMEFORMAT="%R"
exec 3>&1
total_runtime=$( { time ( cargo test --workspace --test '*' > test_output.txt 2>&1 ) 1>&3; } 2>&1 )
exec 3>&-

# Create temporary files
tmp_results=$(mktemp)

# Initialize variables for tracking current crate
current_crate=""
current_file=""
tests_in_crate=0

# Extract test timing for each crate
while IFS= read -r line; do
    if [[ $line =~ Running\ tests/(.*).rs\ \(target/debug/deps/([^-]*)-[a-z0-9]+\) ]]; then
        # Found a test file being run - save the crate name
        current_file="${BASH_REMATCH[1]}"
        current_crate="${BASH_REMATCH[2]}"
        tests_in_crate=0
    # The regex below has been fixed to escape the semicolons properly
    elif [[ $line =~ "test result: ".*" "([0-9]+)" passed; "([0-9]+)" failed; "([0-9]+)" ignored".*"finished in "([0-9.]+)"s" ]]; then
        # Found the completion line with test count and time
        passed="${BASH_REMATCH[1]}"
        failed="${BASH_REMATCH[2]}"
        ignored="${BASH_REMATCH[3]}"
        exec_time="${BASH_REMATCH[4]}"
        tests_in_crate=$((passed + failed))
        
        # Only add results if we have a current crate (skips global summary)
        if [ -n "$current_crate" ]; then
            echo "$exec_time $current_crate ($current_file) - $tests_in_crate tests" >> "$tmp_results"
        fi
    fi
done < test_output.txt

# Calculate total test count with proper print statements
total_tests=$(grep -o "[0-9]* passed" test_output.txt | awk '{sum += $1} END {print sum}')
total_passed=$(grep -o "[0-9]* passed" test_output.txt | awk '{sum += $1} END {print sum}')
total_failed=$(grep -o "[0-9]* failed" test_output.txt | awk '{sum += $1} END {print sum}')
total_ignored=$(grep -o "[0-9]* ignored" test_output.txt | awk '{sum += $1} END {print sum}')

# Check if we found any results
if [ ! -s "$tmp_results" ]; then
    echo "No test timing information found."
    echo "Output sample:"
    head -n 20 test_output.txt
else
    echo ""
    echo "Tests sorted by execution time (fastest first):"
    echo "----------------------------------------"
    
    # Sort and format the results
    sort -n "$tmp_results" | awk '{printf "%.4fs %s\n", $1, substr($0, length($1)+2)}'
    
    echo ""
    echo "----------------------------------------"
    echo "Total execution time: ${total_runtime}s"
    echo "Total tests: $total_tests, Passed: $total_passed, Failed: $total_failed, Ignored: $total_ignored"
fi

# Clean up
rm test_output.txt "$tmp_results"
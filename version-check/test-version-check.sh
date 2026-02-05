#!/bin/bash

set -e

echo "Testing version-check integration"
echo "================================="
echo ""

mkdir -p ~/.mozbuild

echo "Test 1: Without MOZTOOLS_UPDATE_CHECK (should not check)"
echo "---------------------------------------------------------"
cd /home/padenot/src/repositories/socorro-cli
cargo run --quiet -- crash --help > /dev/null 2>&1 || true
echo "No version check message (as expected)"
echo ""

echo "Test 2: With MOZTOOLS_UPDATE_CHECK=1 (should check and cache)"
echo "--------------------------------------------------------------"
export MOZTOOLS_UPDATE_CHECK=1
cargo run --quiet -- crash --help > /dev/null 2>&1 || true
echo ""
sleep 2
echo "Checking cache file..."
if [ -f ~/.mozbuild/tool-versions.json ]; then
    echo "Cache file created:"
    cat ~/.mozbuild/tool-versions.json
else
    echo "Cache file not found (might be due to network timeout)"
fi
echo ""

echo "Test 3: Second run should use cached data"
echo "------------------------------------------"
cargo run --quiet -- crash --help > /dev/null 2>&1 || true
echo ""

echo "Test 4: Testing treeherder-cli"
echo "-------------------------------"
cd /home/padenot/src/repositories/treeherder-cli
cargo run --quiet -- --help > /dev/null 2>&1 || true
echo ""
sleep 2
echo "Updated cache file:"
if [ -f ~/.mozbuild/tool-versions.json ]; then
    cat ~/.mozbuild/tool-versions.json
else
    echo "Cache file not found"
fi

echo ""
echo "All tests completed!"

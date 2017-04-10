#!/usr/bin/env bash

REPO_URL="https://github.com/versioneye/veye-checker.git"
WORK_DIR="${HOME}/Ideaprojects/veye-checker/provision/osx/veye-checker"
TEST_DIR="${WORK_DIR}/tests/acceptance"

RELEASE_DIR="${HOME}/Ideaprojects/releases"
TIMESTAMP=$(date +"%s")

if [[ -n "$RELEASE_VERSION" ]]; then
    RELEASE_VERSION="_build_${TIMESTAMP}"
fi

RELEASE_PATH="${RELEASE_DIR}/veye_checker_x86_64_${RELEASE_VERSION}"

if [ ! -d "$WORK_DIR" ]; then
    git clone ${REPO_URL}
fi

echo "Pulling latest code from master"
cd ${WORK_DIR}
git pull

if [ ! -d "temp" ]; then
    echo "Add temp folder to keep test results"
    mkdir -p temp
fi

echo "Running unit-tests"
#due the configs_test it must be single threaded to avoid conflicts in ENV var
cargo test -- --test-threads=1
if [ $? -ne 0 ]; then
    echo "Failed to pass unit tests"
    exit
fi

echo "Compiling debug version"
cd ${WORK_DIR}
cargo build

#ps: files are ordered differently on OSx
echo "Running acceptance tests against debug release..."
cd ${TEST_DIR}
export VERSIONEYE_BIN_PATH="../../target/debug/veye_checker"
bash tests_osx.sh
if [ $? -ne 0 ]; then
    echo "Failed to pass acceptance tests on debug release"
    exit
fi

echo "release binary into ${RELEASE_PATH}"
cd ${WORK_DIR}
cargo build --release
cp target/release/veye_checker ${RELEASE_PATH}

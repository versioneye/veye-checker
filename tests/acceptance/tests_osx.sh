#!/usr/bin/env bash

set -e
. assert.sh

# run as VERSIONEYE_API_KEY="yourkey" ./run.sh
[ -z "$VERSIONEYE_API_KEY" ] && echo "Need to set VERSIONEYE_API_KEY" && exit 1;

if [ -z ${VERSIONEYE_BIN_PATH+x} ]; then
    VERSIONEYE_BIN_PATH="../../target/debug/veye_checker"
    echo "Using default binary ${VERSIONEYE_BIN_PATH}"
else
    echo "Using specified binary at ${VERSIONEYE_BIN_PATH}"
fi

FIXTURES_PATH="../fixtures/files"
FILE_SHA="5675fd96b29656504b86029551973d60fb41339b"
PYPI_SHA="fe7daf822f1d36d1bd37ac41cf5817e7"
NPM_SHA="6f631aef336d6c46362b51764044ce216be3c051"

echo "#-- initializing expected results"

define(){ IFS='\n' read -r -d '' ${1} || true; }

define EXPECTED1 <<EOF
path_scanner: skipping "../fixtures/files"
filepath;packaging;sha_method;sha_value
../fixtures/files/npm.tgz;;sha1;6f631aef336d6c46362b51764044ce216be3c051
../fixtures/files/pypi.tar.gz;;md5;fe7daf822f1d36d1bd37ac41cf5817e7
../fixtures/files/pypi.whl;;md5;ffa1ee60be515c04b4c13fd13feea27a
../fixtures/files/test.jar;;sha1;5675fd96b29656504b86029551973d60fb41339b
../fixtures/files/test.nupkg;;sha512;U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==
Done!
EOF

define EXPECTED2 <<EOF
filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error
;;;5675fd96b29656504b86029551973d60fb41339b;java;commons-beanutils/commons-beanutils;1.7.0;1;https://www.versioneye.com/java/commons-beanutils/commons-beanutils/1.7.0;unknown;\nDone!
EOF

define EXPECTED3 << EOF
filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error
;;;fe7daf822f1d36d1bd37ac41cf5817e7;python;restea;0.3.4;0;https://www.versioneye.com/python/restea/0.3.4;MIT;\nDone!
EOF

define EXPECTED4 << EOF
filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error
;;;6f631aef336d6c46362b51764044ce216be3c051;nodejs;etag;1.8.0;0;https://www.versioneye.com/nodejs/etag/1.8.0;MIT;\nDone!
EOF

define EXPECTED5 << EOF
path_scanner: skipping "../fixtures/files"
filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error
../fixtures/files/npm.tgz;;sha1;6f631aef336d6c46362b51764044ce216be3c051;nodejs;etag;1.8.0;0;https://www.versioneye.com/nodejs/etag/1.8.0;MIT;
../fixtures/files/pypi.tar.gz;;md5;fe7daf822f1d36d1bd37ac41cf5817e7;python;restea;0.3.4;0;https://www.versioneye.com/python/restea/0.3.4;MIT;
../fixtures/files/pypi.whl;;md5;ffa1ee60be515c04b4c13fd13feea27a;python;wheel;0.30.0a0;0;https://www.versioneye.com/python/wheel/0.30.0a0;MIT;
../fixtures/files/test.jar;;sha1;5675fd96b29656504b86029551973d60fb41339b;java;commons-beanutils/commons-beanutils;1.7.0;1;https://www.versioneye.com/java/commons-beanutils/commons-beanutils/1.7.0;unknown;
../fixtures/files/test.nupkg;;sha512;U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==;csharp;Newtonsoft.Json;9.0.1;0;https://www.versioneye.com/csharp/Newtonsoft.Json/9.0.1;MIT;\nDone!
EOF

echo "#-- Going to execute acceptance tests"
echo "#-- shas command"
assert "exec ${VERSIONEYE_BIN_PATH} shas ${FIXTURES_PATH}" "${EXPECTED1}"

echo "#-- lookup FILE_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${FILE_SHA}"  "${EXPECTED2}"

echo "#-- lookup PYPI_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${PYPI_SHA}" "${EXPECTED3}"

echo "#-- lookup NPM_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${NPM_SHA}" "${EXPECTED4}"

echo "#-- resolve command"
assert "exec ${VERSIONEYE_BIN_PATH} resolve ${FIXTURES_PATH}" "${EXPECTED5}"

assert_end

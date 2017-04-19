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

echo "#-- Going to execute acceptance tests"
echo "#-- shas command"
assert "exec ${VERSIONEYE_BIN_PATH} shas ${FIXTURES_PATH}" \
    "filepath;packaging;sha_method;sha_value\n../fixtures/files/npm.tgz;npm;sha1;6f631aef336d6c46362b51764044ce216be3c051\n../fixtures/files/pypi.tar.gz;pypi;md5;fe7daf822f1d36d1bd37ac41cf5817e7\n../fixtures/files/pypi.whl;pypi;md5;ffa1ee60be515c04b4c13fd13feea27a\n../fixtures/files/test.jar;jar;sha1;5675fd96b29656504b86029551973d60fb41339b\n../fixtures/files/test.nupkg;nupkg;sha512;U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==\nDone!"

echo "#-- lookup FILE_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${FILE_SHA}" \
    "filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error\n;;;5675fd96b29656504b86029551973d60fb41339b;java;commons-beanutils/commons-beanutils;1.7.0;1;https://www.versioneye.com/java/commons-beanutils:commons-beanutils/1.7.0;unknown;\nDone!"

echo "#-- lookup PYPI_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${PYPI_SHA}" \
    "filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error\n;;;fe7daf822f1d36d1bd37ac41cf5817e7;python;restea;0.3.4;0;https://www.versioneye.com/python/restea/0.3.4;MIT;\nDone!"

echo "#-- lookup NPM_SHA"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${NPM_SHA}" \
    "filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error\n;;;6f631aef336d6c46362b51764044ce216be3c051;nodejs;etag;1.8.0;0;https://www.versioneye.com/nodejs/etag/1.8.0;MIT;\nDone!"

echo "#-- resolve command"
assert "exec ${VERSIONEYE_BIN_PATH} resolve ${FIXTURES_PATH}" \
    "filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error\n../fixtures/files/npm.tgz;npm;sha1;6f631aef336d6c46362b51764044ce216be3c051;nodejs;etag;1.8.0;0;https://www.versioneye.com/nodejs/etag/1.8.0;MIT;\n../fixtures/files/pypi.tar.gz;pypi;md5;fe7daf822f1d36d1bd37ac41cf5817e7;python;restea;0.3.4;0;https://www.versioneye.com/python/restea/0.3.4;MIT;\n../fixtures/files/pypi.whl;pypi;md5;ffa1ee60be515c04b4c13fd13feea27a;python;wheel;0.30.0a0;0;https://www.versioneye.com/python/wheel/0.30.0a0;MIT;\n../fixtures/files/test.jar;jar;sha1;5675fd96b29656504b86029551973d60fb41339b;java;commons-beanutils/commons-beanutils;1.7.0;1;https://www.versioneye.com/java/commons-beanutils:commons-beanutils/1.7.0;unknown;\n../fixtures/files/test.nupkg;nupkg;sha512;U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==;csharp;Newtonsoft.Json;9.0.1;0;https://www.versioneye.com/csharp/Newtonsoft~Json/9.0.1;MIT;\nDone!"
assert_end

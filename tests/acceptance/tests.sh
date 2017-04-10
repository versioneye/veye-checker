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

echo "#-- Going to execute acceptance tests"
echo "#-- shas command"
assert "exec ${VERSIONEYE_BIN_PATH} shas ${FIXTURES_PATH}" \
 "filepath,packaging,sha_method,sha_value\n../fixtures/files/test.nupkg,nupkg,sha512,U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==\n../fixtures/files/test.jar,jar,sha1,5675fd96b29656504b86029551973d60fb41339b\nDone!"

echo "#-- lookup command"
assert "exec ${VERSIONEYE_BIN_PATH} lookup ${FILE_SHA}" \
 "filepath,packaging,sha_method,sha_value,language,prod_key,version,n_vulns,product_url,license,error\n,,,5675fd96b29656504b86029551973d60fb41339b,java,commons-beanutils/commons-beanutils,1.7.0,1,https://www.versioneye.com/Java/commons-beanutils:commons-beanutils/1.7.0,unknown,\nDone!"

echo "#-- resolve command"
assert "exec ${VERSIONEYE_BIN_PATH} resolve ${FIXTURES_PATH}" \
    "filepath,packaging,sha_method,sha_value,language,prod_key,version,n_vulns,product_url,license,error\n../fixtures/files/test.nupkg,nupkg,sha512,U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==,csharp,Newtonsoft.Json,9.0.1,0,https://www.versioneye.com/CSharp/Newtonsoft~Json/9.0.1,MIT,\n../fixtures/files/test.jar,jar,sha1,5675fd96b29656504b86029551973d60fb41339b,java,commons-beanutils/commons-beanutils,1.7.0,1,https://www.versioneye.com/Java/commons-beanutils:commons-beanutils/1.7.0,unknown,\nDone!"

assert_end

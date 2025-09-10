#!/bin/bash

set -ex

export AWS_PAGER=""

OUTPUT_PATH=$(bazel cquery -c opt //server:image_multiarch --output=starlark --starlark:expr='target.files.to_list()[0].path')
bazel run -c opt //server:push --stamp
DIGEST=$(cat $OUTPUT_PATH/index.json | jq -r '.manifests[0].digest')
aws lambda update-function-code --function-name=implfuture --image-uri="689191389309.dkr.ecr.us-west-1.amazonaws.com/implfuture@${DIGEST}"

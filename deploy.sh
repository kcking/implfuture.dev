#!/bin/bash

set -ex

export AWS_PAGER=""

bazel run -c opt //server:push-amd64
aws lambda update-function-code --function-name=implfuture --image-uri=689191389309.dkr.ecr.us-west-1.amazonaws.com/implfuture@$(cat target/bin/server/push-amd64.digest)

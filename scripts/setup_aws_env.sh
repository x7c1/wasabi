#!/usr/bin/env bash

# Usage:
# $ source ./scripts/setup_aws_env.local.sh "profile-to-load"

export AWS_PROFILE=$1

AWS_ACCESS_KEY_ID=$(aws configure get aws_access_key_id)
export AWS_ACCESS_KEY_ID

AWS_SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)
export AWS_SECRET_ACCESS_KEY

AWS_DEFAULT_REGION=$(aws configure get region)
export AWS_DEFAULT_REGION

aws configure list

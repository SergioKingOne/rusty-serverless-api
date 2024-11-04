#!/bin/bash
set -a
source .env
set +a

# Convert .env variables to TF_VAR_ format
export TF_VAR_aws_access_key_id=$AWS_ACCESS_KEY_ID
export TF_VAR_aws_secret_access_key=$AWS_SECRET_ACCESS_KEY
export TF_VAR_aws_region=$AWS_REGION
export TF_VAR_lambda_role_name=$AWS_LAMBDA_ROLE_NAME
export TF_VAR_dynamodb_table_name=$AWS_DYNAMODB_TABLE_NAME
export TF_VAR_api_name=$AWS_API_NAME

cd terraform

# Run terraform command with all arguments passed to this script
terraform "$@"
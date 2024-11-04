#!/bin/bash

# Check for --force or -f flag
if [[ "$1" != "--force" && "$1" != "-f" ]]; then
    read -p "Warning: This script will deploy infrastructure changes. Continue? (y/N) " confirm
    if [[ $confirm != [yY] ]]; then
        echo "Deployment cancelled"
        exit 1
    fi
fi

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

# Build the project
cargo lambda build --release

# Create lambda directory if it doesn't exist
mkdir -p terraform/lambda

# zip the binary and copy to terraform/lambda
cd target/lambda/rusty_serverless_api
zip -j lambda_function.zip bootstrap
mv lambda_function.zip ../../../terraform/lambda/

cd ../../../terraform

terraform init
terraform apply -auto-approve
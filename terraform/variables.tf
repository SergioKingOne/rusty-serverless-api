variable "aws_region" {
  description = "The AWS region to deploy to"
  type        = string
  default     = "us-east-1"
}

variable "api_name" {
  description = "Name of the API Gateway"
  type        = string
  default     = "RustyServerlessAPI"
}

variable "lambda_role_name" {
  description = "IAM role name for Lambda"
  type        = string
  default     = "rusty_lambda_role"
}

variable "dynamodb_table_name" {
  description = "DynamoDB table name"
  type        = string
  default     = "RustyServerlessAPI"
}

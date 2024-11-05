import {
  to = aws_iam_role.lambda_role
  id = "rusty-serverless-lambda-role"
}

import {
  to = aws_iam_policy.lambda_policy
  id = "arn:aws:iam::199614859729:policy/rusty_lambda_policy"
}

import {
  to = aws_dynamodb_table.rusty_table
  id = "rusty-serverless-dynamodb-table"
}

import {
  to = aws_lambda_function.rusty_lambda
  id = "rusty_lambda_function"
}

import {
  to = aws_lambda_permission.api_gateway
  id = "rusty_lambda_function/AllowAPIGatewayInvoke"
}

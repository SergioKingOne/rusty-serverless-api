resource "aws_iam_role" "lambda_role" {
  name = var.lambda_role_name

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_iam_policy" "lambda_policy" {
  name        = "rusty_lambda_policy"
  description = "Policy for Rust Lambda functions"
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = [
          "dynamodb:*"
        ],
        Effect   = "Allow",
        Resource = "*"
      },
      {
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ],
        Effect   = "Allow",
        Resource = "arn:aws:logs:*:*:*"
      }
    ]
  })

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_iam_role_policy_attachment" "lambda_attach" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_policy.arn
}

resource "aws_dynamodb_table" "rusty_table" {
  name         = var.dynamodb_table_name
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"

  attribute {
    name = "id"
    type = "S"
  }

  lifecycle {
    prevent_destroy       = false
    create_before_destroy = true
  }
}

resource "aws_lambda_function" "rusty_lambda" {
  filename         = "${path.module}/lambda/lambda_function.zip"
  function_name    = "rusty_lambda_function"
  role             = aws_iam_role.lambda_role.arn
  handler          = "not.required.for.custom.runtime"
  runtime          = "provided.al2"
  source_code_hash = filebase64sha256("${path.module}/lambda/lambda_function.zip")
  memory_size      = 128
  timeout          = 10

  environment {
    variables = {
      DYNAMODB_TABLE = aws_dynamodb_table.rusty_table.name
    }
  }
}

resource "aws_api_gateway_rest_api" "rusty_api" {
  name        = var.api_name
  description = "API Gateway for RustyServerlessAPI"
}

resource "aws_api_gateway_resource" "resource" {
  rest_api_id = aws_api_gateway_rest_api.rusty_api.id
  parent_id   = aws_api_gateway_rest_api.rusty_api.root_resource_id
  path_part   = "items"
}

resource "aws_api_gateway_method" "method" {
  rest_api_id   = aws_api_gateway_rest_api.rusty_api.id
  resource_id   = aws_api_gateway_resource.resource.id
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "integration" {
  rest_api_id             = aws_api_gateway_rest_api.rusty_api.id
  resource_id             = aws_api_gateway_resource.resource.id
  http_method             = aws_api_gateway_method.method.http_method
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.rusty_lambda.invoke_arn
}

resource "aws_lambda_permission" "api_gateway" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.rusty_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_api_gateway_rest_api.rusty_api.execution_arn}/*/*"
}

resource "aws_api_gateway_method" "post" {
  rest_api_id   = aws_api_gateway_rest_api.rusty_api.id
  resource_id   = aws_api_gateway_resource.resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "get" {
  rest_api_id   = aws_api_gateway_rest_api.rusty_api.id
  resource_id   = aws_api_gateway_resource.resource.id
  http_method   = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "put" {
  rest_api_id   = aws_api_gateway_rest_api.rusty_api.id
  resource_id   = aws_api_gateway_resource.resource.id
  http_method   = "PUT"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "delete" {
  rest_api_id   = aws_api_gateway_rest_api.rusty_api.id
  resource_id   = aws_api_gateway_resource.resource.id
  http_method   = "DELETE"
  authorization = "NONE"
}

# Create corresponding integrations for each method
resource "aws_api_gateway_integration" "post" {
  rest_api_id             = aws_api_gateway_rest_api.rusty_api.id
  resource_id             = aws_api_gateway_resource.resource.id
  http_method             = aws_api_gateway_method.post.http_method
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.rusty_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "get" {
  rest_api_id             = aws_api_gateway_rest_api.rusty_api.id
  resource_id             = aws_api_gateway_resource.resource.id
  http_method             = aws_api_gateway_method.get.http_method
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.rusty_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "put" {
  rest_api_id             = aws_api_gateway_rest_api.rusty_api.id
  resource_id             = aws_api_gateway_resource.resource.id
  http_method             = aws_api_gateway_method.put.http_method
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.rusty_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "delete" {
  rest_api_id             = aws_api_gateway_rest_api.rusty_api.id
  resource_id             = aws_api_gateway_resource.resource.id
  http_method             = aws_api_gateway_method.delete.http_method
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.rusty_lambda.invoke_arn
}

# Update the deployment triggers to include all methods and integrations
resource "aws_api_gateway_deployment" "deployment" {
  depends_on = [
    aws_api_gateway_integration.post,
    aws_api_gateway_integration.get,
    aws_api_gateway_integration.put,
    aws_api_gateway_integration.delete
  ]

  rest_api_id = aws_api_gateway_rest_api.rusty_api.id
  stage_name  = "prod"

  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.resource.id,
      aws_api_gateway_method.post.id,
      aws_api_gateway_method.get.id,
      aws_api_gateway_method.put.id,
      aws_api_gateway_method.delete.id,
      aws_api_gateway_integration.post.id,
      aws_api_gateway_integration.get.id,
      aws_api_gateway_integration.put.id,
      aws_api_gateway_integration.delete.id
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

output "api_endpoint" {
  description = "API Gateway endpoint URL"
  value       = aws_api_gateway_rest_api.rusty_api.execution_arn
}

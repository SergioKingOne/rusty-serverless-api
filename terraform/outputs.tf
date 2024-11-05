output "api_endpoint" {
  description = "API Gateway endpoint URL"
  value       = aws_api_gateway_deployment.deployment.invoke_url
}

output "items_endpoint" {
  description = "Items endpoint URL"
  value       = "${aws_api_gateway_deployment.deployment.invoke_url}${aws_api_gateway_resource.resource.path}"
}

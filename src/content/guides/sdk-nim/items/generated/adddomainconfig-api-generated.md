## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| addDomainConfigParams | AddDomainConfigParams | No |  |

## Response

Returns: [`Option[AddDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config200response.nim)

## Example

[inline-code-attrs-start title = 'addDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = AddDomainConfigParams(
  domain = "news.example.com",
  enabled = true,
  allowSubdomains = true,
  allowedOrigins = @["https://news.example.com"],
  description = "Primary domain for news site"
)
let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = params)
if response.isSome:
  let addedConfig = response.get()
  echo "Domain config added for: ", addedConfig.domain
[inline-code-end]

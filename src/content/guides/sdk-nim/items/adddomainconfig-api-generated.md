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
  domain: "comments.newsroom.example",
  enabled: true,
  allowedOrigins: @["https://newsroom.example", "https://www.example.com"]
)

let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = params)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

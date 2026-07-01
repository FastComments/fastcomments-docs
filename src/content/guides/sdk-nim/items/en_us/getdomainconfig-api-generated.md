## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | No |  |

## Response

Returns: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (configOpt, httpResp) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if configOpt.isSome:
  let cfg = configOpt.get()
  discard cfg
[inline-code-end]

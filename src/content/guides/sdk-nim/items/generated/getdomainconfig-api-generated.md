## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | No |  |

## Response

Returns: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.techblog.com")
if response.isSome:
  let domainConfig = response.get()
  echo "Loaded domain config for tenant:", " my-tenant-123"
else:
  echo "No domain config found"
[inline-code-end]

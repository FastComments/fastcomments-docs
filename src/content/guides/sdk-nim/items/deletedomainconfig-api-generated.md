## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | No |  |

## Response

Returns: [`Option[DeleteDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config200response.nim)

## Example

[inline-code-attrs-start title = 'deleteDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]

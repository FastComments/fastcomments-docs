## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | No |  |
| updateDomainConfigParams | UpdateDomainConfigParams | No |  |

## Response

Returns: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news.example.com",
  updateDomainConfigParams = default(UpdateDomainConfigParams)
)
if response.isSome:
  let domainConfig = response.get()
  discard domainConfig
[inline-code-end]

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
  domainToUpdate = "news/article-title",
  updateDomainConfigParams = UpdateDomainConfigParams(
    allowAnonymous = false,
    enableModeration = true,
    allowedOrigins = @["https://news.example.com", "https://cdn.example.com"]
  )
)
if response.isSome:
  let domainConfig = response.get()
  echo "Updated domain config received: ", domainConfig
else:
  echo "No config returned; HTTP status: ", $httpResponse.status
[inline-code-end]

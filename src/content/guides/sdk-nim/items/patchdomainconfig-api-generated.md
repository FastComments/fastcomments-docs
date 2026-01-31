## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | No |  |
| patchDomainConfigParams | PatchDomainConfigParams | No |  |

## Response

Returns: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/example.com",
  patchDomainConfigParams = PatchDomainConfigParams(
    allowedOrigins = @["https://news.example.com", "https://cdn.example.com"],
    moderationEnabled = true,
    maxCommentLength = 2000,
    blockedWords = @["spam", "advertisement"]
  )
)
if response.isSome:
  let config = response.get()
  discard config
[inline-code-end]

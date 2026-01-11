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
let params = PatchDomainConfigParams(
  allowedOrigins = @["https://news.example.com"],
  enableCors = true,
  redirectToHttps = true,
  maxCommentLength = 1000
)

let (response, httpResponse) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news.example.com",
  patchDomainConfigParams = params
)

if response.isSome:
  let config = response.get()
  echo "Updated domain: " & config.domain & " status: " & $httpResponse.status
[inline-code-end]

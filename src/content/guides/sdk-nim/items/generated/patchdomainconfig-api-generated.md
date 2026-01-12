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
let patchParams = PatchDomainConfigParams(
  domainName: "comments.mynews.com",
  enableModeration: true,
  allowedOrigins: @["https://www.mynews.com", "https://editor.mynews.com"]
)

let (response, httpResponse) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/article-title",
  patchDomainConfigParams = patchParams
)

if response.isSome:
  let updated = response.get()
  echo "Updated domain config for: ", updated.domainName
[inline-code-end]

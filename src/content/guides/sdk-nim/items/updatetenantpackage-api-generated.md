## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantPackageBody | UpdateTenantPackageBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateTenantPackage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-456",
  updateTenantPackageBody = UpdateTenantPackageBody(
    name = "Pro Plan",
    description = "Priority support and increased rate limits",
    enabled = true,
    maxUsers = 500,
    allowedFeatures = @["realtime", "moderation", "analytics"]
  )
)

if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

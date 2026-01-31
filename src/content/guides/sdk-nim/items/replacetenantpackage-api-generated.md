## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'replaceTenantPackage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let replaceBody = ReplaceTenantPackageBody(
  name = "Premium Plan",
  features = @["priority-moderation", "advanced-analytics", "api-access"],
  enabled = true,
  seats = 50,
  version = 2
)

let (response, httpResponse) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-789",
  replaceTenantPackageBody = replaceBody
)

if response.isSome:
  let flagResp = response.get()
  echo "ReplaceTenantPackage returned a result for tenant my-tenant-123"
[inline-code-end]

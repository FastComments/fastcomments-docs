## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| replaceTenantUserBody | ReplaceTenantUserBody | No |  |
| updateComments | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'replaceTenantUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.replaceTenantUser(
  tenantId = "my-tenant-123",
  id = "",
  replaceTenantUserBody = ReplaceTenantUserBody(username = "", email = "", roles = @[], active = false),
  updateComments = ""
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

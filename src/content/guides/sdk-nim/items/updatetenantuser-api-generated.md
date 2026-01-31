## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantUserBody | UpdateTenantUserBody | No |  |
| updateComments | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateTenantUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateTenantUserBody = UpdateTenantUserBody(
    displayName = "Jane Roberts",
    email = "jane.roberts@newsportal.com",
    isModerator = true,
    roles = @["editor", "moderator"]
  ),
  updateComments = "Promoted to moderator for News Portal"
)

if response.isSome:
  let flagResp = response.get()
  echo "Received flag response"
else:
  echo "No response body; HTTP status = ", httpResponse.status
[inline-code-end]

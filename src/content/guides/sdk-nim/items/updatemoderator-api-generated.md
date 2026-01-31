## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateModeratorBody | UpdateModeratorBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateModerator Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateModeratorBody = UpdateModeratorBody(
  displayName = "Jane Moderator",
  email = "jane.moderator@example.com",
  active = true,
  roles = @["moderator", "editor"]
)
let (response, httpResponse) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = updateModeratorBody)
if response.isSome:
  let updated = response.get()
  echo "Moderator updated:", updated
else:
  echo "Update failed, HTTP response:", httpResponse.status
[inline-code-end]

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | No |  |

## Response

Returns: [`Option[CreateModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator200response.nim)

## Example

[inline-code-attrs-start title = 'createModerator Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateModeratorBody(
  email = "moderator@news-site.com",
  displayName = "News Moderator",
  permissions = @["approve_comments", "delete_comments"],
  isSuperAdmin = false
)

let (response, httpResponse) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = createBody)

if response.isSome:
  let moderator = response.get()
  echo "Created moderator: ", $moderator
[inline-code-end]

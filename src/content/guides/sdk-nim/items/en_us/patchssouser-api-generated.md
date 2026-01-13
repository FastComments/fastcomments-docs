## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateAPISSOUserData | UpdateAPISSOUserData | No |  |
| updateComments | bool | No |  |

## Response

Returns: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## Example

[inline-code-attrs-start title = 'patchSSOUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = UpdateAPISSOUserData(
    externalId = "ext-987",
    username = "j.smith",
    email = "j.smith@news.example.com",
    displayName = "John Smith",
    roles = @["author", "editor"],
    avatarUrl = "https://cdn.news.example.com/avatars/j.smith.png"
  ),
  updateComments = true
)

if response.isSome:
  let patched = response.get()
  echo patched
[inline-code-end]

---
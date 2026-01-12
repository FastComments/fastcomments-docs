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
let updateData: UpdateAPISSOUserData = UpdateAPISSOUserData(
  name = "Alice Johnson",
  email = "alice.johnson@newsorg.com",
  externalId = "user-789",
  roles = @["subscriber"],
  avatarUrl = "https://cdn.newsorg.com/avatars/user-789.png"
)

let (response, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = updateData,
  updateComments = true
)

if response.isSome:
  let patched = response.get()
  echo patched, " ", httpResponse.statusCode
[inline-code-end]

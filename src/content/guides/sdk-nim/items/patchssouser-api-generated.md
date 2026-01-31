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
  id = "sso-user-456",
  updateAPISSOUserData = UpdateAPISSOUserData(
    displayName = "Jane Reporter",
    email = "jane.reporter@newsco.com",
    externalId = "jane-987",
    avatarUrl = "https://newsco.com/avatars/jane.jpg",
    roles = @["author", "subscriber"]
  ),
  updateComments = false
)

if response.isSome:
  let patched = response.get()
  echo patched
else:
  echo "No response body"
[inline-code-end]

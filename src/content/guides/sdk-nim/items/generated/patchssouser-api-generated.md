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
let updateData: UpdateAPISSOUserData = UpdateAPISSOUserData(email = "jane.doe@example.com", displayName = "Jane Doe", roles = @["editor", "subscriber"], externalId = "sso-789")
let (response, httpResponse) = client.patchSSOUser(tenantId = "my-tenant-123", id = "user-456", updateAPISSOUserData = updateData, updateComments = true)
if response.isSome:
  let patched = response.get()
  discard patched
[inline-code-end]

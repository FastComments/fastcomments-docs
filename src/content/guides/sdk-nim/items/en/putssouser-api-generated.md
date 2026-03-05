## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateAPISSOUserData | UpdateAPISSOUserData | No |  |
| updateComments | bool | No |  |

## Response

Returns: [`Option[PutSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_sso_user_api_response.nim)

## Example

[inline-code-attrs-start title = 'putSSOUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPISSOUserData(
  email: "jane.doe@example.com",
  displayName: "Jane Doe",
  roles: @["editor"],
  avatarUrl: "https://cdn.example.com/avatars/jane.jpg"
)

let (response, httpResponse) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateAPISSOUserData = updateData,
  updateComments = true
)

if response.isSome:
  let updatedUser = response.get()
  discard updatedUser
[inline-code-end]

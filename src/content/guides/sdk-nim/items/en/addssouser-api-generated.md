## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | No |  |

## Response

Returns: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createData = CreateAPISSOUserData(
  id = "user-789",
  email = "jane.doe@example.com",
  username = "jane.doe",
  displayName = "Jane Doe",
  roles = @["author", "editor"],
  avatarUrl = "https://cdn.example.com/avatars/jane.jpg",
  metadata = @[],
  isActive = true
)

let (response, httpResponse) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = createData)

if response.isSome:
  let result = response.get()
  echo "SSO user added: ", result.id
else:
  echo "Failed to add SSO user, HTTP status: ", httpResponse.status.code
[inline-code-end]

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
  externalId = "user-456",
  email = "jane.doe@newsmedia.com",
  displayName = "Jane Doe",
  roles = @["subscriber"],
  isAdmin = false
)
let (response, httpResponse) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = createData)
if response.isSome:
  let added = response.get()
  echo "Added SSO user:", added
[inline-code-end]

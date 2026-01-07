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
let (response, httpResponse) = client.addSSOUser(
  tenantId = "my-tenant-123",
  createAPISSOUserData = CreateAPISSOUserData(
    id = "sso-456",
    email = "alice.johnson@newsorg.com",
    name = "Alice Johnson",
    roles = @["editor", "contributor"],
    isActive = true,
    isAdmin = false
  )
)
if response.isSome:
  let apiResp = response.get()
  discard apiResp
else:
  discard httpResponse
[inline-code-end]

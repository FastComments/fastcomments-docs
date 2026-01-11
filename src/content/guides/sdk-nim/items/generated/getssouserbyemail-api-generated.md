## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| email | string | No |  |

## Response

Returns: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## Example

[inline-code-attrs-start title = 'getSSOUserByEmail Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "jane.doe@newsorg.com")
if response.isSome:
  let ssoUser = response.get()
  echo ssoUser
else:
  echo "SSO user not found"
[inline-code-end]

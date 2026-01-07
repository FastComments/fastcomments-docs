## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | int | No |  |

## Response

Returns: [`Option[GetSSOUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users200response.nim)

## Example

[inline-code-attrs-start title = 'getSSOUsers Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo "Fetched SSO users:"
  echo ssoUsers
else:
  echo "No SSO users returned, HTTP status: ", httpResponse.statusCode
[inline-code-end]

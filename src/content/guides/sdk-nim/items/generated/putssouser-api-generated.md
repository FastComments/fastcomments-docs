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
let (response, httpResponse) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-9876",
  updateAPISSOUserData = UpdateAPISSOUserData(
    name = "Jane Doe",
    email = "jane.doe@news-site.com",
    externalId = "ext-456",
    roles = @["subscriber", "editor"],
    avatarUrl = "https://news-site.com/avatars/jane.jpg",
    moderator = false
  ),
  updateComments = false
)

if response.isSome:
  let respVal = response.get()
  echo "SSO update succeeded; response object received."
else:
  echo "SSO update returned no body; HTTP status: ", httpResponse.status
[inline-code-end]

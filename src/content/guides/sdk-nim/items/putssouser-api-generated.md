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
  id = "user-789",
  updateAPISSOUserData = UpdateAPISSOUserData(
    email = "jane.doe@newsorg.com",
    displayName = "Jane Doe",
    roles = @["author"],
    avatarUrl = "https://cdn.news.org/avatars/jane.jpg",
    externalId = "jane-789",
    active = true
  ),
  updateComments = true
)

if response.isSome:
  let updated = response.get()
  echo "Updated SSO user:", updated
[inline-code-end]

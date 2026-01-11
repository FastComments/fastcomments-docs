## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | No |  |

## Response

Returns: [`Option[CreateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_user_badge200response.nim)

## Example

[inline-code-attrs-start title = 'createUserBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = CreateUserBadgeParams())
if response.isSome:
  let createdBadge = response.get()
  echo "Created badge:", $createdBadge
else:
  echo "Create badge failed:", $httpResponse
[inline-code-end]

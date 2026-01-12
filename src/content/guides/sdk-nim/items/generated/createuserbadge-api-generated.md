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
let params = CreateUserBadgeParams(
  userId = "user-789",
  badgeId = "top-contributor",
  awardedBy = "moderator@news-site.com",
  reason = "Helpful and respectful contributions",
  hidden = false,
  ttlDays = 0,
  tags = @["community", "trusted"]
)

let (response, httpResponse) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = params)

if response.isSome:
  let badge = response.get()
  discard badge
[inline-code-end]

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetUserBadgesOptions | No |  |

## Response

Returns: [`Option[APIGetUserBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badges_response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadges Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetUserBadgesOptions()
let (badgesOpt, httpResp) = client.getUserBadges(tenantId = "my-tenant-123", options = opts)
if badgesOpt.isSome:
  let badges = badgesOpt.get()
[inline-code-end]

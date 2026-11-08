## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createUserBadgeParams | CreateUserBadgeParams | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APICreateUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_create_user_badge_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'createUserBadge उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeRespOpt, httpResp) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = default(CreateUserBadgeParams))
if badgeRespOpt.isSome:
  let badgeResp = badgeRespOpt.get()
[inline-code-end]
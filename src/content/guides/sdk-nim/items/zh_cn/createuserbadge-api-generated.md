## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | No |  |

## 响应

返回: [`Option[APICreateUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_create_user_badge_response.nim)

## 示例

[inline-code-attrs-start title = 'createUserBadge 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeRespOpt, httpResp) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = default(CreateUserBadgeParams))
if badgeRespOpt.isSome:
  let badgeResp = badgeRespOpt.get()
[inline-code-end]
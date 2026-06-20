## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| pageSize | int | 否 |  |
| afterId | string | 否 |  |
| includeContext | bool | 否 |  |
| afterCreatedAt | int64 | 否 |  |
| unreadOnly | bool | 否 |  |
| dmOnly | bool | 否 |  |
| noDm | bool | 否 |  |
| includeTranslations | bool | 否 |  |
| includeTenantNotifications | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## 示例

[inline-code-attrs-start title = 'getUserNotifications 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  pageSize = 0,
  afterId = "",
  includeContext = false,
  afterCreatedAt = 0,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  includeTenantNotifications = false,
  sso = ""
)

if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---
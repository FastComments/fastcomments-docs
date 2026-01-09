## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| pageSize | int | 否 |  |
| afterId | string | 否 |  |
| includeContext | bool | 否 |  |
| afterCreatedAt | int64 | 否 |  |
| unreadOnly | bool | 否 |  |
| dmOnly | bool | 否 |  |
| noDm | bool | 否 |  |
| includeTranslations | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## 示例

[inline-code-attrs-start title = 'getUserNotifications 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 50,
  afterId = "notif_9a1b2c3d",
  includeContext = true,
  afterCreatedAt = int64(1699999999000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  sso = ""
)
if response.isSome:
  let notifications = response.get()
  discard notifications
else:
  discard httpResponse
[inline-code-end]

---
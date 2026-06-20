## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| pageSize | int | いいえ |  |
| afterId | string | いいえ |  |
| includeContext | bool | いいえ |  |
| afterCreatedAt | int64 | いいえ |  |
| unreadOnly | bool | いいえ |  |
| dmOnly | bool | いいえ |  |
| noDm | bool | いいえ |  |
| includeTranslations | bool | いいえ |  |
| includeTenantNotifications | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
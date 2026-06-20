## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getUserNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "news-tenant-123", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTI0NyIsImlhdCI6MTYw945600fQ.sflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
if response.isSome:
  let countResp = response.get()
  echo "Received user notification count response: ", countResp
else:
  echo "No notification count returned"
[inline-code-end]

---
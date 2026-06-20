---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## 예제

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-sso-token-456")
if response.isSome:
  let result = response.get()
  echo "ResetUserNotificationsResponse:", result
else:
  echo "Reset failed, HTTP response:", httpResponse
[inline-code-end]

---
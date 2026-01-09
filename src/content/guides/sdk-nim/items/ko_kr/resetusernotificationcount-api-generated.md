---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## 예제

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "sso-jwt-9a8b7c6d")
if response.isSome:
  let resetResult = response.get()
  echo resetResult
else:
  echo "Reset failed, status: ", httpResponse.status
[inline-code-end]

---
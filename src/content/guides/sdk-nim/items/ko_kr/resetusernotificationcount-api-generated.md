## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## 응답

반환: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## 예시

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (resetRespOpt, httpResp) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-456")
if resetRespOpt.isSome:
  let resetResp = resetRespOpt.get()
  echo resetResp
else:
  echo "Reset notification count response not available"
[inline-code-end]
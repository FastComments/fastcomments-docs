## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[GetUserNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count200response.nim)

## 예제

[inline-code-attrs-start title = 'getUserNotificationCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if response.isSome:
  let notificationData = response.get()
  echo "Received notification data: ", $notificationData
else:
  echo "No notification data returned. HTTP response: ", $httpResponse.status
[inline-code-end]

---
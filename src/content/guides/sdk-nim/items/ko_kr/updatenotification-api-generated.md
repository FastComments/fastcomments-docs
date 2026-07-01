## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateNotificationBody | UpdateNotificationBody | No |  |
| userId | string = "" | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'updateNotification 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateNotificationBody(message: "Your comment was approved", isRead: false)
let (optResp, httpResp) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-789",
  updateNotificationBody = body,
  userId = "user-42"
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]
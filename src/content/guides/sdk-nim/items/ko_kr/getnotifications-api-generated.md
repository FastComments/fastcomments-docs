## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| urlId | string | 예 |  |
| fromCommentId | string | 아니오 |  |
| viewed | bool | 아니오 |  |
| skip | float64 | 아니오 |  |

## 응답

반환값: [`Option[GetNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications200response.nim)

## 예제

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(
  tenantId = "fastcomments-tenant-42",
  userId = "",
  urlId = "news/latest-tech-innovations",
  fromCommentId = "",
  viewed = false,
  skip = 0.0
)

if response.isSome:
  let notifications = response.get()
  echo "Received notifications: ", notifications
else:
  echo "No notifications, response: ", httpResponse
[inline-code-end]

---
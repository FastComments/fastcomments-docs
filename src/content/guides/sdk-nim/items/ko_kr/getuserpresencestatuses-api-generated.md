## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlIdWS | string | 아니오 |  |
| userIds | string | 아니오 |  |

## 응답

반환: [`Option[GetUserPresenceStatuses_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses200response.nim)

## 예제

[inline-code-attrs-start title = 'getUserPresenceStatuses 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserPresenceStatuses(
  tenantId = "my-tenant-123",
  urlIdWS = "news/2025/technology/ai-ethics",
  userIds = "user-789,user-456"
)
if response.isSome:
  let presence = response.get()
  echo "Presence received: ", presence
else:
  echo "No presence information returned, HTTP status: ", httpResponse.status.code
[inline-code-end]

---
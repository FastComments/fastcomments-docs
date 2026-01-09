## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니요 |  |
| afterCreatedAt | int64 | 아니요 |  |
| unreadOnly | bool | 아니요 |  |
| dmOnly | bool | 아니요 |  |
| noDm | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## 예제

[inline-code-attrs-start title = 'resetUserNotifications 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = int64(0),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---
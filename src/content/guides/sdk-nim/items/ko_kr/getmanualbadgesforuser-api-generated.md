## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgesUserId | string | 아니오 |  |
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## 예제

[inline-code-attrs-start title = 'getManualBadgesForUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getManualBadgesForUser(
  badgesUserId = "user-98765",
  commentId = "comment-0a1b2c3d",
  sso = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let badges = response.get()
  echo "Received manual badges for user"
  echo "HTTP status: ", httpResponse.status
[inline-code-end]

---
---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgesUserId | string | לא |  |
| commentId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getManualBadgesForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
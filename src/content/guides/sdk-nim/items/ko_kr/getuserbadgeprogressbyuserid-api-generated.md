## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-----|
| tenantId | string | Yes |  |
| userId | string | No |  |

## 응답

반환: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## 예시

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if badgeProgressOpt.isSome:
  let progress = badgeProgressOpt.get()
  echo progress
[inline-code-end]

---
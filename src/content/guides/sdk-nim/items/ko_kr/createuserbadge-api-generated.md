## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createUserBadgeParams | CreateUserBadgeParams | 아니요 |  |

## 응답

반환값: [`Option[APICreateUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_create_user_badge_response.nim)

## 예제

[inline-code-attrs-start title = 'createUserBadge 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createUserBadge(
  tenantId = "my-tenant-123",
  createUserBadgeParams = CreateUserBadgeParams(
    userId = "user-456",
    badgeId = "top-commenter",
    reason = "Top commenter for June 2026",
    awardedBy = "mod-team",
    metadata = @["news","engagement"]
  )
)
if response.isSome:
  let badgeResp = response.get()
  discard badgeResp
[inline-code-end]

---
## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## Response

반환: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "badge-456")
if badgeProgressOpt.isSome:
  let badgeProgress = badgeProgressOpt.get()
  echo badgeProgress
[inline-code-end]
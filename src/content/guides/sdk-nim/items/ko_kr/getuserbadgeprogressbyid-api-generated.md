## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[GetUserBadgeProgressById_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_by_id200response.nim)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressById 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "editor-badge-42")
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress received:"
  echo badgeProgress
else:
  echo "No badge progress found for tenant 'my-tenant-123' and id 'editor-badge-42'"
  echo httpResponse
[inline-code-end]

---
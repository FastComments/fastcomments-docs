## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## תגובה

מחזיר: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "badge-456")
if badgeProgressOpt.isSome:
  let badgeProgress = badgeProgressOpt.get()
  echo badgeProgress
[inline-code-end]
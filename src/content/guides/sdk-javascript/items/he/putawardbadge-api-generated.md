## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| badgeId | string | כן |  |
| userId | string | לא |  |
| commentId | string | לא |  |
| broadcastId | string | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`PutAwardBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutAwardBadgeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑putAwardBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const badgeId: string = "badge-superstar"
    const userId: string = "user-42"
    const commentId: string = "comment-7f9c3"
    const broadcastId: string = "broadcast-2023-09"
    const result: PutAwardBadgeResponse = await putAwardBadge(badgeId, userId, commentId, broadcastId)
    console.log(result)
})()
[inline-code-end]
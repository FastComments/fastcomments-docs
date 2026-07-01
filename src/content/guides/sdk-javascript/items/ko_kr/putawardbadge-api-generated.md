## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| badgeId | string | Yes |  |
| userId | string | No |  |
| commentId | string | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`PutAwardBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutAwardBadgeResponse.ts)

## 예시

[inline-code-attrs-start title = 'putAwardBadge 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
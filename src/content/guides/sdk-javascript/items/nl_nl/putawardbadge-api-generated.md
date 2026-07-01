## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| badgeId | string | Yes |  |
| userId | string | No |  |
| commentId | string | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Respons

Retourneert: [`PutAwardBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutAwardBadgeResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'putAwardBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
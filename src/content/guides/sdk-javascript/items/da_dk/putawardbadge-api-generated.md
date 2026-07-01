## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| badgeId | string | Ja |  |
| userId | string | Nej |  |
| commentId | string | Nej |  |
| broadcastId | string | Nej |  |
| tenantId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PutAwardBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutAwardBadgeResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'putAwardBadge Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---
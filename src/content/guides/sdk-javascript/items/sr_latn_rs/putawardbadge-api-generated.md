## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| badgeId | string | Da |  |
| userId | string | Ne |  |
| commentId | string | Ne |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Response

Vraća: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer putAwardBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = 'gold-medal-2023';
const userId: string = 'usr_100234';
const commentId: string = 'c_78910';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';
const response: AwardUserBadgeResponse = await putAwardBadge(badgeId, userId, commentId, undefined, sso);
[inline-code-end]

---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| badgeId | string | Oui |  |
| userId | string | Non |  |
| commentId | string | Non |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de putAwardBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = 'gold-medal-2023';
const userId: string = 'usr_100234';
const commentId: string = 'c_78910';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';
const response: AwardUserBadgeResponse = await putAwardBadge(badgeId, userId, commentId, undefined, sso);
[inline-code-end]

---
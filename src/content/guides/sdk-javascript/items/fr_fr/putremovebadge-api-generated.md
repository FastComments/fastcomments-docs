## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| badgeId | string | Oui |  |
| userId | string | Non |  |
| commentId | string | Non |  |
| broadcastId | string | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`PutRemoveBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutRemoveBadgeResponse.ts)

## Exemple

[inline-code-attrs-start title = 'putRemoveBadge Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = "badge-12345";
const userId: string = "user-9876";
const commentId: string = "comment-5555";
const broadcastId: string = "broadcast-001";

const result: PutRemoveBadgeResponse = await putRemoveBadge(
  badgeId,
  userId,
  commentId,
  broadcastId
);
[inline-code-end]
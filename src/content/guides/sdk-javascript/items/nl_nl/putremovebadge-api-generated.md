## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| badgeId | string | Ja |  |
| userId | string | Nee |  |
| commentId | string | Nee |  |
| broadcastId | string | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retour: [`PutRemoveBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutRemoveBadgeResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'putRemoveBadge voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
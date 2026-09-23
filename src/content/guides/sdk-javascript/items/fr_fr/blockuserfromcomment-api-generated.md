## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| blockFromCommentParams | BlockFromCommentParams | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Renvoie : [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant-123e4567-e89b-12d3-a456-426614174000";
  const commentId: string = "cmt-9876543210";
  const blockParams: BlockFromCommentParams = {
    reason: "spam",
    durationHours: 24
  };
  const userId: string = "user-abc123";
  const anonUserId: string = "anon-xyz789";
  const result: BlockSuccess = await blockUserFromComment(tenantId, commentId, blockParams, userId, anonUserId);
  console.log(result);
}
demo();
[inline-code-end]
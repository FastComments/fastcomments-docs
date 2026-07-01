## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| blockFromCommentParams | BlockFromCommentParams | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Retourne : [`BlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockUserFromCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'blockUserFromComment Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentId: string = 'cmt_20231101';

const blockParams: BlockFromCommentParams = {
  reason: 'spam',
  blockDurationHours: 24,
};

const userId: string = 'user_123'; // paramètre optionnel

const response: BlockUserFromCommentResponse = await blockUserFromComment(
  tenantId,
  commentId,
  blockParams,
  userId
);
[inline-code-end]
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| blockFromCommentParams | BlockFromCommentParams | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`BlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockUserFromCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentId: string = 'cmt_20231101';

const blockParams: BlockFromCommentParams = {
  reason: 'spam',
  blockDurationHours: 24,
};

const userId: string = 'user_123'; // opcionalni parametar

const response: BlockUserFromCommentResponse = await blockUserFromComment(
  tenantId,
  commentId,
  blockParams,
  userId
);
[inline-code-end]
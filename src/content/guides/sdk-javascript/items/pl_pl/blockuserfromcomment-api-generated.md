## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| blockFromCommentParams | BlockFromCommentParams | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`BlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockUserFromCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentId: string = 'cmt_20231101';

const blockParams: BlockFromCommentParams = {
  reason: 'spam',
  blockDurationHours: 24,
};

const userId: string = 'user_123'; // optional parameter

const response: BlockUserFromCommentResponse = await blockUserFromComment(
  tenantId,
  commentId,
  blockParams,
  userId
);
[inline-code-end]

---
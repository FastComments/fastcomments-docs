## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| blockFromCommentParams | BlockFromCommentParams | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'comment_7f3b2a9c';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'Repeated abusive language and targeted harassment',
  durationDays: 90,
  preventReposting: true
};
const userId: string = 'user_12345';
const anonUserId: string = 'anon_98765';

const result: BlockFromCommentPublic200Response = await blockUserFromComment(
  tenantId,
  id,
  blockFromCommentParams,
  userId,
  anonUserId
);
[inline-code-end]

---
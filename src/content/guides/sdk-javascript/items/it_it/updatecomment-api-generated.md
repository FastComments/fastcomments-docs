## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updatableCommentParams | UpdatableCommentParams | Sì |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-prod-01';
const id: string = 'cmt-000127';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Thanks — I updated the steps to include the missing config flag.',
  isHidden: false
};
const contextUserId: string = 'moderator_77';
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: APIEmptyResponse = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---
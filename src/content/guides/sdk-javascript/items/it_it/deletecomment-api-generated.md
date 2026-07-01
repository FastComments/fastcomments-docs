## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| contextUserId | string | No |  |
| isLive | boolean | No |  |

## Risposta

Restituisce: [`DeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_01';
  const commentId: string = 'comment_abc123';
  const contextUserId: string = 'user_42';
  const isLive: boolean = false;

  const deleteResult: DeleteCommentResponse = await deleteComment(tenantId, commentId, contextUserId, isLive);
  const simpleResult: DeleteCommentResponse = await deleteComment(tenantId, commentId);
})();
[inline-code-end]

---
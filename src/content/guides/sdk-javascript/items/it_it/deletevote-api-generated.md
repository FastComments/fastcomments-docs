## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| editKey | string | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8421';
  const id: string = 'vote_3f9b7c2a';
  const editKey: string = 'edit_7Xk9LpQ';
  const responseWithoutEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id);
  const responseWithEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
  console.log(responseWithoutEdit, responseWithEdit);
})();
[inline-code-end]
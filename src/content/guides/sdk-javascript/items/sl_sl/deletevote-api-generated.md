---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| editKey | string | Ne |  |

## Odgovor

Vrne: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---
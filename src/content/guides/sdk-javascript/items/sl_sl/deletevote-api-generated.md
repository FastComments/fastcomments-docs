## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| editKey | string | Ne |  |

## Odgovor

Vrne: [`DeleteVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteVoteResponse.ts)

## Primer

[inline-code-attrs-start title = 'deleteVote Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-9876";
  const editKey: string = "edit-key-123";

  const resultWithEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId, editKey);
  const resultWithoutEdit: DeleteVoteResponse = await deleteVote(tenantId, commentId);
})();
[inline-code-end]

---
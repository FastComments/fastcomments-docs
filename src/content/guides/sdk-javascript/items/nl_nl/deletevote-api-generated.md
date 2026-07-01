## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| editKey | string | Nee |  |

## Respons

Retourneert: [`DeleteVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteVoteResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteVote Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
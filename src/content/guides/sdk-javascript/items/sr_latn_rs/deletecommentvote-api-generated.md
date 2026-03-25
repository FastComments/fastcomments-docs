## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| voteId | string | Da |  |
| urlId | string | Da |  |
| broadcastId | string | Da |  |
| editKey | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Primer

[inline-code-attrs-start title = 'deleteCommentVote Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-87e4fd';
const commentId: string = 'cmt-9a12b3f4';
const voteId: string = 'vote-4f6d21b9';
const urlId: string = 'https://www.acme.com/articles/2026/03/25/how-to-test';
const broadcastId: string = 'broadcast-20260325-01';
const editKey: string = 'editkey-6b7c8d9e';
const sso: string = 'sso-jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const response: DeleteCommentVote200Response = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  sso
);
[inline-code-end]

---
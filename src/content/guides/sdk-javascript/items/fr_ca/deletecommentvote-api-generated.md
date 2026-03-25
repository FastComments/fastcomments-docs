---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| voteId | string | Oui |  |
| urlId | string | Oui |  |
| broadcastId | string | Oui |  |
| editKey | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
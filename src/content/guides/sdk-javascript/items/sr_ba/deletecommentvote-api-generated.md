---
## Parametri

| Ime | Tip | Obavezno | Opis |
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

## Primjer

[inline-code-attrs-start title = 'Primjer deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3a2b7c';
const commentId: string = 'cmt-5a1f3d92';
const voteId: string = 'vote-3b9c7e1a';
const urlId: string = 'articles/2026/06/typescript-best-practices';
const broadcastId: string = 'broadcast-77f4d2';
const editKey: string = 'edk-9b2f4c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature';
const result: DeleteCommentVote200Response = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
[inline-code-end]

---
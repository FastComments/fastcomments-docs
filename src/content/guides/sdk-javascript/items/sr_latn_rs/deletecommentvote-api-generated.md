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

Vraća: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f1b3c";
const commentId: string = "comment_6a7b8c9d";
const voteId: string = "vote_55a1";
const urlId: string = "news/2026/06/19/typescript-updates";
const broadcastId: string = "broadcast_20260619_live_01";
const editKey: string = "edit_3f2a9b";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature";
const result: VoteDeleteResponse = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
[inline-code-end]
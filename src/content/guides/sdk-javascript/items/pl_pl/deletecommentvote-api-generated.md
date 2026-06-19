## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| voteId | string | Tak |  |
| urlId | string | Tak |  |
| broadcastId | string | Tak |  |
| editKey | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---
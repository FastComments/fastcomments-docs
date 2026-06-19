## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| voteId | string | Sim |  |
| urlId | string | Sim |  |
| broadcastId | string | Sim |  |
| editKey | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
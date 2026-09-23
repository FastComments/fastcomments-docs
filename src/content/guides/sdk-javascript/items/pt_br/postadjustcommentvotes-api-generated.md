## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Sim |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo postAdjustCommentVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_987654321";
const adjustParams: AdjustCommentVotesParams = { voteDelta: 1 };
const broadcastId: string = "brd_112233";
const sso: string = "sso-token-xyz";

const result: AdjustVotesResponse = await postAdjustCommentVotes(
  tenantId,
  commentId,
  adjustParams,
  broadcastId,
  sso
);
[inline-code-end]
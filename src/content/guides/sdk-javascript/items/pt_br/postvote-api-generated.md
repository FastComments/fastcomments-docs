## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| direction | string | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Response

Retorna: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Example

[inline-code-attrs-start title = 'postVote Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Chamar apenas com parâmetros obrigatórios
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // Chamar com parâmetros opcionais
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]
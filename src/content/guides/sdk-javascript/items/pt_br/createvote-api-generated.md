## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| direction | CreateVoteDirectionEnum | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b6a';
const commentId: string = 'comment_3b7d2e';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Up;
const anonUserId: string = 'anon_4c2a1f';

const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---
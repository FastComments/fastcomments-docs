## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "northwind-io";
const urlId: string = "blog/2025/06/typescript-updates";
const userId: string = "u_72f9b1c4";
const anonUserId: string = "anon_e4d2a9";

const votesForRegistered: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
const votesForAnonymous: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
[inline-code-end]

---
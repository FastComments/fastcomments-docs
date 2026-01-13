## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const urlId: string = 'news/2026/01/12/product-launch';
const userId: string = 'user_9c3f2b';
const anonUserId: string = 'anon_d4e7a1';

const votesForUser: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId);
const votesForAnon: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
[inline-code-end]

---
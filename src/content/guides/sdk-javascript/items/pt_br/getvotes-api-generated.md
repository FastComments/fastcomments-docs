## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |

## Resposta

Retorna: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Exemplo de acesso a uma propriedade opcional da resposta
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]
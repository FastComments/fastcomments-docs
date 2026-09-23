## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| value | string | Não |  |
| filters | string | Não |  |
| searchFilters | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getSearchCommentsSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearch() {
  const tenantId: string = "tenant_12345";
  const value: string = "spam";
  const filters: string = "status:pending";
  const searchFilters: string = "author:john";
  const sso: string = "sso_token_abc";

  const result: ModerationCommentSearchResponse = await getSearchCommentsSummary(
    tenantId,
    value,
    filters,
    searchFilters,
    sso
  );

  console.log(result);
}

runSearch();
[inline-code-end]
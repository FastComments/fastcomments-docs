## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| afterId | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Resposta

Retorna: [`GetApiIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiIdsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getApiIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const textSearch: string = "urgent feedback";
const byIPFromComment: string = "203.0.113.42";
const filters: string = "status:approved";
const afterId: string = "comment-789";
const demo: boolean = true;
const tenantId: string = "tenant-001";

const apiIds: GetApiIdsResponse = await getApiIds({
  textSearch,
  byIPFromComment,
  filters,
  afterId,
  demo,
  tenantId,
});
[inline-code-end]
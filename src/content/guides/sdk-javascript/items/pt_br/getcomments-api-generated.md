## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| page | number | Não |  |
| limit | number | Não |  |
| skip | number | Não |  |
| asTree | boolean | Não |  |
| skipChildren | number | Não |  |
| limitChildren | number | Não |  |
| maxTreeDepth | number | Não |  |
| urlId | string | Não |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |
| contextUserId | string | Não |  |
| hashTag | string | Não |  |
| parentId | string | Não |  |
| direction | SortDirections | Não |  |
| fromDate | number | Não |  |
| toDate | number | Não |  |

## Resposta

Retorna: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // uma semana atrás
const toDate: number = Date.now();

const commentsResponse: GetCommentsResponse = await getComments({
  tenantId,
  page,
  limit,
  asTree,
  urlId,
  direction,
  fromDate,
  toDate,
});
[inline-code-end]
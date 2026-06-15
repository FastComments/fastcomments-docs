## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
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

Retorna: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---
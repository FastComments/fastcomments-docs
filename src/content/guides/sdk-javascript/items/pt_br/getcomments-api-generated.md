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

Retorna: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_789";
const page: number = 1;
const limit: number = 25;
const asTree: boolean = true;
const maxTreeDepth: number = 3;
const urlId: string = "articles/2026/fastcomments-intro";
const userId: string = "user_12345";
const direction: SortDirections = "desc";
const fromDate: number = 1672531200000;
const toDate: number = Date.now();

const result: APIGetCommentsResponse = await getComments(
  tenantId,
  page,
  limit,
  0,
  asTree,
  0,
  5,
  maxTreeDepth,
  urlId,
  userId,
  undefined,
  undefined,
  "#release",
  undefined,
  direction,
  fromDate,
  toDate
);
[inline-code-end]

---
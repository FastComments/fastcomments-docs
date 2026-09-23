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

Retorna: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "acme-corp";
  const page: number = 1;
  const limit: number = 50;
  const asTree: boolean = false;
  const direction: SortDirections = "asc";
  const fromDate: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 dias atrás
  const toDate: number = Date.now();

  const commentsResponse: APIGetCommentsResponse = await getComments(
    tenantId,
    page,
    limit,
    undefined,
    asTree,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    direction,
    fromDate,
    toDate
  );

  console.log(commentsResponse);
}
[inline-code-end]

---
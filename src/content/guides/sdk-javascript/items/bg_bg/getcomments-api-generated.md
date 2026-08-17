## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Не |  |
| limit | number | Не |  |
| skip | number | Не |  |
| asTree | boolean | Не |  |
| skipChildren | number | Не |  |
| limitChildren | number | Не |  |
| maxTreeDepth | number | Не |  |
| urlId | string | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |
| contextUserId | string | Не |  |
| hashTag | string | Не |  |
| parentId | string | Не |  |
| direction | SortDirections | Не |  |
| fromDate | number | Не |  |
| toDate | number | Не |  |

## Response

Връща: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример за getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // преди една седмица
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
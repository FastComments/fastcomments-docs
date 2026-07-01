## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| page | number | Ні |  |
| limit | number | Ні |  |
| skip | number | Ні |  |
| asTree | boolean | Ні |  |
| skipChildren | number | Ні |  |
| limitChildren | number | Ні |  |
| maxTreeDepth | number | Ні |  |
| urlId | string | Ні |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |
| contextUserId | string | Ні |  |
| hashTag | string | Ні |  |
| parentId | string | Ні |  |
| direction | SortDirections | Ні |  |
| fromDate | number | Ні |  |
| toDate | number | Ні |  |

## Відповідь

Повертає: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // one week ago
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
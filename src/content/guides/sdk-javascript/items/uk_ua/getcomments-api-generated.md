## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
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

Повертає: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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